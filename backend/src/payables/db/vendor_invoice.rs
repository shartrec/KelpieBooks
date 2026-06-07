/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use std::str::FromStr;

use chrono::NaiveDate;
use rocket_db_pools::sqlx::{
    self,
    PgConnection,
    Row,
};
use shared_core::payables::{
    dtos::{
        top_payable::TopPayable,
        vendor_invoice_list_item::VendorInvoiceListItem,
    },
    models::{
        invoice_status::InvoiceStatus,
        vendor_invoice::VendorInvoice,
        vendor_invoice_item::VendorInvoiceItem,
    },
    requests::vendor_invoice::{
        CreateVendorInvoiceRequest,
        UpdateVendorInvoiceRequest,
    },
};
use uuid::Uuid;

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
        partner_id: row.get("partner_id"),
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

fn from_row_to_top_payable(row: &sqlx::postgres::PgRow) -> TopPayable {
    TopPayable {
        partner_name: row.get("partner_name"),
        due_date: row.get("due_date"),
        amount: row.get("amount_remaining"),
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

pub(crate) async fn get(
    pool: &mut PgConnection,
    id: Uuid,
) -> Result<Option<VendorInvoice>, sqlx::Error> {
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

pub(crate) async fn get_by_org(
    pool: &mut PgConnection,
    organization_id: Uuid,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    partner_id: Option<Uuid>,
    min_amount: Option<i64>,
    status: Option<String>,
) -> Result<Vec<VendorInvoiceListItem>, sqlx::Error> {
    let mut query = String::from(
        r#"
        SELECT
            vi.id,
            vi.partner_id,
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
    "#,
    );

    let mut i = 2;
    if start_date.is_some() {
        query.push_str(&format!(" AND vi.issue_date >= ${}", i));
        i += 1;
    }
    if end_date.is_some() {
        query.push_str(&format!(" AND vi.issue_date <= ${}", i));
        i += 1;
    }
    if partner_id.is_some() {
        query.push_str(&format!(" AND vi.partner_id = ${}", i));
        i += 1;
    }
    if min_amount.is_some() {
        query.push_str(&format!(" AND vi.gross_amount >= ${}", i));
        i += 1;
    }
    if let Some(_status) = &status {
        query.push_str(&format!(" AND vi.status::TEXT = ANY(${})", i));
    }

    query.push_str(" ORDER BY vi.issue_date DESC");

    let mut query_builder = sqlx::query(&query).bind(organization_id);

    if let Some(start_date) = start_date {
        query_builder = query_builder.bind(start_date);
    }
    if let Some(end_date) = end_date {
        query_builder = query_builder.bind(end_date);
    }
    if let Some(partner_id) = partner_id {
        query_builder = query_builder.bind(partner_id);
    }
    if let Some(min_amount) = min_amount {
        query_builder = query_builder.bind(min_amount);
    }
    if let Some(status) = &status {
        let statuses: Vec<&str> = status.split(',').collect();
        query_builder = query_builder.bind(statuses);
    }

    query_builder.fetch_all(pool).await.map(|rows| {
        rows.iter()
            .map(from_row_to_vendor_invoice_list_item)
            .collect()
    })
}

pub(crate) async fn get_top_payables(
    pool: &mut PgConnection,
    organization_id: Uuid,
    due_date_before: &NaiveDate,
) -> Result<Vec<TopPayable>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT
            p.legal_name as partner_name,
            vi.due_date,
            vi.amount_remaining
        FROM vendor_invoices vi
        JOIN partners p ON vi.partner_id = p.id
        WHERE vi.organization_id = $1
          AND vi.status::TEXT IN ('Open', 'PartiallyPaid')
          AND vi.due_date <= $2
        ORDER BY vi.amount_remaining DESC
        LIMIT 5
        "#,
    )
    .bind(organization_id)
    .bind(due_date_before)
    .fetch_all(pool)
    .await
    .map(|rows| rows.iter().map(from_row_to_top_payable).collect())
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

pub(crate) async fn update_totals(
    pool: &mut PgConnection,
    id: Uuid,
    net_amount: i64,
    tax_amount: i64,
    gross_amount: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE vendor_invoices
        SET net_amount = $1, tax_amount = $2, gross_amount = $3, amount_remaining = $3
        WHERE id = $4
        "#,
    )
    .bind(net_amount)
    .bind(tax_amount)
    .bind(gross_amount)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
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
