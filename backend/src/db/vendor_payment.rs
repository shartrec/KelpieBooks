/*
 * Copyright (c) 2026. Trevor Campbell and others.
 *
 * This file is part of KelpieBooks.
 *
 * KelpieBooks is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License,or
 * (at your option) any later version.
 *
 * KelpieBooks is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with KelpieBooks; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 *
 * Contributors:
 *      Trevor Campbell
 *
 */

use rocket_db_pools::sqlx::{self, PgConnection, Row};
use shared_core::models::vendor_payment::VendorPayment;
use shared_core::requests::vendor_payment::CreateVendorPaymentRequest;
use uuid::Uuid;

fn from_row_to_vendor_payment(row: &sqlx::postgres::PgRow) -> VendorPayment {
    VendorPayment {
        id: row.get("id"),
        organization_id: row.get("organization_id"),
        partner_id: row.get("partner_id"),
        transaction_id: row.get("transaction_id"),
        payment_date: row.get("payment_date"),
        paid_from_account: row.get("paid_from_account"),
        amount: row.get("amount"),
        reference: row.get("reference"),
        created_at: row.get("created_at"),
    }
}

pub(crate) async fn get(
    pool: &mut PgConnection,
    id: Uuid,
) -> Result<Option<VendorPayment>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT *
        FROM vendor_payments
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map(|row| row.map(|r| from_row_to_vendor_payment(&r)))
}

pub(crate) async fn get_all(
    pool: &mut PgConnection,
    organization_id: Uuid,
) -> Result<Vec<VendorPayment>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT *
        FROM vendor_payments
        WHERE organization_id = $1
        ORDER BY payment_date DESC, created_at DESC
        "#,
    )
    .bind(organization_id)
    .fetch_all(pool)
    .await
    .map(|rows| rows.iter().map(from_row_to_vendor_payment).collect())
}

pub(crate) async fn get_all_by_invoice(
    pool: &mut PgConnection,
    invoice_id: Uuid,
) -> Result<Vec<VendorPayment>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT vp.*
        FROM vendor_payments vp
        JOIN vendor_payment_allocations vpa ON vp.id = vpa.vendor_payment_id
        WHERE vpa.vendor_invoice_id = $1
        ORDER BY vp.payment_date DESC, vp.created_at DESC
        "#,
    )
    .bind(invoice_id)
    .fetch_all(pool)
    .await
    .map(|rows| rows.iter().map(from_row_to_vendor_payment).collect())
}

pub(crate) async fn insert(
    pool: &mut PgConnection,
    organization_id: Uuid,
    transaction_id: Uuid,
    req: &CreateVendorPaymentRequest,
) -> Result<VendorPayment, sqlx::Error> {
    let row = sqlx::query(
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
    Ok(from_row_to_vendor_payment(&row))
}

pub(crate) async fn update(
    pool: &mut PgConnection,
    id: Uuid,
    req: &CreateVendorPaymentRequest,
) -> Result<VendorPayment, sqlx::Error> {
    let row = sqlx::query(
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
    )
    .bind(req.partner_id)
    .bind(req.payment_date)
    .bind(req.bank_account_id)
    .bind(req.amount)
    .bind(&req.reference)
    .bind(id)
    .fetch_one(pool)
    .await?;
    Ok(from_row_to_vendor_payment(&row))
}

pub(crate) async fn delete(pool: &mut PgConnection, id: Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM vendor_payments WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}
