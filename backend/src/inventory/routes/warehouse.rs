/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 * (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use rocket::{
    delete,
    get,
    post,
    put,
    routes,
    serde::json::Json,
    Route,
};
use rocket_db_pools::Connection;
use shared_core::inventory::models::warehouse::Warehouse;

use crate::{
    core::routes::security::AuthenticatedUser,
    inventory::services::warehouse as warehouse_service,
    security::{
        ManageInventory,
        RequirePrivilege,
        UseInventory,
    },
    util::{
        types::PathUuid,
        ApiError,
    },
    DbKelpie,
};

pub(crate) fn routes() -> Vec<Route> {
    routes![
        get_warehouses,
        get_warehouse,
        create_warehouse,
        update_warehouse,
        delete_warehouse,
    ]
}

// =============================================================================
// Warehouse Route Handlers
// =============================================================================

#[get("/api/inventory/warehouses")]
async fn get_warehouses(
    mut pool: Connection<DbKelpie>,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<UseInventory>,
) -> Result<Json<Vec<Warehouse>>, ApiError> {
    let warehouses = warehouse_service::get_warehouses(&mut pool, user.organization_id).await?;
    Ok(Json(warehouses))
}

#[get("/api/inventory/warehouses/<id>")]
async fn get_warehouse(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<UseInventory>,
) -> Result<Json<Warehouse>, ApiError> {
    let warehouse = warehouse_service::get_warehouse(&mut pool, *id, user.organization_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Warehouse not found".to_string()))?;
    Ok(Json(warehouse))
}

#[post("/api/inventory/warehouses", data = "<wh>")]
async fn create_warehouse(
    mut pool: Connection<DbKelpie>,
    wh: Json<Warehouse>,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<ManageInventory>,
) -> Result<Json<Warehouse>, ApiError> {
    let new_wh = warehouse_service::create_warehouse(&mut pool, user.organization_id, &wh).await?;
    Ok(Json(new_wh))
}

#[put("/api/inventory/warehouses/<id>", data = "<wh>")]
async fn update_warehouse(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
    wh: Json<Warehouse>,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<ManageInventory>,
) -> Result<Json<Warehouse>, ApiError> {
    let updated_wh =
        warehouse_service::update_warehouse(&mut pool, *id, user.organization_id, &wh).await?;
    Ok(Json(updated_wh))
}

#[delete("/api/inventory/warehouses/<id>")]
async fn delete_warehouse(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<ManageInventory>,
) -> Result<&'static str, ApiError> {
    let rows_affected =
        warehouse_service::delete_warehouse(&mut pool, *id, user.organization_id).await?;
    if rows_affected == 0 {
        return Err(ApiError::NotFound("Warehouse not found.".to_string()));
    }
    Ok("Warehouse deleted successfully.")
}
