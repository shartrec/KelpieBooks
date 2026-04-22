/*
 * Copyright (c) 2025. Trevor Campbell and others.
 *
 * This file is part of KelpieRustWeb.
 *
 * KelpieRustWeb is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License,or
 * (at your option) any later version.
 *
 * KelpieRustWeb is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with KelpieRustWeb; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 *
 * Contributors:
 *      Trevor Campbell
 *
 */
use jsonwebtoken::{decode, encode, errors::Error as JwtError, DecodingKey, EncodingKey, Header, TokenData, Validation};
use rocket::serde::Serialize;

use crate::db::security;
use crate::DbKelpie;
use base64::{engine::general_purpose, Engine as _};
use rand::{rngs::OsRng, RngCore};
use rocket::http::{Cookie, CookieJar, Status};
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket_db_pools::Connection;

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

pub(crate) fn routes() -> Vec<Route> {
    routes![login, ]
}

#[post("/api/login", data = "<login_request>")]
async fn login(mut pool: Connection<DbKelpie>, cookies: &CookieJar<'_>, login_request: Json<LoginRequest>) -> Result<Status, Status> {
    let role = security::check_login(&mut *pool, &login_request.username, &login_request.password).await;

    match role {
        Ok(Some(role)) => {
            let user = AuthenticatedUser {
                username: login_request.username.clone(),
                role: role,
            };
            let token = generate_session_token(&user);
            cookies.add(Cookie::build(("session", token))
                .http_only(false)
            );

            Ok(Status::Ok)
        }
        Ok(None) => Err(Status::Unauthorized),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub(super) struct AuthenticatedUser {
    username: String,
    role: Role,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = request.cookies();
        if let Some(cookie) = cookies.get_private("session") {
            if let Some(user) = validate_session_token(cookie.value()) {
                return Outcome::Success(user);
            }
        }
        Outcome::Error((Status::Unauthorized, ()))
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Claims {
    username: String, // Subject (e.g., username)
    role: String, // User role (e.g., admin or tipper)
    exp: usize,   // Expiration time (as a UNIX timestamp)
}

use crate::routes::Role;
use rocket::{post, routes, Route};
use std::sync::OnceLock;

static SECRET_KEY: OnceLock<String> = OnceLock::new();

pub fn init_secret_key() -> String {
    // Generate a secure random 256-bit key and encode as base64
    let mut buf = [0u8; 32];
    OsRng.fill_bytes(&mut buf);
    let key = general_purpose::STANDARD.encode(&buf);
    key
}

fn get_secret_key() -> &'static str {
    SECRET_KEY.get_or_init(|| {
        // Initialize the secret key if it hasn't been set yet
        init_secret_key()
    })
}

fn generate_session_token(user: &AuthenticatedUser) -> String {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("Failed to calculate expiration")
        .timestamp() as usize;

    let claims = Claims {
        username: user.username.clone(),
        role: user.role.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_secret_key().as_ref()),
    )
        .expect("Failed to generate token")
}

pub(super) fn validate_session_token(token: &str) -> Option<AuthenticatedUser> {
    let validation = Validation::default();
    let token_data: Result<TokenData<Claims>, JwtError> = decode(
        token,
        &DecodingKey::from_secret(get_secret_key().as_ref()),
        &validation,
    );
    match token_data {
        Ok(data) => {
            let now = chrono::Utc::now().timestamp() as usize;
            if data.claims.exp > now {
                Some(AuthenticatedUser {
                    username: data.claims.username,
                    role: Role::from(data.claims.role.as_str()).unwrap_or(Role::Guest),
                })
            } else {
                None
            }
        }
        Err(_) => None,
    }
}
