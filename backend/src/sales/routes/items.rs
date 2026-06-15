/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use rocket::{delete, get, post, put, routes, Route};
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use shared_core::sales::models::item::Item;
use uuid::Uuid;
use crate::core::routes::security::AuthenticatedUser;
use crate::DbKelpie;
use crate::sales::services::item_service;
use crate::security::{ManageSales, RequirePrivilege, UseSales};
use crate::util::ApiError;
use crate::util::types::PathUuid;
use shared_core::sales::requests::item::CreateItemRequest;

pub(crate) fn routes() -> Vec<Route> {
    routes![
        get_items,
        get_item,
        create_item,
        update_item,
        delete_item,
    ]
}

#[get("/api/sales/items")]
async fn get_items(
    mut pool: Connection<DbKelpie>,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<UseSales>,
) -> Result<Json<Vec<Item>>, ApiError> {
    let items = item_service::get_items(&mut pool, user.organization_id).await?;
    Ok(Json(items))
}

#[get("/api/sales/items/<id>")]
async fn get_item(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<UseSales>,
) -> Result<Json<Option<Item>>, ApiError> {
    let item = item_service::get_item(&mut pool, *id, user.organization_id).await?;
    Ok(Json(item))
}

#[post("/api/sales/items", data = "<item>")]
async fn create_item(
    mut pool: Connection<DbKelpie>,
    item: Json<CreateItemRequest>,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<ManageSales>,
) -> Result<Json<Item>, ApiError> {
    let new_item = item_service::create_item(&mut pool, user.organization_id, &item).await?;
    Ok(Json(new_item))
}

#[put("/api/sales/items/<id>", data = "<item>")]
async fn update_item(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
    item: Json<Item>,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<ManageSales>,
) -> Result<Json<Item>, ApiError> {
    let updated_item = item_service::update_item(&mut pool, *id, user.organization_id, &item).await?;
    Ok(Json(updated_item))
}

#[delete("/api/sales/items/<id>")]
async fn delete_item(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<ManageSales>,
) -> Result<Json<u64>, ApiError> {
    let rows_affected = item_service::delete_item(&mut pool, *id, user.organization_id).await?;
    Ok(Json(rows_affected))
}
