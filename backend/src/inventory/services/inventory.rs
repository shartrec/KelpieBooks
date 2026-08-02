/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 * (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use rocket_db_pools::Connection;
use rust_decimal::Decimal;
use shared_core::inventory::{
    dtos::inventory::{
        AdjustmentReason,
        ReceiveStockRequest,
        StockAdjustmentRequest,
    },
    models::{
        stock_balance::{
            ReferenceType,
            TransactionType,
        },
        warehouse_profile::{
            ItemWarehouseProfile,
            WarehouseInventoryBalance,
        },
    },
};
use sqlx::{Acquire, Error};
use uuid::Uuid;
use shared_core::inventory::dtos::inventory::ItemStockBalancesResponse;
use crate::{
    inventory::db::{
        inventory as inventory_db,
        inventory::{
            adjust_on_hand,
            get_balance_for_location,
        },
        stock_transaction,
        stock_transaction::NewStockTransaction,
    },
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
) -> Result<ItemStockBalancesResponse, Error> {
    inventory_db::get_item_stock_balances(pool, item_id, org_id).await
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

    let balance =
        inventory_db::update_inventory_quantities(pool, id, org_id, qty_on_hand, qty_allocated)
            .await?;

    Ok(balance)
}

pub async fn receive_vendor_stock(
    pool: &mut Connection<DbKelpie>,
    org_id: Uuid,
    user_id: Uuid,
    req: &ReceiveStockRequest,
) -> Result<Vec<WarehouseInventoryBalance>, ApiError> {
    if req.items.is_empty() {
        return Err(ApiError::BadRequest(
            "Cannot receive an empty item list.".to_string(),
        ));
    }

    // Begin ACID transaction
    let mut tx = pool.begin().await?;
    let mut updated_balances = Vec::new();

    // Build standard reference string combining vendor/PO details for the ledger
    let ref_notes = match (&req.po_number, &req.notes) {
        (Some(po), Some(n)) => format!("PO: {} | {}", po, n),
        (Some(po), None) => format!("PO: {}", po),
        (None, Some(n)) => n.clone(),
        (None, None) => "Vendor Stock Receipt".to_string(),
    };

    for line in &req.items {
        if line.quantity <= rust_decimal::Decimal::ZERO {
            return Err(ApiError::BadRequest(
                "Received quantity must be greater than zero.".to_string(),
            ));
        }

        // 1. Update/Upsert the location balance on hand
        let balance = adjust_on_hand(
            &mut tx,
            org_id,
            req.warehouse_id,
            line.location_id,
            line.item_id,
            line.quantity,
        )
        .await?;

        // 2. Write immutable transaction audit entry
        stock_transaction::log_transaction(
            &mut tx,
            NewStockTransaction {
                organization_id: org_id,
                warehouse_id: req.warehouse_id,
                location_id: line.location_id,
                item_id: line.item_id,
                transaction_type: TransactionType::Receipt,
                quantity_change: line.quantity,
                reference_type: Some(ReferenceType::PurchaseOrder),
                reference_id: req.vendor_id, // Vendor ID tracked as ref target until full PO entity exists
                notes: Some(&ref_notes),
                created_by: user_id,
            },
        )
        .await?;

        updated_balances.push(balance);
    }

    // Commit changes
    tx.commit().await?;

    Ok(updated_balances)
}

pub async fn adjust_stock(
    pool: &mut Connection<DbKelpie>,
    org_id: Uuid,
    user_id: Uuid,
    req: &StockAdjustmentRequest,
) -> Result<Vec<WarehouseInventoryBalance>, ApiError> {
    if req.items.is_empty() {
        return Err(ApiError::BadRequest(
            "Adjustment list cannot be empty.".to_string(),
        ));
    }

    let mut tx = pool.begin().await?;
    let mut updated_balances = Vec::new();

    for line in &req.items {
        if line.quantity_delta.is_zero() {
            continue;
        }

        let old_balance =
            match get_balance_for_location(&mut *tx, line.item_id, line.location_id, org_id).await {
                Ok(balance) => balance.map(|wib| wib.quantity_on_hand).unwrap_or(Decimal::ZERO),
                Err(e) => return Err(ApiError::Db(e)),
            };
        if (old_balance + line.quantity_delta) < Decimal::ZERO {
            return Err(ApiError::BadRequest(format!(
                "Adjustment rejected: Item {} at location {} would have a negative balance.",
                line.item_id, line.location_id
            )));
        }

        // Apply balance adjustment (will fail via WHERE clause if balance drops below 0)
        let balance = adjust_on_hand(
            &mut *tx,
            org_id,
            req.warehouse_id,
            line.location_id,
            line.item_id,
            line.quantity_delta,
        )
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => ApiError::BadRequest(format!(
                "Adjustment rejected: Item {} at location {} would have a negative balance.",
                line.item_id, line.location_id
            )),
            other => ApiError::Db(other),
        })?;

        // Format audit trail notes
        let reason_str = match line.reason {
            AdjustmentReason::CycleCount => "Cycle Count",
            AdjustmentReason::Damage => "Damage / Write-off",
            AdjustmentReason::Scrap => "Scrap",
            AdjustmentReason::AuditCorrection => "Audit Correction",
            AdjustmentReason::FoundStock => "Found Stock",
            AdjustmentReason::Other => "Other",
        };

        let audit_note = match &line.notes {
            Some(notes) => format!("[Reason: {}] {}", reason_str, notes),
            None => format!("[Reason: {}]", reason_str),
        };

        // Write immutable ledger entry
        stock_transaction::log_transaction(
            &mut *tx,
            NewStockTransaction {
                organization_id: org_id,
                warehouse_id: req.warehouse_id,
                location_id: line.location_id,
                item_id: line.item_id,
                transaction_type: TransactionType::Adjustment,
                quantity_change: line.quantity_delta,
                reference_type: Some(ReferenceType::ManualAdjustment),
                reference_id: None,
                notes: Some(&audit_note),
                created_by: user_id,
            },
        )
        .await?;

        updated_balances.push(balance);
    }

    tx.commit().await?;

    Ok(updated_balances)
}
