/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use rocket_db_pools::sqlx::{
    self,
    PgConnection,
    Row,
};
use shared_core::payables::models::vendor_payment_allocation::VendorPaymentAllocation;
use uuid::Uuid;

fn from_row_to_vendor_payment_allocation(row: &sqlx::postgres::PgRow) -> VendorPaymentAllocation {
    VendorPaymentAllocation {
        id: row.get("id"),
        organization_id: row.get("organization_id"),
        vendor_invoice_id: row.get("vendor_invoice_id"),
        vendor_payment_id: row.get("vendor_payment_id"),
        allocated_amount: row.get("allocated_amount"),
        created_at: row.get("created_at"),
    }
}

pub(crate) async fn get(
    pool: &mut PgConnection,
    id: Uuid,
) -> Result<Option<VendorPaymentAllocation>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT *
        FROM vendor_payment_allocations
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map(|row| row.map(|r| from_row_to_vendor_payment_allocation(&r)))
}

pub(crate) async fn get_all(
    pool: &mut PgConnection,
    organization_id: Uuid,
) -> Result<Vec<VendorPaymentAllocation>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT *
        FROM vendor_payment_allocations
        WHERE organization_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(organization_id)
    .fetch_all(pool)
    .await
    .map(|rows| {
        rows.iter()
            .map(from_row_to_vendor_payment_allocation)
            .collect()
    })
}

pub(crate) async fn insert(
    pool: &mut PgConnection,
    vendor_payment_id: Uuid,
    req: &VendorPaymentAllocation,
) -> Result<VendorPaymentAllocation, sqlx::Error> {
    let row = sqlx::query(
        r#"
        INSERT INTO vendor_payment_allocations (
            organization_id,
            vendor_invoice_id,
            vendor_payment_id,
            allocated_amount
        )
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(req.organization_id)
    .bind(req.vendor_invoice_id)
    .bind(vendor_payment_id)
    .bind(req.allocated_amount)
    .fetch_one(pool)
    .await?;
    Ok(from_row_to_vendor_payment_allocation(&row))
}

pub(crate) async fn update(
    pool: &mut PgConnection,
    id: Uuid,
    req: &VendorPaymentAllocation,
) -> Result<VendorPaymentAllocation, sqlx::Error> {
    let row = sqlx::query(
        r#"
        UPDATE vendor_payment_allocations
        SET
            vendor_invoice_id = $1,
            vendor_payment_id = $2,
            allocated_amount = $3
        WHERE id = $4
        RETURNING *
        "#,
    )
    .bind(req.vendor_invoice_id)
    .bind(req.vendor_payment_id)
    .bind(req.allocated_amount)
    .bind(id)
    .fetch_one(pool)
    .await?;
    Ok(from_row_to_vendor_payment_allocation(&row))
}

pub(crate) async fn delete(pool: &mut PgConnection, id: Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM vendor_payment_allocations WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}
