pub mod logging;

use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{Request, Response};
use rocket_db_pools::sqlx;

#[derive(Debug)]
pub(crate) enum ApiError {
    Db(sqlx::Error),
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
            ApiError::Db(e) => (Status::Conflict, e.to_string()),
        };
        let body = Json(ApiErrorMessage { error: msg });
        Response::build()
            .status(status)
            .header(rocket::http::ContentType::JSON)
            .sized_body(body.0.error.len(), std::io::Cursor::new(body.0.error.to_string()))
            .ok()
    }
}
