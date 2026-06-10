/*
 * Copyright (c) 2025-2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use std::sync::OnceLock;

use base64::{
    engine::general_purpose,
    Engine as _,
};
use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use chrono::{Utc, Duration};
use jsonwebtoken::{
    decode,
    encode,
    errors::Error as JwtError,
    DecodingKey,
    EncodingKey,
    Header,
    TokenData,
    Validation,
};
use log::{error, log};
use rand::{
    rngs::OsRng,
    RngCore, thread_rng,
};
use rocket::{
    get,
    http::{
        Cookie,
        CookieJar,
        Status,
    },
    post,
    request::{
        FromRequest,
        Outcome,
        Request,
    },
    routes,
    serde::{
        json::Json,
        Deserialize,
        Serialize,
    },
    Route,
};
use rocket_db_pools::Connection;
use shared_core::core::requests::auth::{ForgotPasswordRequest, LoginRequest, ResetPasswordSubmit};
use uuid::Uuid;
use shared_core::core::dtos::user_detail::AuthUserDetail;
use shared_core::core::models::role::Role;
use crate::{
    util::ApiError,
    DbKelpie,
};
use crate::config::load_config;
use crate::core::db::user;
#[cfg(feature = "password-reset")]
use crate::core::db::password_reset;
#[cfg(feature = "email")]
use crate::core::services::email_service;

pub(crate) fn routes() -> Vec<Route> {
    let mut routes = routes![login, me, logout];
    #[cfg(feature = "password-reset")]
    {
        let mut pwd_routes = routes![forgot_password, reset_password];
        routes.append(&mut pwd_routes);
    }
    routes
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

            let role = auth_user.role.as_ref().map(|r| r.name.clone());
            // 💡 Map the SystemPrivilege enum variants directly into string flags
            let privileges = auth_user
                .role
                .map(|r| r.privileges.iter().map(|p| format!("{:?}", p)).collect())
                .unwrap_or_else(Vec::new);

            let user_detail = AuthUserDetail {
                id: user.id,
                email: user.email,
                full_name: user.full_name,
                display_name: user.display_name,
                role: role,
                organization_id: user.organization_id,
                privileges: privileges,
            };

            Ok(Json(user_detail))
        }
        Ok(None) => Err(Status::Unauthorized),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/api/auth/me")]
async fn me(user: AuthenticatedUser) -> Json<AuthUserDetail> {
    let role = user.role.as_ref().map(|r| r.name.clone());
    // 💡 Map the SystemPrivilege enum variants directly into string flags
    let privileges = user
        .role
        .map(|r| r.privileges.iter().map(|p| format!("{:?}", p)).collect())
        .unwrap_or_else(Vec::new);

    Json(AuthUserDetail {
        id: user.user_id,
        email: user.username,
        full_name: user.full_name,
        display_name: user.display_name,
        role: role,
        organization_id: user.organization_id,
        privileges: privileges,
    })
}

#[post("/api/auth/logout")]
fn logout(cookies: &CookieJar<'_>) -> Status {
    cookies.remove(Cookie::from("session"));
    Status::Ok
}

#[cfg(feature = "email")]
#[post("/api/auth/forgot-password", data = "<payload>")]
async fn forgot_password(mut conn: Connection<DbKelpie>, payload: Json<ForgotPasswordRequest>) -> Status {
    let email = &payload.email;

    // 1. Check if the user exists
    if let Ok(Some(user)) = user::get_by_email(&mut conn, email).await {
        // 2. Generate a secure 32-byte random token string
        let mut token_bytes = [0u8; 32];
        thread_rng().fill_bytes(&mut token_bytes);
        let raw_token = hex::encode(token_bytes);

        // 3. Hash the token for database storage
        let token_hash = hash(&raw_token, 10).unwrap();
        let expires_at = Utc::now() + Duration::minutes(20);

        // 4. Save to password_reset_tokens table
        if let Ok(id) = password_reset::save_reset_token(&mut conn, user.id, &token_hash, expires_at).await {

            let config = load_config();

            // 5. Send the email (ideally queued via a background worker thread)
            match email_service::send_reset_email(&user.email, id, &raw_token, &config.app.default_locale).await {
                Ok(_) => {}
                Err(e) => {
                    error!("Email not sent, Error {}", e);
                }
            }
        }
    }

    // 💡 LAZY PURGE: Clean up old garbage rows asynchronously before dealing with the new one
    let _ = password_reset::delete_expired_reset_tokens(&mut conn).await;

    // 💡 CRITICAL: Always return 200 OK / Accepted, even if the user didn't exist!
    Status::Accepted
}

#[cfg(feature = "password-reset")]
#[post("/api/auth/reset-password", data = "<payload>")]
async fn reset_password(mut db: Connection<DbKelpie>, payload: Json<ResetPasswordSubmit>) -> Result<Status, ApiError> {
    let raw_token: String = payload.raw_token.clone().to_string();

    // 1. Locate token records that match, aren't expired, and haven't been used yet
    let token_record = password_reset::find_active_token(&mut db, &payload.id).await?
        .ok_or(ApiError::NotFound("Invalid or expired token".to_string()))?;

    if token_record.expires_at < Utc::now() {
        return Err(ApiError::BadRequest("Token has expired".to_string()));
    }
    if let Ok(b) = verify(raw_token, token_record.token_hash.as_str()) {
        if b {
            let hashed_password = hash_pwd(&payload.new_password)?;

            // 3. Update the user record and atomically flag the token as used
            user::update_password(&mut db, token_record.user_id, &hashed_password).await?;
            password_reset::mark_token_as_used(&mut db, payload.id).await?;

            Ok(Status::Ok)
        } else {
            Err(ApiError::BadRequest("Invalid token".to_string()))
        }
    } else {
        return Err(ApiError::BadRequest("Invalid token".to_string()));
    }

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
    locale: Option<String>,
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

    let privileges = user
        .role
        .as_ref()
        .map(|r| {
            r.privileges
                .iter()
                .map(|p| p.as_str().to_string())
                .collect()
        })
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
            let privileges: Vec<shared_core::core::models::auth::SystemPrivilege> = data
                .claims
                .privileges
                .iter()
                .filter_map(|p_str| {
                    shared_core::core::models::auth::SystemPrivilege::from_str(p_str).ok()
                })
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
