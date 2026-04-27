/*
 * Copyright (c) 2026-2026. Trevor Campbell and others.
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

pub mod logging;
pub mod types;

use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{Request, Response};
use rocket_db_pools::sqlx;
use bcrypt;

#[derive(Debug)]
pub(crate) enum ApiError {
    Db(sqlx::Error),
    Conflict(String),
    Hashing(bcrypt::BcryptError),
    Error(String),
    Invalid(String),
    NotFound(String),
    Internal(String),
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
        let (status, msg) = match self {
            ApiError::NotFound(msg) => (Status::NotFound, msg),
            ApiError::Error(msg) => (Status::InternalServerError, msg),
            ApiError::Invalid(msg) => (Status::BadRequest, msg),
            ApiError::Internal(msg) => (Status::InternalServerError, msg),
            ApiError::Conflict(e) => (Status::Conflict, e.to_string()),
            ApiError::Db(e) => (Status::InternalServerError, e.to_string()),
            ApiError::Hashing(e) => (Status::InternalServerError, format!("Password hashing error: {}", e)),
        };
        let body = Json(ApiErrorMessage { error: msg });
        Response::build()
            .status(status)
            .header(rocket::http::ContentType::JSON)
            .sized_body(body.0.error.len(), std::io::Cursor::new(body.0.error.to_string()))
            .ok()
    }
}
