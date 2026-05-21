/*
 * Copyright (c) 2026-2026. Trevor Campbell and others.
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
use shared_core::models::vendor_invoice_payment::VendorInvoicePayment;
use shared_core::requests::vendor_invoice_payment::CreateVendorInvoicePaymentRequest;
use uuid::Uuid;

fn from_row_to_vendor_invoice_payment(row: &sqlx::postgres::PgRow) -> VendorInvoicePayment {
    VendorInvoicePayment {
        id: row.get("id"),
        organization_id: row.get("organization_id"),
        vendor_invoice_id: row.get("vendor_invoice_id"),
        transaction_id: row.get("transaction_id"),
        payment_date: row.get("payment_date"),
        amount: row.get("amount"),
        reference: row.get("reference"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

pub(crate) async fn get_all_by_invoice(
    pool: &mut PgConnection,
    vendor_invoice_id: Uuid,
) -> Result<Vec<VendorInvoicePayment>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT *
        FROM vendor_invoice_payments
        WHERE vendor_invoice_id = $1
        ORDER BY payment_date DESC
        "#,
    )
    .bind(vendor_invoice_id)
    .fetch_all(pool)
    .await
    .map(|rows| rows.iter().map(from_row_to_vendor_invoice_payment).collect())
}

pub(crate) async fn insert(
    pool: &mut PgConnection,
    org_id: Uuid,
    transaction_id: Uuid,
    req: &CreateVendorInvoicePaymentRequest,
) -> Result<VendorInvoicePayment, sqlx::Error> {
    let row = sqlx::query(
        r#"
        INSERT INTO vendor_invoice_payments (organization_id, vendor_invoice_id, transaction_id, payment_date, amount, reference)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
    )
    .bind(org_id)
    .bind(req.vendor_invoice_id)
    .bind(transaction_id)
    .bind(req.payment_date)
    .bind(req.amount)
    .bind(&req.reference)
    .fetch_one(pool)
    .await?;
    Ok(from_row_to_vendor_invoice_payment(&row))
}
