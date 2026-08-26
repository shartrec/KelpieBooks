/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use rocket::{
    get,
    post,
    put,
    routes,
    serde::json::Json,
    Route,
};
use rocket_db_pools::Connection;
use shared_core::inventory::{
    dtos::inventory::BulkLocationGenerateRequest,
    models::warehouse::WarehouseLocation,
};

use crate::{
    core::routes::security::AuthenticatedUser,
    inventory::services::locations as location_service,
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
        get_locations,
        get_location,
        generate_locations,
        create_location,
        update_location,
    ]
}

#[get("/api/inventory/warehouses/<warehouse_id>/locations")]
async fn get_locations(
    mut pool: Connection<DbKelpie>,
    warehouse_id: PathUuid,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<UseInventory>,
) -> Result<Json<Vec<WarehouseLocation>>, ApiError> {
    let locations = location_service::get_locations_for_warehouse(
        &mut pool,
        user.organization_id,
        *warehouse_id,
    )
    .await?;
    Ok(Json(locations))
}

#[post(
    "/api/inventory/warehouses/<warehouse_id>/locations/generate",
    data = "<req>"
)]
async fn generate_locations(
    mut pool: Connection<DbKelpie>,
    warehouse_id: PathUuid,
    req: Json<BulkLocationGenerateRequest>,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<ManageInventory>,
) -> Result<Json<Vec<WarehouseLocation>>, ApiError> {
    let new_locations =
        location_service::generate_locations(&mut pool, user.organization_id, *warehouse_id, &req)
            .await?;
    Ok(Json(new_locations))
}
#[get("/api/inventory/locations/<id>")]
async fn get_location(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<UseInventory>,
) -> Result<Json<WarehouseLocation>, ApiError> {
    let location =
        crate::inventory::services::locations::get_location(&mut pool, user.organization_id, *id)
            .await?
            .ok_or_else(|| ApiError::NotFound("Warehouse location not found".to_string()))?;
    Ok(Json(location))
}

#[post("/api/inventory/locations", data = "<loc>")]
async fn create_location(
    mut pool: Connection<DbKelpie>,
    loc: Json<WarehouseLocation>,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<ManageInventory>,
) -> Result<Json<WarehouseLocation>, ApiError> {
    let new_loc = crate::inventory::services::locations::create_location(
        &mut pool,
        user.organization_id,
        &loc,
    )
    .await?;
    Ok(Json(new_loc))
}

#[put("/api/inventory/locations/<id>", data = "<loc>")]
async fn update_location(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
    loc: Json<WarehouseLocation>,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<ManageInventory>,
) -> Result<Json<WarehouseLocation>, ApiError> {
    let updated_loc = crate::inventory::services::locations::update_location(
        &mut pool,
        user.organization_id,
        *id,
        &loc,
    )
    .await?;
    Ok(Json(updated_loc))
}
