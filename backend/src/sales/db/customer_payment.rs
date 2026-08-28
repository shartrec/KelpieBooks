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
use shared_core::sales::{
    models::customer_payment::CustomerPayment,
    requests::customer_payment::CreateCustomerPaymentRequest,
};
use uuid::Uuid;
use shared_core::{OrgId, TransactionId};

pub(crate) async fn get(
    pool: &mut PgConnection,
    id: Uuid,
) -> Result<Option<CustomerPayment>, sqlx::Error> {
    sqlx::query_as!(
        CustomerPayment,
        r#"
        SELECT *
        FROM customer_payments
        WHERE id = $1
        "#,
        id,
    )
    .fetch_optional(pool)
    .await
}

pub(crate) async fn get_all_by_order(
    pool: &mut PgConnection,
    order_id: Uuid,
) -> Result<Vec<CustomerPayment>, sqlx::Error> {
    sqlx::query_as!(
        CustomerPayment,
        r#"
        SELECT cp.*
        FROM customer_payments cp
        JOIN customer_payment_allocations cpa ON cp.id = cpa.customer_payment_id
        WHERE cpa.sales_order_id = $1
        ORDER BY cp.payment_date DESC, cp.created_at DESC
        "#,
        order_id,
    )
    .fetch_all(pool)
    .await
}

pub(crate) async fn insert(
    pool: &mut PgConnection,
    org_id: OrgId,
    transaction_id: TransactionId,
    req: &CreateCustomerPaymentRequest,
) -> Result<CustomerPayment, sqlx::Error> {
    let row = sqlx::query_as!(
        CustomerPayment,
        r#"
        INSERT INTO customer_payments (
            organization_id,
            partner_id,
            transaction_id,
            payment_date,
            deposited_to_account,
            amount,
            reference
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#,
        *org_id,
        req.partner_id,
        *transaction_id,
        req.payment_date,
        *req.bank_account_id,
        req.amount,
        req.reference,
    )
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub(crate) async fn update(
    pool: &mut PgConnection,
    id: Uuid,
    req: &CreateCustomerPaymentRequest,
) -> Result<CustomerPayment, sqlx::Error> {
    let row = sqlx::query_as!(
        CustomerPayment,
        r#"
        UPDATE customer_payments
        SET
            partner_id = $1,
            payment_date = $2,
            deposited_to_account = $3,
            amount = $4,
            reference = $5
        WHERE id = $6
        RETURNING *
        "#,
        req.partner_id,
        req.payment_date,
        *req.bank_account_id,
        req.amount,
        req.reference,
        id,
    )
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub(crate) async fn delete(pool: &mut PgConnection, id: Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM customer_payments WHERE id = $1", id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}
