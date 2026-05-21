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
use shared_core::dtos::vendor_invoice_list_item::VendorInvoiceListItem;
use shared_core::models::vendor_invoice::VendorInvoice;
use shared_core::models::vendor_invoice_item::VendorInvoiceItem;
use shared_core::requests::vendor_invoice::{CreateVendorInvoiceRequest, UpdateVendorInvoiceRequest};
use uuid::Uuid;
use std::str::FromStr;
use shared_core::models::invoice_status::InvoiceStatus;

fn from_row_to_vendor_invoice(row: &sqlx::postgres::PgRow) -> VendorInvoice {
    let status_str: String = row.get("status");
    let status = InvoiceStatus::from_str(&status_str).unwrap();
    VendorInvoice {
        id: row.get("id"),
        organization_id: row.get("organization_id"),
        partner_id: row.get("partner_id"),
        transaction_id: row.get("transaction_id"),
        invoice_number: row.get("invoice_number"),
        status,
        issue_date: row.get("issue_date"),
        due_date: row.get("due_date"),
        net_amount: row.get("net_amount"),
        tax_amount: row.get("tax_amount"),
        gross_amount: row.get("gross_amount"),
        amount_remaining: row.get("amount_remaining"),
        notes: row.get("notes"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
        items: vec![],
    }
}

fn from_row_to_vendor_invoice_list_item(row: &sqlx::postgres::PgRow) -> VendorInvoiceListItem {
    let status_str: String = row.get("status");
    let status = InvoiceStatus::from_str(&status_str).unwrap();
    VendorInvoiceListItem {
        id: row.get("id"),
        partner_name: row.get("partner_name"),
        invoice_number: row.get("invoice_number"),
        issue_date: row.get("issue_date"),
        due_date: row.get("due_date"),
        net_amount: row.get("net_amount"),
        tax_amount: row.get("tax_amount"),
        gross_amount: row.get("gross_amount"),
        amount_remaining: row.get("amount_remaining"),
        status,
    }
}

fn from_row_to_vendor_invoice_item(row: &sqlx::postgres::PgRow) -> VendorInvoiceItem {
    VendorInvoiceItem {
        id: row.get("id"),
        vendor_invoice_id: row.get("vendor_invoice_id"),
        account_id: row.get("account_id"),
        description: row.get("description"),
        net_amount: row.get("net_amount"),
        tax_amount: row.get("tax_amount"),
        total_amount: row.get("total_amount"),
    }
}

pub(crate) async fn get(pool: &mut PgConnection, id: Uuid) -> Result<Option<VendorInvoice>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT id, organization_id, partner_id, transaction_id, invoice_number, status::TEXT, issue_date, due_date, net_amount, tax_amount, gross_amount, amount_remaining, notes, created_at, updated_at
        FROM vendor_invoices
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map(|row| row.map(|r| from_row_to_vendor_invoice(&r)))
}

pub(crate) async fn get_items(
    pool: &mut PgConnection,
    vendor_invoice_id: Uuid,
) -> Result<Vec<VendorInvoiceItem>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT *
        FROM vendor_invoice_items
        WHERE vendor_invoice_id = $1
        "#,
    )
    .bind(vendor_invoice_id)
    .fetch_all(pool)
    .await
    .map(|rows| rows.iter().map(from_row_to_vendor_invoice_item).collect())
}

pub(crate) async fn get_all_by_org(
    pool: &mut PgConnection,
    organization_id: Uuid,
) -> Result<Vec<VendorInvoiceListItem>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT
            vi.id,
            p.legal_name as partner_name,
            vi.invoice_number,
            vi.issue_date,
            vi.due_date,
            vi.net_amount,
            vi.tax_amount,
            vi.gross_amount,
            vi.amount_remaining,
            vi.status::TEXT
        FROM vendor_invoices vi
        JOIN partners p ON vi.partner_id = p.id
        WHERE vi.organization_id = $1
        ORDER BY vi.issue_date DESC
        "#,
    )
    .bind(organization_id)
    .fetch_all(pool)
    .await
    .map(|rows| rows.iter().map(from_row_to_vendor_invoice_list_item).collect())
}

