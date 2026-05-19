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

use chrono::NaiveDate;
use rocket_db_pools::sqlx::{self, PgConnection, Row};
use shared_core::models::invoice_status::InvoiceStatus;
use shared_core::models::vendor_invoice::VendorInvoice;
use uuid::Uuid;

fn from_row_to_vendor_invoice(row: &sqlx::postgres::PgRow) -> VendorInvoice {
    let status_str: String = row.get("status");
    let status = InvoiceStatus::try_from(status_str.as_str())
        .expect("DB schema and InvoiceStatus enum are out of sync!");

    VendorInvoice {
        id: row.get("id"),
        organization_id: row.get("organization_id"),
        partner_id: row.get("partner_id"),
        transaction_id: row.get("transaction_id"),
        invoice_number: row.get("invoice_number"),
        status,
        issue_date: row.get("issue_date"),
        due_date: row.get("due_date"),
        amount_due: row.get("amount_due"),
        amount_remaining: row.get("amount_remaining"),
        notes: row.get("notes"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

pub(crate) async fn get(
    pool: &mut PgConnection,
    id: Uuid,
) -> Result<Option<VendorInvoice>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT
            id,
            organization_id,
            partner_id,
            transaction_id,
            invoice_number,
            status::TEXT as status,
            issue_date,
            due_date,
            amount_due,
            amount_remaining,
            notes,
            created_at,
            updated_at
        FROM vendor_invoices
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map(|row| row.map(|r| from_row_to_vendor_invoice(&r)))
}

pub(crate) async fn get_all(
    pool: &mut PgConnection,
    organization_id: Uuid,
) -> Result<Vec<VendorInvoice>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT
            id,
            organization_id,
            partner_id,
            transaction_id,
            invoice_number,
            status::TEXT as status,
            issue_date,
            due_date,
            amount_due,
            amount_remaining,
            notes,
            created_at,
            updated_at
        FROM vendor_invoices
        WHERE organization_id = $1
        ORDER BY issue_date DESC, created_at DESC
        "#,
    )
    .bind(organization_id)
    .fetch_all(pool)
    .await
    .map(|rows| rows.iter().map(from_row_to_vendor_invoice).collect())
}

pub(crate) async fn insert(
    pool: &mut PgConnection,
    organization_id: Uuid,
    partner_id: Uuid,
    transaction_id: Option<Uuid>,
    invoice_number: String,
    status: InvoiceStatus,
    issue_date: NaiveDate,
    due_date: NaiveDate,
    amount_due: i64,
    amount_remaining: i64,
    notes: Option<String>,
) -> Result<VendorInvoice, sqlx::Error> {
    let row = sqlx::query(
        r#"
        INSERT INTO vendor_invoices (
            organization_id,
            partner_id,
            transaction_id,
            invoice_number,
            status,
            issue_date,
            due_date,
            amount_due,
            amount_remaining,
            notes
        )
        VALUES ($1, $2, $3, $4, $5::invoice_status, $6, $7, $8, $9, $10)
        RETURNING
            id,
            organization_id,
            partner_id,
            transaction_id,
            invoice_number,
            status::TEXT as status,
            issue_date,
            due_date,
            amount_due,
            amount_remaining,
            notes,
            created_at,
            updated_at
        "#,
    )
    .bind(organization_id)
    .bind(partner_id)
    .bind(transaction_id)
    .bind(invoice_number)
    .bind(status.as_str())
    .bind(issue_date)
    .bind(due_date)
    .bind(amount_due)
    .bind(amount_remaining)
    .bind(notes)
    .fetch_one(pool)
    .await?;
    Ok(from_row_to_vendor_invoice(&row))
}

pub(crate) async fn update(
    pool: &mut PgConnection,
    id: Uuid,
    partner_id: Uuid,
    transaction_id: Option<Uuid>,
    invoice_number: String,
    status: InvoiceStatus,
    issue_date: NaiveDate,
    due_date: NaiveDate,
    amount_due: i64,
    amount_remaining: i64,
    notes: Option<String>,
) -> Result<VendorInvoice, sqlx::Error> {
    let row = sqlx::query(
        r#"
        UPDATE vendor_invoices
        SET
            partner_id = $1,
            transaction_id = $2,
            invoice_number = $3,
            status = $4::invoice_status,
            issue_date = $5,
            due_date = $6,
            amount_due = $7,
            amount_remaining = $8,
            notes = $9,
            updated_at = NOW()
        WHERE id = $10
        RETURNING
            id,
            organization_id,
            partner_id,
            transaction_id,
            invoice_number,
            status::TEXT as status,
            issue_date,
            due_date,
            amount_due,
            amount_remaining,
            notes,
            created_at,
            updated_at
        "#,
    )
    .bind(partner_id)
    .bind(transaction_id)
    .bind(invoice_number)
    .bind(status.as_str())
    .bind(issue_date)
    .bind(due_date)
    .bind(amount_due)
    .bind(amount_remaining)
    .bind(notes)
    .bind(id)
    .fetch_one(pool)
    .await?;
    Ok(from_row_to_vendor_invoice(&row))
}

pub(crate) async fn delete(pool: &mut PgConnection, id: Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM vendor_invoices WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}
