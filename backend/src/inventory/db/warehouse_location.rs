/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use sqlx::PgConnection;
use uuid::Uuid;
use shared_core::inventory::models::warehouse::WarehouseLocation;
// =============================================================================
// Warehouse Location Operations (Multi-locations per Warehouse)
// =============================================================================

pub async fn locations_by_warehouse(conn: &mut PgConnection, warehouse_id: Uuid, org_id: Uuid) -> Result<Vec<WarehouseLocation>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM warehouse_locations WHERE warehouse_id = $1 AND organization_id = $2 ORDER BY display_label")
        .bind(warehouse_id)
        .bind(org_id)
        .fetch_all(conn)
        .await
}

pub async fn get_location(conn: &mut PgConnection, id: Uuid, org_id: Uuid) -> Result<Option<WarehouseLocation>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM warehouse_locations WHERE id = $1 AND organization_id = $2")
        .bind(id)
        .bind(org_id)
        .fetch_optional(conn)
        .await
}

pub async fn create_location(conn: &mut PgConnection, org_id: Uuid, loc: &WarehouseLocation) -> Result<WarehouseLocation, sqlx::Error> {
    sqlx::query_as(
        "INSERT INTO warehouse_locations (id, organization_id, warehouse_id, zone, aisle, shelf, bin, display_label, is_picking_location)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *",
    )
        .bind(Uuid::new_v4())
        .bind(org_id)
        .bind(loc.warehouse_id)
        .bind(&loc.zone)
        .bind(&loc.aisle)
        .bind(&loc.shelf)
        .bind(&loc.bin)
        .bind(&loc.display_label)
        .bind(loc.is_picking_location)
        .fetch_one(conn)
        .await
}

pub async fn update_location(conn: &mut PgConnection, id: Uuid, org_id: Uuid, loc: &WarehouseLocation) -> Result<WarehouseLocation, sqlx::Error> {
    sqlx::query_as(
        "UPDATE warehouse_locations SET zone = $1, aisle = $2, shelf = $3, bin = $4, display_label = $5, is_picking_location = $6
         WHERE id = $7 AND organization_id = $8 RETURNING *",
    )
        .bind(&loc.zone)
        .bind(&loc.aisle)
        .bind(&loc.shelf)
        .bind(&loc.bin)
        .bind(&loc.display_label)
        .bind(loc.is_picking_location)
        .bind(id)
        .bind(org_id)
        .fetch_one(conn)
        .await
}