pub(crate) async fn insert(
    pool: &mut PgConnection,
    org_id: Uuid,
    transaction_id: Uuid,
    req: &CreateVendorInvoiceRequest,
) -> Result<VendorInvoice, sqlx::Error> {
    let net_amount = req.items.iter().map(|i| i.net_amount).sum::<i64>();
    let tax_amount = req.items.iter().map(|i| i.tax_amount).sum::<i64>();
    let gross_amount = net_amount + tax_amount;
    let row = sqlx::query(
        r#"
        INSERT INTO vendor_invoices (organization_id, partner_id, transaction_id, invoice_number, issue_date, due_date, net_amount, tax_amount, gross_amount, amount_remaining, notes)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING id, organization_id, partner_id, transaction_id, invoice_number, status::TEXT, issue_date, due_date, net_amount, tax_amount, gross_amount, amount_remaining, notes, created_at, updated_at
        "#,
    )
    .bind(org_id)
    .bind(req.partner_id)
    .bind(transaction_id)
    .bind(&req.invoice_number)
    .bind(req.issue_date)
    .bind(req.due_date)
    .bind(net_amount)
    .bind(tax_amount)
    .bind(gross_amount)
    .bind(gross_amount) // amount_remaining is the same as gross_amount on creation
    .bind(&req.notes)
    .fetch_one(pool)
    .await?;
    Ok(from_row_to_vendor_invoice(&row))
}

pub(crate) async fn update(
    pool: &mut PgConnection,
    id: Uuid,
    req: &UpdateVendorInvoiceRequest,
) -> Result<VendorInvoice, sqlx::Error> {
    let row = sqlx::query(
        r#"
        UPDATE vendor_invoices
        SET invoice_number = $1, issue_date = $2, due_date = $3, notes = $4
        WHERE id = $5
        RETURNING id, organization_id, partner_id, transaction_id, invoice_number, status::TEXT, issue_date, due_date, net_amount, tax_amount, gross_amount, amount_remaining, notes, created_at, updated_at
        "#,
    )
    .bind(&req.invoice_number)
    .bind(req.issue_date)
    .bind(req.due_date)
    .bind(&req.notes)
    .bind(id)
    .fetch_one(pool)
    .await?;
    Ok(from_row_to_vendor_invoice(&row))
}

pub(crate) async fn update_amount_remaining(
    pool: &mut PgConnection,
    id: Uuid,
    amount: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE vendor_invoices
        SET amount_remaining = amount_remaining + $1
        WHERE id = $2
        "#,
    )
    .bind(amount)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn insert_item(
    pool: &mut PgConnection,
    vendor_invoice_id: Uuid,
    item: &VendorInvoiceItem,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO vendor_invoice_items (id, vendor_invoice_id, account_id, description, net_amount, tax_amount, total_amount)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(item.id)
    .bind(vendor_invoice_id)
    .bind(item.account_id)
    .bind(&item.description)
    .bind(item.net_amount)
    .bind(item.tax_amount)
    .bind(item.total_amount)
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn delete_items(
    pool: &mut PgConnection,
    vendor_invoice_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM vendor_invoice_items
        WHERE vendor_invoice_id = $1
        "#,
    )
    .bind(vendor_invoice_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn is_duplicate(
    pool: &mut PgConnection,
    organization_id: Uuid,
    partner_id: Uuid,
    invoice_number: &str,
) -> Result<bool, sqlx::Error> {
    let count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM vendor_invoices
        WHERE organization_id = $1 AND partner_id = $2 AND invoice_number = $3
        "#,
    )
    .bind(organization_id)
    .bind(partner_id)
    .bind(invoice_number)
    .fetch_one(pool)
    .await?;
    Ok(count > 0)
}
