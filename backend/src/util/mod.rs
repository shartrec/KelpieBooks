/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

pub(crate) mod locale_context;
pub(crate) mod logging;
pub(crate) mod reports;
pub(crate) mod types;

use std::env;
use std::path::PathBuf;
use bcrypt;
use log::error;
use rocket::{http::Status, response::Responder, serde::json::Json, Build, Request, Response, Rocket};
use rocket_db_pools::sqlx;
use shared_core::core::dtos::ApiErrorMessage;

#[derive(Debug)]
pub(crate) enum ApiError {
    Db(sqlx::Error),
    Conflict(String),
    Hashing(bcrypt::BcryptError),
    BadRequest(String),
    NotFound(String),
    Internal(String),
    Forbidden(String),
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        ApiError::Db(err)
    }
}

impl From<bcrypt::BcryptError> for ApiError {
    fn from(err: bcrypt::BcryptError) -> Self {
        ApiError::Hashing(err)
    }
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        // 1. Log full debug attributes securely on the server console
        error!("API Error: {:?}", self);

        // 2. Resolve internal variants to an HTTP code status and structural string
        let (status, msg) = match self {
            ApiError::NotFound(msg) => (Status::NotFound, msg),
            ApiError::Forbidden(msg) => (Status::Forbidden, msg),
            ApiError::BadRequest(msg) => (Status::BadRequest, msg),
            ApiError::Internal(msg) => (Status::InternalServerError, msg),
            ApiError::Conflict(e) => (Status::Conflict, e.to_string()),
            ApiError::Db(e) => (Status::InternalServerError, e.to_string()),
            ApiError::Hashing(e) => (
                Status::InternalServerError,
                format!("Password hashing error: {}", e),
            ),
        };

        // 3. Create our actual structural error body
        let error_body = ApiErrorMessage { error: msg };

        // 4. Delegate to Rocket's native JSON responder wrapper!
        // This automatically serializes the struct keys to a valid json string `{"error":"..."}`
        // and safely configures the sizing and ContentType::JSON headers under the hood.
        Response::build_from(Json(error_body).respond_to(req)?)
            .status(status)
            .ok()
    }
}

pub fn get_static_dir(rocket: &Rocket<Build>) -> PathBuf {
    let profile = rocket.figment().profile().as_ref();

    match profile {
        "debug" => {
            // Development Mode: Point directly to the source tree folder
            // This allows local asset changes to show up immediately
            println!("🚀 Running in DEVELOPMENT mode. Using source assets tree.");
            rocket::fs::relative!("static").into()
        }
        _ => {
            // Production / Release Mode: Look next to the running executable binary
            println!("⚙️ Running in PRODUCTION mode. Using neighboring standalone assets folder.");
            let current_dir = env::current_dir().expect("Failed to read runtime directory");
            current_dir.join("static")
        }
    }
}
pub fn get_template_dir(rocket: &Rocket<Build>) -> PathBuf {
    let profile = rocket.figment().profile().as_ref();

    match profile {
        "debug" => {
            // Development Mode: Point directly to the source tree folder
            // This allows local asset changes to show up immediately
            println!("🚀 Running in DEVELOPMENT mode. Using source assets tree.");
            rocket::fs::relative!("templates").into()
        }
        _ => {
            // Production / Release Mode: Look next to the running executable binary
            println!("⚙️ Running in PRODUCTION mode. Using neighboring standalone assets folder.");
            let current_dir = env::current_dir().expect("Failed to read runtime directory");
            current_dir.join("templates")
        }
    }
}