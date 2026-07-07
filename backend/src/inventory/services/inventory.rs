/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 * (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use rocket_db_pools::Connection;
use rust_decimal::Decimal;
use uuid::Uuid;
use shared_core::inventory::models::warehouse_profile::{ItemWarehouseProfile, WarehouseInventoryBalance};
use crate::{
    inventory::db::inventory as inventory_db,
    util::ApiError,
    DbKelpie,
};

// =============================================================================
// Item Warehouse Profile Service
// =============================================================================

pub async fn get_item_warehouse_profile(
    pool: &mut Connection<DbKelpie>,
    item_id: Uuid,
    org_id: Uuid,
) -> Result<Option<ItemWarehouseProfile>, sqlx::Error> {
    inventory_db::get_warehouse_profile(pool, item_id, org_id).await
}

pub async fn save_item_warehouse_profile(
    pool: &mut Connection<DbKelpie>,
    org_id: Uuid,
    profile: &ItemWarehouseProfile,
) -> Result<ItemWarehouseProfile, sqlx::Error> {
    inventory_db::upsert_warehouse_profile(pool, org_id, profile).await
}

// =============================================================================
// Inventory Ledger Balance Service
// =============================================================================

pub async fn get_balances_by_item(
    pool: &mut Connection<DbKelpie>,
    item_id: Uuid,
    org_id: Uuid,
) -> Result<Vec<WarehouseInventoryBalance>, sqlx::Error> {
    inventory_db::balances_by_item(pool, item_id, org_id).await
}

pub async fn get_balance_at_location(
    pool: &mut Connection<DbKelpie>,
    item_id: Uuid,
    location_id: Uuid,
    org_id: Uuid,
) -> Result<Option<WarehouseInventoryBalance>, sqlx::Error> {
    inventory_db::get_balance_for_location(pool, item_id, location_id, org_id).await
}

pub async fn update_stock_levels(
    pool: &mut Connection<DbKelpie>,
    id: Uuid,
    org_id: Uuid,
    qty_on_hand: Decimal,
    qty_allocated: Decimal,
) -> Result<WarehouseInventoryBalance, ApiError> {
    // 💡 Business Guard: Check for negative physical balances before adjusting quantities
    if qty_on_hand.is_sign_negative() {
        return Err(ApiError::BadRequest(
            "Physical stock levels on hand cannot drop below zero.".to_string(),
        ));
    }

    let balance = inventory_db::update_inventory_quantities(
        pool,
        id,
        org_id,
        qty_on_hand,
        qty_allocated
    ).await?;

    Ok(balance)
}