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
use shared_core::payables::{
    models::vendor_payment::VendorPayment,
    requests::vendor_payment::CreateVendorPaymentRequest,
};
use uuid::Uuid;
use shared_core::OrgId;

pub(crate) async fn get(
    pool: &mut PgConnection,
    id: Uuid,
) -> Result<Option<VendorPayment>, sqlx::Error> {
    sqlx::query_as!(
        VendorPayment,
        r#"
        SELECT *
        FROM vendor_payments
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

pub(crate) async fn get_all_by_invoice(
    pool: &mut PgConnection,
    invoice_id: Uuid,
) -> Result<Vec<VendorPayment>, sqlx::Error> {
    sqlx::query_as!(
        VendorPayment,
        r#"
        SELECT vp.*
        FROM vendor_payments vp
        JOIN vendor_payment_allocations vpa ON vp.id = vpa.vendor_payment_id
        WHERE vpa.vendor_invoice_id = $1
        ORDER BY vp.payment_date DESC, vp.created_at DESC
        "#,
        invoice_id
    )
    .fetch_all(pool)
    .await
}

pub(crate) async fn insert(
    pool: &mut PgConnection,
    org_id: OrgId,
    transaction_id: Uuid,
    req: &CreateVendorPaymentRequest,
) -> Result<VendorPayment, sqlx::Error> {
    let row = sqlx::query_as!(
        VendorPayment,
        r#"
        INSERT INTO vendor_payments (
            organization_id,
            partner_id,
            transaction_id,
            payment_date,
            paid_from_account,
            amount,
            reference
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#,
        *org_id,
        req.partner_id,
        transaction_id,
        req.payment_date,
        req.bank_account_id,
        req.amount,
        req.reference
    )
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub(crate) async fn update(
    pool: &mut PgConnection,
    id: Uuid,
    req: &CreateVendorPaymentRequest,
) -> Result<VendorPayment, sqlx::Error> {
    let row = sqlx::query_as!(
        VendorPayment,
        r#"
        UPDATE vendor_payments
        SET
            partner_id = $1,
            payment_date = $2,
            paid_from_account = $3,
            amount = $4,
            reference = $5
        WHERE id = $6
        RETURNING *
        "#,
        req.partner_id,
        req.payment_date,
        req.bank_account_id,
        req.amount,
        req.reference,
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub(crate) async fn delete(pool: &mut PgConnection, id: Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM vendor_payments WHERE id = $1", id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}
