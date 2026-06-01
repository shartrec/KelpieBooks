/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use rocket::{get, routes, Route};
use crate::DbKelpie;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use shared_core::models::auth::SystemPrivilege;
use crate::routes::security::AuthenticatedUser;

pub(crate) fn routes() -> Vec<Route> {
    routes![
        get_privileges,
    ]
}

#[get("/privileges")]
pub async fn get_privileges(
    mut pool: Connection<DbKelpie>,
    user: AuthenticatedUser,
) -> Result<Json<Vec<SystemPrivilege>>, Status> {
    Ok(Json(SystemPrivilege::iterator().collect()))
}
