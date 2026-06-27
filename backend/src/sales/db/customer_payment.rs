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
use shared_core::sales::{
    models::customer_payment::CustomerPayment,
    requests::customer_payment::CreateCustomerPaymentRequest,
};
use uuid::Uuid;

fn from_row_to_customer_payment(row: &sqlx::postgres::PgRow) -> CustomerPayment {
    CustomerPayment {
        id: row.get("id"),
        organization_id: row.get("organization_id"),
        partner_id: row.get("partner_id"),
        transaction_id: row.get("transaction_id"),
        payment_date: row.get("payment_date"),
        deposited_to_account: row.get("deposited_to_account"),
        amount: row.get("amount"),
        reference: row.get("reference"),
        created_at: row.get("created_at"),
    }
}

pub(crate) async fn get(
    pool: &mut PgConnection,
    id: Uuid,
) -> Result<Option<CustomerPayment>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT *
        FROM customer_payments
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map(|row| row.map(|r| from_row_to_customer_payment(&r)))
}

pub(crate) async fn get_all(
    pool: &mut PgConnection,
    organization_id: Uuid,
) -> Result<Vec<CustomerPayment>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT *
        FROM customer_payments
        WHERE organization_id = $1
        ORDER BY payment_date DESC, created_at DESC
        "#,
    )
    .bind(organization_id)
    .fetch_all(pool)
    .await
    .map(|rows| rows.iter().map(from_row_to_customer_payment).collect())
}

pub(crate) async fn get_all_by_invoice(
    pool: &mut PgConnection,
    invoice_id: Uuid,
) -> Result<Vec<CustomerPayment>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT cp.*
        FROM customer_payments cp
        JOIN customer_payment_allocations cpa ON cp.id = cpa.customer_payment_id
        WHERE cpa.sales_invoice_id = $1
        ORDER BY cp.payment_date DESC, cp.created_at DESC
        "#,
    )
    .bind(invoice_id)
    .fetch_all(pool)
    .await
    .map(|rows| rows.iter().map(from_row_to_customer_payment).collect())
}

pub(crate) async fn insert(
    pool: &mut PgConnection,
    organization_id: Uuid,
    transaction_id: Uuid,
    req: &CreateCustomerPaymentRequest,
) -> Result<CustomerPayment, sqlx::Error> {
    let row = sqlx::query(
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
    )
    .bind(organization_id)
    .bind(req.partner_id)
    .bind(transaction_id)
    .bind(req.payment_date)
    .bind(req.bank_account_id)
    .bind(req.amount)
    .bind(&req.reference)
    .fetch_one(pool)
    .await?;
    Ok(from_row_to_customer_payment(&row))
}

pub(crate) async fn update(
    pool: &mut PgConnection,
    id: Uuid,
    req: &CreateCustomerPaymentRequest,
) -> Result<CustomerPayment, sqlx::Error> {
    let row = sqlx::query(
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
    )
    .bind(req.partner_id)
    .bind(req.payment_date)
    .bind(req.bank_account_id)
    .bind(req.amount)
    .bind(&req.reference)
    .bind(id)
    .fetch_one(pool)
    .await?;
    Ok(from_row_to_customer_payment(&row))
}

pub(crate) async fn delete(pool: &mut PgConnection, id: Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM customer_payments WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}
