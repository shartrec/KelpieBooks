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
};
use shared_core::payables::models::vendor_payment_allocation::VendorPaymentAllocation;
use uuid::Uuid;

pub(crate) async fn get(
    pool: &mut PgConnection,
    id: Uuid,
) -> Result<Option<VendorPaymentAllocation>, sqlx::Error> {
    sqlx::query_as!(
        VendorPaymentAllocation,
        r#"
        SELECT *
        FROM vendor_payment_allocations
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

pub(crate) async fn insert(
    pool: &mut PgConnection,
    vendor_payment_id: Uuid,
    req: &VendorPaymentAllocation,
) -> Result<VendorPaymentAllocation, sqlx::Error> {
    let row = sqlx::query_as!(
        VendorPaymentAllocation,
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
        req.organization_id,
        req.vendor_invoice_id,
        vendor_payment_id,
        req.allocated_amount,
    )
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub(crate) async fn update(
    pool: &mut PgConnection,
    id: Uuid,
    req: &VendorPaymentAllocation,
) -> Result<VendorPaymentAllocation, sqlx::Error> {
    let row = sqlx::query_as!(
        VendorPaymentAllocation,
        r#"
        UPDATE vendor_payment_allocations
        SET
            vendor_invoice_id = $1,
            vendor_payment_id = $2,
            allocated_amount = $3
        WHERE id = $4
        RETURNING *
        "#,
        req.vendor_invoice_id,
        req.vendor_payment_id,
        req.allocated_amount,
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub(crate) async fn delete(pool: &mut PgConnection, id: Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM vendor_payment_allocations WHERE id = $1", id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}
