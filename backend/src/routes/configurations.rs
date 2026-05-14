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

use crate::routes::security::AuthenticatedUser;
use crate::services::account_service;
use crate::DbKelpie;
use rocket::serde::json::Json;
use rocket::{get, post, put, routes};
use rocket_db_pools::Connection;
use shared_core::models::SystemTag;
use shared_core::requests::configuration::UpdateConfigurationRequest;
use std::collections::HashMap;
use uuid::Uuid;

pub fn routes() -> Vec<rocket::Route> {
    routes![get_system_accounts, set_system_accounts, update_configuration]
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

    match  account_service::update_system_accounts(
            &mut db,
            user.organization_id,
            system_accounts.into_inner()
            ).await {
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
    match account_service::update_configuration(&mut db, user.organization_id, req.into_inner()).await {
        Ok(_) => Ok(()),
        Err(_) => Err(rocket::http::Status::InternalServerError),
    }
}
