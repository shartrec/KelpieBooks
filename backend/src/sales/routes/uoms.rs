/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
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
use shared_core::sales::models::item::UnitOfMeasure;

use crate::{
    core::routes::security::AuthenticatedUser,
    sales::services::uom_service,
    security::{
        ManageSales,
        RequirePrivilege,
        UseSales,
    },
    util::{
        types::PathUuid,
        ApiError,
    },
    DbKelpie,
};

pub(crate) fn routes() -> Vec<Route> {
    routes![get_uoms, get_uom, create_uom, update_uom, delete_uom,]
}

#[get("/api/sales/uoms")]
async fn get_uoms(
    mut pool: Connection<DbKelpie>,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<UseSales>,
) -> Result<Json<Vec<UnitOfMeasure>>, ApiError> {
    let uoms = uom_service::get_uoms(&mut pool, user.organization_id).await?;
    Ok(Json(uoms))
}

#[get("/api/sales/uoms/<id>")]
async fn get_uom(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<UseSales>,
) -> Result<Json<UnitOfMeasure>, ApiError> {
    let uom = uom_service::get_uom(&mut pool, *id, user.organization_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Unit of Measure not found".to_string()))?;
    Ok(Json(uom))
}

#[post("/api/sales/uoms", data = "<uom>")]
async fn create_uom(
    mut pool: Connection<DbKelpie>,
    uom: Json<UnitOfMeasure>,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<ManageSales>,
) -> Result<Json<UnitOfMeasure>, ApiError> {
    let new_uom = uom_service::create_uom(&mut pool, user.organization_id, &uom).await?;
    Ok(Json(new_uom))
}

#[put("/api/sales/uoms/<id>", data = "<uom>")]
async fn update_uom(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
    uom: Json<UnitOfMeasure>,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<ManageSales>,
) -> Result<Json<UnitOfMeasure>, ApiError> {
    let updated_uom = uom_service::update_uom(&mut pool, *id, user.organization_id, &uom).await?;
    Ok(Json(updated_uom))
}

#[delete("/api/sales/uoms/<id>")]
async fn delete_uom(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<ManageSales>,
) -> Result<&'static str, ApiError> {
    let rows_affected = uom_service::delete_uom(&mut pool, *id, user.organization_id).await?;
    if rows_affected == 0 {
        return Err(ApiError::NotFound("Unit of Measure not found.".to_string()));
    }
    Ok("Unit of Measure deleted successfully.")
}
