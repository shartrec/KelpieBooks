/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 * (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
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
    dtos::inventory::{
        ItemStockBalancesResponse,
        ReceiveStockRequest,
        StockAdjustmentRequest,
    },
    models::warehouse_profile::{
        ItemWarehouseProfile,
        WarehouseInventoryBalance,
    },
};

use crate::{
    core::routes::security::AuthenticatedUser,
    inventory::services::{
        inventory as inventory_service,
        inventory::{
            adjust_stock,
            receive_vendor_stock,
        },
    },
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
        get_item_profile,
        save_item_profile,
        get_item_balances,
        receive_stock,
        post_stock_adjustment,
    ]
}

// =============================================================================
// Item Logistics Extension Profile Handlers
// =============================================================================

#[get("/api/inventory/items/<item_id>/profile")]
async fn get_item_profile(
    mut pool: Connection<DbKelpie>,
    item_id: PathUuid,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<UseInventory>,
) -> Result<Json<ItemWarehouseProfile>, ApiError> {
    let profile =
        inventory_service::get_item_warehouse_profile(&mut pool, user.organization_id, *item_id)
            .await?
            .ok_or_else(|| {
                ApiError::NotFound("Item warehouse configuration profile not found".to_string())
            })?;
    Ok(Json(profile))
}

#[put("/api/inventory/items/profile", data = "<profile>")]
async fn save_item_profile(
    mut pool: Connection<DbKelpie>,
    profile: Json<ItemWarehouseProfile>,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<ManageInventory>,
) -> Result<Json<ItemWarehouseProfile>, ApiError> {
    let saved =
        inventory_service::save_item_warehouse_profile(&mut pool, user.organization_id, &profile)
            .await?;
    Ok(Json(saved))
}

// =============================================================================
// Inventory Ledger Stock Position Handlers
// =============================================================================

#[get("/api/inventory/items/<item_id>/balances")]
async fn get_item_balances(
    mut pool: Connection<DbKelpie>,
    item_id: PathUuid,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<UseInventory>,
) -> Result<Json<ItemStockBalancesResponse>, ApiError> {
    let balances =
        inventory_service::get_balances_by_item(&mut pool, user.organization_id, *item_id).await?;
    Ok(Json(balances))
}

#[post("/api/inventory/receiving", data = "<req>")]
async fn receive_stock(
    mut pool: Connection<DbKelpie>,
    req: Json<ReceiveStockRequest>,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<ManageInventory>,
) -> Result<Json<Vec<WarehouseInventoryBalance>>, ApiError> {
    let updated_balances =
        receive_vendor_stock(&mut pool, user.organization_id, user.user_id, &req).await?;

    Ok(Json(updated_balances))
}

#[post("/api/inventory/adjustments", data = "<req>")]
async fn post_stock_adjustment(
    mut pool: Connection<DbKelpie>,
    req: Json<StockAdjustmentRequest>,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<ManageInventory>,
) -> Result<Json<Vec<WarehouseInventoryBalance>>, ApiError> {
    let updated_balances =
        adjust_stock(&mut pool, user.organization_id, user.user_id, &req).await?;

    Ok(Json(updated_balances))
}
