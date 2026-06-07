/*
 * Copyright (c) 2025-2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use crate::util::ApiError;
use bcrypt::{hash, BcryptError, DEFAULT_COST};
use jsonwebtoken::{
    decode, encode, errors::Error as JwtError, DecodingKey, EncodingKey, Header, TokenData,
    Validation,
};
use rocket::{get, post, routes, Route};
use shared_core::dtos::user_detail::AuthUserDetail;
use shared_core::requests::auth::LoginRequest;
use std::sync::OnceLock;

use crate::db::user;
use crate::DbKelpie;
use base64::{engine::general_purpose, Engine as _};
use rand::{rngs::OsRng, RngCore};
use rocket::http::{Cookie, CookieJar, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;
use shared_core::models::role::Role;
use uuid::Uuid;

pub(crate) fn routes() -> Vec<Route> {
    routes![login, me, logout]
}

#[post("/api/login", data = "<login_request>")]
async fn login(
    mut pool: Connection<DbKelpie>,
    cookies: &CookieJar<'_>,
    login_request: Json<LoginRequest>,
) -> Result<Json<AuthUserDetail>, Status> {
    let db_user = user::get_by_email(&mut pool, &login_request.email).await;

    match db_user {
        Ok(Some(user)) => {
            let valid =
                bcrypt::verify(&login_request.password_raw, &user.password_hash).unwrap_or(false);
            if !valid {
                return Err(Status::Unauthorized);
            }

            let user_locale = login_request
                .locale
                .clone()
                .unwrap_or_else(|| "en-GB".to_string());

            let auth_user = AuthenticatedUser {
                user_id: user.id,
                organization_id: user.organization_id,
                strict_audit_mode: user.strict_audit_mode,
                username: user.email.clone(),
                full_name: user.full_name.clone(),
                display_name: user.display_name.clone(),
                role: user.role,
                organisation_name: user.organisation_name.clone(),
                locale: user_locale,
            };
            let token = generate_session_token(&auth_user);
            cookies.add(Cookie::build(("session", token)).http_only(false));

            let role  = auth_user.role.as_ref().map(|r| r.name.clone());
            // 💡 Map the SystemPrivilege enum variants directly into string flags
            let privileges = auth_user.role
                .map(|r| r.privileges.iter().map(|p| format!("{:?}", p)).collect())
                .unwrap_or_else(Vec::new);

            let user_detail = AuthUserDetail {
                id: user.id,
                email: user.email,
                full_name: user.full_name,
                display_name: user.display_name,
                role: role,
                organization_id: user.organization_id,
                privileges: privileges
            };

            Ok(Json(user_detail))
        }
        Ok(None) => Err(Status::Unauthorized),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/api/auth/me")]
async fn me(user: AuthenticatedUser) -> Json<AuthUserDetail> {

    let role  = user.role.as_ref().map(|r| r.name.clone());
    // 💡 Map the SystemPrivilege enum variants directly into string flags
    let privileges = user.role
        .map(|r| r.privileges.iter().map(|p| format!("{:?}", p)).collect())
        .unwrap_or_else(Vec::new);

    Json(AuthUserDetail {
        id: user.user_id,
        email: user.username,
        full_name: user.full_name,
        display_name: user.display_name,
        role: role,
        organization_id: user.organization_id,
        privileges: privileges
    })
}

#[post("/api/auth/logout")]
fn logout(cookies: &CookieJar<'_>) -> Status {
    cookies.remove(Cookie::from("session"));
    Status::Ok
}

pub(crate) struct AuthenticatedUser {
    pub(crate) user_id: Uuid,
    pub(crate) organization_id: Uuid,
    pub(crate) strict_audit_mode: bool,
    pub(crate) username: String,
    pub(crate) full_name: String,
    pub(crate) display_name: Option<String>,
    pub(crate) role: Option<Role>,
    pub(crate) organisation_name: String,
    pub(crate) locale: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(cookie) = request.cookies().get("session") {
            let outcome = request.guard::<Connection<DbKelpie>>().await;
            if let Outcome::Success(_) = outcome {
                if let Some(user) = validate_session_token(cookie.value()).await {
                    return Outcome::Success(user);
                }
            }
        }
        Outcome::Error((Status::Unauthorized, ()))
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Claims {
    user_id: String,
    organization_id: String,
    strict_audit_mode: bool,
    username: String,
    full_name: String,
    display_name: Option<String>,
    role_id: String,
    privileges: Vec<String>,
    exp: usize,
    organisation_name: String,
    locale: Option<String>
}

pub(crate) fn hash_pwd(password: &str) -> Result<String, ApiError> {
    hash(password, DEFAULT_COST).map_err(|e| ApiError::from(BcryptError::from(e)))
}

static SECRET_KEY: OnceLock<String> = OnceLock::new();

pub(crate) fn init_secret_key() -> String {
    let mut buf = [0u8; 32];
    OsRng.fill_bytes(&mut buf);
    general_purpose::STANDARD.encode(&buf)
}

fn get_secret_key() -> &'static str {
    SECRET_KEY.get_or_init(init_secret_key)
}

fn generate_session_token(user: &AuthenticatedUser) -> String {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("Failed to calculate expiration")
        .timestamp() as usize;

    let role = user.role.as_ref().map(|r| r.id.to_string());

    let privileges = user.role.as_ref()
        .map(|r| r.privileges.iter().map(|p| p.as_str().to_string()).collect())
        .unwrap_or_else(Vec::new);

    let claims = Claims {
        user_id: user.user_id.to_string(),
        organization_id: user.organization_id.to_string(),
        strict_audit_mode: user.strict_audit_mode,
        username: user.username.clone(),
        full_name: user.full_name.clone(),
        display_name: user.display_name.clone(),
        role_id: role.unwrap_or("".to_string()),
        privileges,
        exp: expiration,
        organisation_name: user.organisation_name.clone(),
        locale: Some(user.locale.clone()),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_secret_key().as_ref()),
    )
    .expect("Failed to generate token")
}

pub(crate) async fn validate_session_token(token: &str) -> Option<AuthenticatedUser> {
    let validation = Validation::default();
    let token_data: Result<TokenData<Claims>, JwtError> = decode(
        token,
        &DecodingKey::from_secret(get_secret_key().as_ref()),
        &validation,
    );

    if let Ok(data) = token_data {
        if data.claims.exp > chrono::Utc::now().timestamp() as usize {
            let org_id = Uuid::parse_str(&data.claims.organization_id).unwrap();
            let role_id = Uuid::parse_str(&data.claims.role_id).unwrap_or_default();

            // 💡 Safely parse strings back to your SystemPrivilege enum variants
            use std::str::FromStr;
            let privileges: Vec<shared_core::models::auth::SystemPrivilege> = data.claims.privileges
                .iter()
                .filter_map(|p_str| shared_core::models::auth::SystemPrivilege::from_str(p_str).ok())
                .collect();

            return Some(AuthenticatedUser {
                user_id: Uuid::parse_str(&data.claims.user_id).unwrap(),
                organization_id: org_id,
                strict_audit_mode: data.claims.strict_audit_mode,
                username: data.claims.username,
                full_name: data.claims.full_name,
                display_name: data.claims.display_name,
                role: Some(Role {
                    id: role_id,
                    name: "".to_string(), // You can also add role_name to claims if needed
                    privileges,
                }),
                organisation_name: data.claims.organisation_name,
                locale: data.claims.locale.unwrap_or_else(|| "en-GB".to_string()),
            });
        }
    }
    None
}
