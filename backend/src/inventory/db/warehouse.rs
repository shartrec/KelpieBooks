/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 * (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use rocket_db_pools::sqlx::{
    self,
    PgConnection,
};
use shared_core::{
    inventory::models::warehouse::Warehouse,
    OrgId,
    WarehouseId,
};
// =============================================================================
// Warehouse Operations
// =============================================================================

pub async fn all_warehouses(
    conn: &mut PgConnection,
    org_id: OrgId,
) -> Result<Vec<Warehouse>, sqlx::Error> {
    sqlx::query_as!(
        Warehouse,
        "SELECT * FROM warehouses WHERE organization_id = $1 ORDER BY code",
        *org_id,
    )
    .fetch_all(conn)
    .await
}

pub async fn get_warehouse(
    conn: &mut PgConnection,
    org_id: OrgId,
    id: WarehouseId,
) -> Result<Option<Warehouse>, sqlx::Error> {
    sqlx::query_as!(
        Warehouse,
        "SELECT * FROM warehouses WHERE id = $1 AND organization_id = $2",
        *id,
        *org_id,
    )
    .fetch_optional(conn)
    .await
}

pub async fn create_warehouse(
    conn: &mut PgConnection,
    org_id: OrgId,
    wh: &Warehouse,
) -> Result<Warehouse, sqlx::Error> {
    sqlx::query_as!(
        Warehouse,
        "INSERT INTO warehouses (organization_id, code, name, is_active) VALUES ($1, $2, $3, $4) RETURNING *",
        *org_id,
        &wh.code,
        &wh.name,
        wh.is_active
    )
        .fetch_one(conn)
        .await
}

pub async fn update_warehouse(
    conn: &mut PgConnection,
    org_id: OrgId,
    id: WarehouseId,
    wh: &Warehouse,
) -> Result<Warehouse, sqlx::Error> {
    sqlx::query_as!(
        Warehouse,
        "UPDATE warehouses SET code = $1, name = $2, is_active = $3 WHERE id = $4 AND organization_id = $5 RETURNING *",
        &wh.code,
        &wh.name,
        wh.is_active,
        *id,
        *org_id,
    )
        .fetch_one(conn)
        .await
}

pub async fn delete_warehouse(
    conn: &mut PgConnection,
    org_id: OrgId,
    id: WarehouseId,
) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
    sqlx::query!(
        "DELETE FROM warehouses WHERE id = $1 AND organization_id = $2",
        *id,
        *org_id,
    )
    .execute(conn)
    .await
}
