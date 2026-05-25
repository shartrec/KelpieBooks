/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::routes::security::AuthenticatedUser;
use crate::services::account_service;
use crate::DbKelpie;
use rocket::serde::json::Json;
use rocket::{get, post, put, routes};
use rocket_db_pools::Connection;
use shared_core::models::system_tag::SystemTag;
use shared_core::requests::configuration::UpdateConfigurationRequest;
use std::collections::HashMap;
use uuid::Uuid;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        get_system_accounts,
        set_system_accounts,
        update_configuration
    ]
}

#[get("/api/configurations/system-accounts")]
async fn get_system_accounts(
    mut db: Connection<DbKelpie>,
    user: AuthenticatedUser,
) -> Result<Json<HashMap<SystemTag, Uuid>>, rocket::http::Status> {
    match account_service::get_system_accounts(&mut db, user.organization_id).await {
        Ok(accounts) => Ok(Json(accounts)),
        Err(_) => Err(rocket::http::Status::InternalServerError),
    }
}

#[post("/api/configurations/system-accounts", data = "<system_accounts>")]
async fn set_system_accounts(
    mut db: Connection<DbKelpie>,
    user: AuthenticatedUser,
    system_accounts: Json<HashMap<SystemTag, Uuid>>,
) -> Result<Json<HashMap<SystemTag, Uuid>>, rocket::http::Status> {
    match account_service::update_system_accounts(
        &mut db,
        user.organization_id,
        &system_accounts.into_inner(),
    )
    .await
    {
        Ok(accounts) => Ok(Json(accounts)),
        Err(_) => Err(rocket::http::Status::InternalServerError),
    }
}

#[put("/api/configurations", data = "<req>")]
async fn update_configuration(
    mut db: Connection<DbKelpie>,
    user: AuthenticatedUser,
    req: Json<UpdateConfigurationRequest>,
) -> Result<(), rocket::http::Status> {
    match account_service::update_configuration(&mut db, user.organization_id, &req.into_inner())
        .await
    {
        Ok(_) => Ok(()),
        Err(_) => Err(rocket::http::Status::InternalServerError),
    }
}
