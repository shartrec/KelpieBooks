/*
 * Copyright (c) 2025-2026. Trevor Campbell and others.
 *
 * This file is part of KelpieBooks.
 *
 * KelpieBooks is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License,or
 * (at your option) any later version.
 *
 * KelpieBooks is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with KelpieBooks; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 *
 * Contributors:
 *      Trevor Campbell
 *
 */
use crate::routes::Role;
use crate::util::ApiError;
use bcrypt::{hash, BcryptError, DEFAULT_COST};
use jsonwebtoken::{
    decode, encode, errors::Error as JwtError, DecodingKey, EncodingKey, Header, TokenData,
    Validation,
};
use rocket::{get, post, routes, Route};
use shared_core::dtos::user_detail::UserDetail;
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
use uuid::Uuid;

pub(crate) fn routes() -> Vec<Route> {
    routes![login, me, logout]
}

#[post("/api/login", data = "<login_request>")]
async fn login(
    mut pool: Connection<DbKelpie>,
    cookies: &CookieJar<'_>,
    login_request: Json<LoginRequest>,
) -> Result<Json<UserDetail>, Status> {
    let db_user =
        user::get_by_email(&mut pool, &login_request.email).await;

    match db_user {
        Ok(Some(user)) => {
            let valid = bcrypt::verify(&login_request.password_raw, &user.password_hash).unwrap_or(false);
            if !valid {
                return Err(Status::Unauthorized);
            }

            let auth_user = AuthenticatedUser {
                user_id: user.id,
                organization_id: user.organization_id,
                strict_audit_mode: false, // TODO: Get this from the organization
                username: user.email.clone(),
                full_name: user.full_name.clone(),
                display_name: user.display_name.clone(),
                role: Role::User,
                organisation_name: user.organisation_name.clone(),
            };
            let token = generate_session_token(&auth_user);
            cookies.add(Cookie::build(("session", token)).http_only(false));

            let user_detail = UserDetail {
                id: user.id,
                email: user.email,
                full_name: user.full_name,
                display_name: user.display_name,
                role: auth_user.role.to_string(),
                organisation_name: user.organisation_name,
            };

            Ok(Json(user_detail))
        }
        Ok(None) => Err(Status::Unauthorized),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/api/auth/me")]
async fn me(user: AuthenticatedUser) -> Json<UserDetail> {
    Json(UserDetail {
        id: user.user_id,
        email: user.username,
        full_name: user.full_name,
        display_name: user.display_name,
        role: user.role.to_string(),
        organisation_name: user.organisation_name,
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
    pub(crate) role: Role,
    pub(crate) organisation_name: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(cookie) = request.cookies().get("session") {
            if let Some(user) = validate_session_token(cookie.value()) {
                return Outcome::Success(user);
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
    role: String,
    exp: usize,
    organisation_name: String,
}

pub(crate) fn hash_pwd(password: &str) -> Result<String, ApiError> {
    hash(password, DEFAULT_COST).map_err(|e| ApiError::from(BcryptError::from(e)))
}

static SECRET_KEY: OnceLock<String> = OnceLock::new();

pub fn init_secret_key() -> String {
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

    let claims = Claims {
        user_id: user.user_id.to_string(),
        organization_id: user.organization_id.to_string(),
        strict_audit_mode: user.strict_audit_mode,
        username: user.username.clone(),
        full_name: user.full_name.clone(),
        display_name: user.display_name.clone(),
        role: user.role.to_string(),
        exp: expiration,
        organisation_name: user.organisation_name.clone(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_secret_key().as_ref()),
    )
    .expect("Failed to generate token")
}

pub(crate) fn validate_session_token(token: &str) -> Option<AuthenticatedUser> {
    let validation = Validation::default();
    let token_data: Result<TokenData<Claims>, JwtError> = decode(
        token,
        &DecodingKey::from_secret(get_secret_key().as_ref()),
        &validation,
    );
    match token_data {
        Ok(data) => {
            if data.claims.exp > chrono::Utc::now().timestamp() as usize {
                Some(AuthenticatedUser {
                    user_id: Uuid::parse_str(&data.claims.user_id).unwrap(),
                    organization_id: Uuid::parse_str(&data.claims.organization_id).unwrap(),
                    strict_audit_mode: data.claims.strict_audit_mode,
                    username: data.claims.username,
                    full_name: data.claims.full_name,
                    display_name: data.claims.display_name,
                    role: Role::from(&data.claims.role).unwrap_or(Role::Guest),
                    organisation_name: data.claims.organisation_name,
                })
            } else {
                None
            }
        }
        Err(_) => None,
    }
}
