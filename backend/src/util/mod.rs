/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

pub mod logging;
pub mod types;
pub mod locale_context;

use bcrypt;
use log::error;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::{Request, Response};
use rocket_db_pools::sqlx;
use shared_core::dtos::ApiErrorMessage;

#[derive(Debug)]
pub(crate) enum ApiError {
    Db(sqlx::Error),
    Conflict(String),
    Hashing(bcrypt::BcryptError),
    Invalid(String),
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
            ApiError::Invalid(msg) => (Status::BadRequest, msg),
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
