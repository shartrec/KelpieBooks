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
use rust_decimal::Decimal;
use shared_core::inventory::models::warehouse_profile::{
    ItemWarehouseProfile,
    WarehouseInventoryBalance,
};
use uuid::Uuid;
use shared_core::inventory::dtos::inventory::{ItemLocationBalanceDto, ItemStockBalancesResponse};
use shared_core::sales::models::item::ItemType;
// =============================================================================
// Item Warehouse Profile Operations (Physical Attributes Extension)
// =============================================================================

pub async fn get_warehouse_profile(
    conn: &mut PgConnection,
    item_id: Uuid,
    org_id: Uuid,
) -> Result<Option<ItemWarehouseProfile>, sqlx::Error> {
    sqlx::query_as!(
        ItemWarehouseProfile,
        "SELECT * FROM item_warehouse_profiles WHERE item_id = $1 AND organization_id = $2",
        item_id,
        org_id
    )
    .fetch_optional(conn)
    .await
}

pub async fn upsert_warehouse_profile(
    conn: &mut PgConnection,
    org_id: Uuid,
    profile: &ItemWarehouseProfile,
) -> Result<ItemWarehouseProfile, sqlx::Error> {
    sqlx::query_as!(
        ItemWarehouseProfile,
        "INSERT INTO item_warehouse_profiles (item_id, organization_id, weight_kg, length_cm, width_cm, height_cm, reorder_point, safety_stock, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW())
         ON CONFLICT (item_id)
         DO UPDATE SET
            weight_kg = EXCLUDED.weight_kg,
            length_cm = EXCLUDED.length_cm,
            width_cm = EXCLUDED.width_cm,
            height_cm = EXCLUDED.height_cm,
            reorder_point = EXCLUDED.reorder_point,
            safety_stock = EXCLUDED.safety_stock,
            updated_at = NOW()
         RETURNING *",
        profile.item_id,
        org_id,
        profile.weight_kg,
        profile.length_cm,
        profile.width_cm,
        profile.height_cm,
        profile.reorder_point,
        profile.safety_stock
    )
        .fetch_one(conn)
        .await
}

// =============================================================================
// Warehouse Inventory Balance Ledger Operations
// =============================================================================
pub async fn get_item_stock_balances(
    conn: &mut PgConnection,
    item_id: Uuid,
    org_id: Uuid,
) -> Result<ItemStockBalancesResponse, sqlx::Error> {

    // check the item is a stocked item first
    let it = sqlx::query_scalar!(r#"SELECT item_type  AS "type_id: ItemType"  FROM items
            WHERE id = $1 AND organization_id = $2"#,
        item_id,
        org_id
    )
        .fetch_optional(& mut *conn).await?;

    match it {
        Some(ItemType::Stocked) => {}
        _ => {
            return Ok(ItemStockBalancesResponse {
                item_id,
                total_on_hand: None,
                total_allocated: None,
                total_available: None,
                location_balances: vec![],
            });
        }
    }

    let location_balances = sqlx::query_as!(
        ItemLocationBalanceDto,
        r#"
        SELECT
            w.id AS warehouse_id,
            w.code AS warehouse_code,
            w.name AS warehouse_name,
            loc.id AS location_id,
            loc.display_label AS location_display_label,
            loc.is_picking_location,
            b.quantity_on_hand,
            b.quantity_allocated,
            (b.quantity_on_hand - b.quantity_allocated) AS quantity_available
        FROM warehouse_inventory_balances b
        INNER JOIN warehouses w ON w.id = b.warehouse_id
        INNER JOIN warehouse_locations loc ON loc.id = b.location_id
        WHERE b.organization_id = $1 AND b.item_id = $2
        ORDER BY w.name ASC, loc.display_label ASC
        "#,
        org_id,
        item_id
    )
        .fetch_all(conn)
        .await?;

    let total_on_hand = location_balances.iter().map(|b: &ItemLocationBalanceDto | b.quantity_on_hand.unwrap_or(Decimal::ZERO)).sum();
    let total_allocated = location_balances.iter().map(|b: &ItemLocationBalanceDto| b.quantity_allocated.unwrap_or(Decimal::ZERO)).sum();
    let total_available = total_on_hand - total_allocated;

    Ok(ItemStockBalancesResponse {
        item_id,
        total_on_hand: Some(total_on_hand),
        total_allocated: Some(total_allocated),
        total_available: Some(total_available),
        location_balances,
    })
}

pub async fn get_balance_for_location(
    conn: &mut PgConnection,
    item_id: Uuid,
    location_id: Uuid,
    org_id: Uuid,
) -> Result<Option<WarehouseInventoryBalance>, sqlx::Error> {
    sqlx::query_as!(
        WarehouseInventoryBalance,
        "SELECT * FROM warehouse_inventory_balances WHERE item_id = $1 AND location_id = $2 AND organization_id = $3",
        item_id,
        location_id,
        org_id
        )
        .fetch_optional(conn)
        .await
}

/// Atomically adjusts the `quantity_on_hand` for an item at a location.
/// Creates a new balance record if one doesn't exist for this location/item combination.
pub async fn adjust_on_hand(
    conn: &mut PgConnection,
    org_id: Uuid,
    warehouse_id: Uuid,
    location_id: Uuid,
    item_id: Uuid,
    delta: Decimal,
) -> Result<WarehouseInventoryBalance, sqlx::Error> {
    sqlx::query_as!(
        WarehouseInventoryBalance,
        r#"
        INSERT INTO warehouse_inventory_balances
            (id, organization_id, warehouse_id, location_id, item_id, quantity_on_hand, quantity_allocated, unit_cost)
        VALUES ($1, $2, $3, $4, $5, $6, 0.0, 0.0)
        ON CONFLICT (location_id, item_id)
        DO UPDATE SET
            quantity_on_hand = warehouse_inventory_balances.quantity_on_hand + EXCLUDED.quantity_on_hand,
            updated_at = NOW()
        RETURNING *
        "#,
        Uuid::new_v4(),
        org_id,
        warehouse_id,
        location_id,
        item_id,
        delta
    )
        .fetch_one(conn)
        .await
}

/// Adjusts allocated quantities (reserving/unreserving stock for orders).
pub async fn adjust_allocated(
    conn: &mut PgConnection,
    location_id: Uuid,
    item_id: Uuid,
    org_id: Uuid,
    delta: Decimal,
) -> Result<WarehouseInventoryBalance, sqlx::Error> {
    sqlx::query_as!(
        WarehouseInventoryBalance,
        r#"
        UPDATE warehouse_inventory_balances
        SET quantity_allocated = quantity_allocated + $1,
            updated_at = NOW()
        WHERE location_id = $2 AND item_id = $3 AND organization_id = $4
        RETURNING *
        "#,
    delta,
    location_id,
    item_id,
    org_id
    )
    .fetch_one(conn)
    .await
}
