/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 * (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use rocket_db_pools::Connection;
use shared_core::inventory::models::warehouse::Warehouse;
use uuid::Uuid;

use crate::{
    inventory::db::{
        location as locations_db,
        warehouse as warehouse_db,
    },
    util::ApiError,
    DbKelpie,
};
// =============================================================================
// Warehouse Service Operations
// =============================================================================

pub async fn get_warehouses(
    pool: &mut Connection<DbKelpie>,
    org_id: Uuid,
) -> Result<Vec<Warehouse>, sqlx::Error> {
    warehouse_db::all_warehouses(pool, org_id).await
}

pub async fn get_warehouse(
    pool: &mut Connection<DbKelpie>,
    id: Uuid,
    org_id: Uuid,
) -> Result<Option<Warehouse>, sqlx::Error> {
    warehouse_db::get_warehouse(pool, id, org_id).await
}

pub async fn create_warehouse(
    pool: &mut Connection<DbKelpie>,
    org_id: Uuid,
    wh: &Warehouse,
) -> Result<Warehouse, sqlx::Error> {
    warehouse_db::create_warehouse(pool, org_id, wh).await
}

pub async fn update_warehouse(
    pool: &mut Connection<DbKelpie>,
    id: Uuid,
    org_id: Uuid,
    wh: &Warehouse,
) -> Result<Warehouse, sqlx::Error> {
    warehouse_db::update_warehouse(pool, id, org_id, wh).await
}

pub async fn delete_warehouse(
    pool: &mut Connection<DbKelpie>,
    id: Uuid,
    org_id: Uuid,
) -> Result<u64, ApiError> {
    // 💡 Business Guard: Prevent removing a warehouse if locations are nested under it
    let locations = locations_db::all_by_warehouse(pool, id, org_id).await?;
    if !locations.is_empty() {
        return Err(ApiError::Conflict(
            "Warehouse contains active storage locations and cannot be deleted.".to_string(),
        ));
    }

    let result = warehouse_db::delete_warehouse(pool, id, org_id).await?;
    Ok(result.rows_affected())
}
