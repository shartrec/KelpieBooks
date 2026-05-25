/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

pub mod logging;
pub mod types;

use bcrypt;
use log::error;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{Request, Response};
use rocket_db_pools::sqlx;

#[derive(Debug)]
pub(crate) enum ApiError {
    Db(sqlx::Error),
    Conflict(String),
    Hashing(bcrypt::BcryptError),
    Error(String),
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

#[derive(Debug, Serialize)]
pub(crate) struct ApiErrorMessage {
    pub(crate) error: String,
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        // Log the full error details before creating the response
        error!("API Error: {:?}", self);

        let (status, msg) = match self {
            ApiError::NotFound(msg) => (Status::NotFound, msg),
            ApiError::Error(msg) => (Status::InternalServerError, msg),
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
        let body = Json(ApiErrorMessage { error: msg });
        Response::build()
            .status(status)
            .header(rocket::http::ContentType::JSON)
            .sized_body(
                body.0.error.len(),
                std::io::Cursor::new(body.0.error.to_string()),
            )
            .ok()
    }
}
