/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use rocket::serde::json::Json;
use rocket::{get, routes, Route};
use shared_core::models::auth::SystemPrivilege;
use crate::routes::security::AuthenticatedUser;

pub(crate) fn routes() -> Vec<Route> {
    routes![get_privileges]
}

#[get("/privileges")]
pub async fn get_privileges(
    _user: AuthenticatedUser,
) -> Json<Vec<SystemPrivilege>> {
    Json(SystemPrivilege::iterator().collect())
}
