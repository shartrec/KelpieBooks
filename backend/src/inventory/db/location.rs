/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 * (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use shared_core::inventory::models::warehouse::WarehouseLocation;
use sqlx::{
    PgConnection,
    Postgres,
    QueryBuilder,
};
use uuid::Uuid;

pub async fn all_by_warehouse(
    conn: &mut PgConnection,
    warehouse_id: Uuid,
    org_id: Uuid,
) -> Result<Vec<WarehouseLocation>, sqlx::Error> {
    sqlx::query_as!(
        WarehouseLocation,
        r#"
        SELECT id, warehouse_id, organization_id, zone, aisle, shelf, bin, display_label, is_picking_location, created_at
        FROM warehouse_locations
        WHERE warehouse_id = $1 AND organization_id = $2
        ORDER BY zone, aisle, shelf, bin
        "#,
        warehouse_id,
        org_id
    )
        .fetch_all(conn)
        .await
}

pub async fn get_location(
    conn: &mut PgConnection,
    id: Uuid,
    org_id: Uuid,
) -> Result<Option<WarehouseLocation>, sqlx::Error> {
    sqlx::query_as!(
        WarehouseLocation,
        "SELECT * FROM warehouse_locations WHERE id = $1 AND organization_id = $2",
        id,
        org_id
    )
        .fetch_optional(conn)
        .await
}

pub async fn create_location(
    conn: &mut PgConnection,
    org_id: Uuid,
    loc: &WarehouseLocation,
) -> Result<WarehouseLocation, sqlx::Error> {
    sqlx::query_as!(
        WarehouseLocation,
        "INSERT INTO warehouse_locations (id, organization_id, warehouse_id, zone, aisle, shelf, bin, display_label, is_picking_location)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *",
        Uuid::new_v4(),
        org_id,
        loc.warehouse_id,
        loc.zone,
        loc.aisle,
        loc.shelf,
        loc.bin,
        loc.display_label,
        loc.is_picking_location
    )
        .fetch_one(conn)
        .await
}

pub async fn update_location(
    conn: &mut PgConnection,
    id: Uuid,
    org_id: Uuid,
    loc: &WarehouseLocation,
) -> Result<WarehouseLocation, sqlx::Error> {
    sqlx::query_as!(
        WarehouseLocation,
        "UPDATE warehouse_locations SET zone = $1, aisle = $2, shelf = $3, bin = $4, display_label = $5, is_picking_location = $6
         WHERE id = $7 AND organization_id = $8 RETURNING *",
        loc.zone,
        loc.aisle,
        loc.shelf,
        loc.bin,
        loc.display_label,
        loc.is_picking_location,
        id,
        org_id
    )
        .fetch_one(conn)
        .await
}

pub async fn bulk_insert(
    conn: &mut PgConnection,
    locations: &[WarehouseLocation],
) -> Result<(), sqlx::Error> {
    if locations.is_empty() {
        return Ok(());
    }

    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "INSERT INTO warehouse_locations (id, warehouse_id, organization_id, zone, aisle, shelf, bin, display_label, is_picking_location) "
    );

    query_builder.push_values(locations, |mut b, loc| {
        b.push_bind(loc.id)
            .push_bind(loc.warehouse_id)
            .push_bind(loc.organization_id)
            .push_bind(&loc.zone)
            .push_bind(&loc.aisle)
            .push_bind(&loc.shelf)
            .push_bind(&loc.bin)
            .push_bind(&loc.display_label)
            .push_bind(loc.is_picking_location);
    });

    // Handle potential duplicate label conflicts within a warehouse gracefully
    query_builder.push(" ON CONFLICT (warehouse_id, display_label) DO NOTHING");

    let query = query_builder.build();
    query.execute(conn).await?;

    Ok(())
}
