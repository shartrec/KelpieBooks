/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 * (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use rocket_db_pools::sqlx::{self, PgConnection};
use shared_core::inventory::models::warehouse::Warehouse;
use uuid::Uuid;

// =============================================================================
// Warehouse Operations
// =============================================================================

pub async fn all_warehouses(conn: &mut PgConnection, org_id: Uuid) -> Result<Vec<Warehouse>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM warehouses WHERE organization_id = $1 ORDER BY code")
        .bind(org_id)
        .fetch_all(conn)
        .await
}

pub async fn get_warehouse(conn: &mut PgConnection, id: Uuid, org_id: Uuid) -> Result<Option<Warehouse>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM warehouses WHERE id = $1 AND organization_id = $2")
        .bind(id)
        .bind(org_id)
        .fetch_optional(conn)
        .await
}

pub async fn create_warehouse(conn: &mut PgConnection, org_id: Uuid, wh: &Warehouse) -> Result<Warehouse, sqlx::Error> {
    sqlx::query_as(
        "INSERT INTO warehouses (id, organization_id, code, name, is_active) VALUES ($1, $2, $3, $4, $5) RETURNING *",
    )
        .bind(Uuid::new_v4())
        .bind(org_id)
        .bind(&wh.code)
        .bind(&wh.name)
        .bind(wh.is_active)
        .fetch_one(conn)
        .await
}

pub async fn update_warehouse(conn: &mut PgConnection, id: Uuid, org_id: Uuid, wh: &Warehouse) -> Result<Warehouse, sqlx::Error> {
    sqlx::query_as(
        "UPDATE warehouses SET code = $1, name = $2, is_active = $3 WHERE id = $4 AND organization_id = $5 RETURNING *",
    )
        .bind(&wh.code)
        .bind(&wh.name)
        .bind(wh.is_active)
        .bind(id)
        .bind(org_id)
        .fetch_one(conn)
        .await
}

pub async fn delete_warehouse(conn: &mut PgConnection, id: Uuid, org_id: Uuid) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
    sqlx::query("DELETE FROM warehouses WHERE id = $1 AND organization_id = $2")
        .bind(id)
        .bind(org_id)
        .execute(conn)
        .await
}

