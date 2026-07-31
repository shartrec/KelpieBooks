/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 * (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use rust_decimal::Decimal;
use shared_core::inventory::models::stock_balance::{
    ReferenceType,
    StockTransaction,
    TransactionType,
};
use sqlx::PgConnection;
use uuid::Uuid;

pub struct NewStockTransaction<'a> {
    pub organization_id: Uuid,
    pub warehouse_id: Uuid,
    pub location_id: Uuid,
    pub item_id: Uuid,
    pub transaction_type: TransactionType,
    pub quantity_change: Decimal,
    pub reference_type: Option<ReferenceType>,
    pub reference_id: Option<Uuid>,
    pub notes: Option<&'a str>,
    pub created_by: Uuid,
}

/// Inserts an immutable transaction record into the audit ledger.
pub async fn log_transaction(
    conn: &mut PgConnection,
    entry: NewStockTransaction<'_>,
) -> Result<StockTransaction, sqlx::Error> {
    sqlx::query_as::<_, StockTransaction>(
        r#"
        INSERT INTO stock_transactions
            (id, organization_id, warehouse_id, location_id, item_id,
             transaction_type, quantity_change, reference_type, reference_id, notes, created_by)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING id, organization_id, warehouse_id, location_id, item_id,
                  transaction_type, quantity_change, reference_type, reference_id, notes, created_by, created_at
        "#,
    )
        .bind(Uuid::new_v4())
        .bind(entry.organization_id)
        .bind(entry.warehouse_id)
        .bind(entry.location_id)
        .bind(entry.item_id)
        .bind(entry.transaction_type)
        .bind(entry.quantity_change)
        .bind(entry.reference_type)
        .bind(entry.reference_id)
        .bind(entry.notes)
        .bind(entry.created_by)
        .fetch_one(conn)
        .await
}

/// Queries recent movement history for a specific item in a warehouse.
pub async fn get_history_for_item(
    conn: &mut PgConnection,
    item_id: Uuid,
    org_id: Uuid,
    limit: i64,
) -> Result<Vec<StockTransaction>, sqlx::Error> {
    sqlx::query_as::<_, StockTransaction>(
        r#"
        SELECT id, organization_id, warehouse_id, location_id, item_id,
               transaction_type, quantity_change, reference_type, reference_id, notes, created_by, created_at
        FROM stock_transactions
        WHERE item_id = $1 AND organization_id = $2
        ORDER BY created_at DESC
        LIMIT $3
        "#,
    )
        .bind(item_id)
        .bind(org_id)
        .bind(limit)
        .fetch_all(conn)
        .await
}
