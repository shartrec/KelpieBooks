/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 * (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use std::collections::HashMap;

use chrono::NaiveDate;
use rocket_db_pools::Connection;
use rust_decimal::{
    prelude::FromPrimitive,
    Decimal,
};
use shared_core::{
    inventory::{
        dtos::inventory::{
            AdjustmentReason,
            ItemStockBalancesResponse,
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
    },
    ledger::{
        models::system_tag::SystemTag,
        requests::transaction::{
            CreateTransactionRequest,
            JournalEntryLine,
        },
    },
};
use sqlx::{
    Acquire,
    Error,
    PgConnection,
};
use uuid::Uuid;

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
    ledger::services::{
        account_service,
        account_service::get_system_accounts,
    },
    util::ApiError,
    DbKelpie,
};

pub struct InventorySystemAccounts {
    pub inventory_asset_id: Uuid,
    pub received_not_invoiced_id: Uuid,
    pub inventory_adjustment_id: Uuid,
    pub cogs_id: Uuid,
}

impl InventorySystemAccounts {
    pub fn from_map(map: &HashMap<SystemTag, Uuid>) -> Result<Self, ApiError> {
        let get_account = |tag: SystemTag| {
            map.get(&tag).copied().ok_or_else(|| {
                ApiError::NotFound(format!("Missing system account mapping for tag: {:?}", tag))
            })
        };

        Ok(Self {
            inventory_asset_id: get_account(SystemTag::InventoryAsset)?,
            received_not_invoiced_id: get_account(SystemTag::ReceivedNotInvoiced)?,
            inventory_adjustment_id: get_account(SystemTag::InventoryAdjustment)?,
            cogs_id: get_account(SystemTag::CostOfGoodsSold)?,
        })
    }
}

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

        //todo get total amount
        let total_amount = Decimal::from_f32(1.0).unwrap();
        let _tx_id = post_receive_journal_entry(
            &mut tx,
            org_id,
            total_amount,
            &*req.po_number.clone().unwrap_or("n/a".to_string()),
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

        let old_balance = match get_balance_for_location(
            &mut *tx,
            line.item_id,
            line.location_id,
            org_id,
        )
        .await
        {
            Ok(balance) => balance
                .map(|wib| wib.quantity_on_hand)
                .unwrap_or(Decimal::ZERO),
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

        //todo get total amount
        let total_amount = Decimal::from_f32(1.0).unwrap();
        let _tx_id = post_receive_journal_entry(&mut tx, org_id, total_amount, &audit_note).await?;

        updated_balances.push(balance);
    }

    tx.commit().await?;

    Ok(updated_balances)
}

pub async fn post_receive_journal_entry(
    conn: &mut PgConnection,
    org_id: Uuid,
    total_value: Decimal, // quantity * unit_cost
    reference: &str,
) -> Result<Uuid, ApiError> {
    if total_value <= Decimal::ZERO {
        return Ok(Uuid::nil()); // Skip 0-value postings
    }

    // 1. Fetch system account mappings
    let system_accounts_map = get_system_accounts(conn, org_id).await?;
    let accounts = InventorySystemAccounts::from_map(&system_accounts_map)?;

    // 2. Draft Journal Entry
    // Debit: Inventory Asset (Increases Asset)
    // Credit: Received Not Invoiced (Increases Liability)
    let description = Some(format!("Inventory receipt {}", reference));
    let jels = vec![
        JournalEntryLine {
            line_id: Uuid::new_v4(),
            account_id: accounts.inventory_asset_id,
            debit: total_value,
            credit: Decimal::ZERO,
            description: description.clone(),
        },
        JournalEntryLine {
            line_id: Uuid::new_v4(),
            account_id: accounts.received_not_invoiced_id,
            debit: Decimal::ZERO,
            credit: total_value,
            description: description.clone(),
        },
    ];

    let ct_req = CreateTransactionRequest {
        date: NaiveDate::default(),
        description,
        reference: Some(reference.to_string()),
        entries: jels,
    };
    let journal_id = account_service::create_transaction(conn, org_id, &ct_req).await?;

    Ok(journal_id)
}

pub async fn post_adjustment_journal_entry(
    conn: &mut PgConnection,
    org_id: Uuid,
    adjustment_value: Decimal, // positive = stock gain, negative = stock loss
    reference: &str,
) -> Result<Uuid, ApiError> {
    if adjustment_value.is_zero() {
        return Ok(Uuid::nil());
    }

    let system_accounts_map = get_system_accounts(conn, org_id).await?;
    let accounts = InventorySystemAccounts::from_map(&system_accounts_map)?;

    let (debit_account, credit_account, amount) = if adjustment_value > Decimal::ZERO {
        // Gain: Debit Asset, Credit Adjustment (P&L Gain)
        (
            accounts.inventory_asset_id,
            accounts.inventory_adjustment_id,
            adjustment_value,
        )
    } else {
        // Loss: Debit Adjustment (P&L Expense), Credit Asset
        let abs_amount = adjustment_value.abs();
        (
            accounts.inventory_adjustment_id,
            accounts.inventory_asset_id,
            abs_amount,
        )
    };

    let description = Some(format!("Inventory receipt {}", reference));
    let jels = vec![
        JournalEntryLine {
            line_id: Uuid::new_v4(),
            account_id: debit_account,
            debit: amount,
            credit: Decimal::ZERO,
            description: description.clone(),
        },
        JournalEntryLine {
            line_id: Uuid::new_v4(),
            account_id: credit_account,
            debit: Decimal::ZERO,
            credit: amount,
            description: description.clone(),
        },
    ];

    let ct_req = CreateTransactionRequest {
        date: NaiveDate::default(),
        description,
        reference: Some(reference.to_string()),
        entries: jels,
    };
    let journal_id = account_service::create_transaction(conn, org_id, &ct_req).await?;

    Ok(journal_id)
}
