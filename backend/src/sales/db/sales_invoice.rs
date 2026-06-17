/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::NaiveDate;
use rocket_db_pools::sqlx::{
    self,
    PgConnection,
    Row,
};
use rust_decimal::Decimal;
use shared_core::sales::models::{
    invoice_status::InvoiceStatus,
    sales_invoice::SalesInvoice,
    sales_invoice_item::SalesInvoiceLine,
};
use uuid::Uuid;

fn from_row_to_sales_invoice(row: &sqlx::postgres::PgRow) -> SalesInvoice {
    SalesInvoice {
        id: row.get("id"),
        org_id: row.get("org_id"),
        partner_id: row.get("partner_id"),
        invoice_number: row.get("invoice_number"),
        issue_date: row.get("issue_date"),
        due_date: row.get("due_date"),
        status: row.get("status"),
        subtotal: row.get("subtotal"),
        tax_total: row.get("tax_total"),
        total_amount: row.get("total_amount"),
        lines: vec![], // Lines are fetched separately
    }
}

fn from_row_to_sales_invoice_line(row: &sqlx::postgres::PgRow) -> SalesInvoiceLine {
    SalesInvoiceLine {
        id: row.get("id"),
        invoice_id: row.get("invoice_id"),
        item_id: row.get("item_id"),
        description: row.get("description"),
        quantity: row.get("quantity"),
        unit_price: row.get("unit_price"),
        tax_category_id: row.get("tax_rate_id"),
        tax_amount: row.get("tax_amount"),
        line_total: row.get("line_total"),
        sort_order: row.get("sort_order"),
    }
}

pub(crate) async fn create_draft_invoice(
    pool: &mut PgConnection,
    org_id: Uuid,
    partner_id: Uuid,
    invoice_number: &str,
    issue_date: NaiveDate,
    due_date: NaiveDate,
) -> Result<SalesInvoice, sqlx::Error> {
    let row = sqlx::query(
        r#"
        INSERT INTO sales_invoices (organization_id, partner_id, invoice_number, issue_date, due_date, status, subtotal, tax_total, total_amount, amount_due)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING id, organization_id, partner_id, invoice_number, issue_date, due_date, status, subtotal, total_amount, amount_due
        "#,
    )
    .bind(org_id)
    .bind(partner_id)
    .bind(invoice_number)
    .bind(issue_date)
    .bind(due_date)
    .bind(InvoiceStatus::Draft)
    .bind(Decimal::ZERO)
    .bind(Decimal::ZERO)
    .bind(Decimal::ZERO)
    .bind(Decimal::ZERO)
    .fetch_one(pool)
    .await?;

    Ok(from_row_to_sales_invoice(&row))
}

pub(crate) async fn insert_sales_invoice_line(
    pool: &mut PgConnection,
    org_id: Uuid,
    line: &SalesInvoiceLine,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO sales_invoice_lines (organization_id, invoice_id, item_id, description, quantity, unit_price, tax_category_id, tax_amount, line_total, sort_order)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        "#,
    )
    .bind(org_id)
    .bind(line.invoice_id)
    .bind(line.item_id)
    .bind(&line.description)
    .bind(line.quantity)
    .bind(line.unit_price)
    .bind(line.tax_category_id)
    .bind(line.tax_amount)
    .bind(line.line_total)
    .bind(line.sort_order)
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn delete_sales_invoice_lines(
    pool: &mut PgConnection,
    invoice_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM sales_invoice_lines
        WHERE invoice_id = $1
        "#,
    )
    .bind(invoice_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn get_sales_invoice_with_lines(
    pool: &mut PgConnection,
    id: Uuid,
    org_id: Uuid,
) -> Result<Option<SalesInvoice>, sqlx::Error> {
    let invoice_row = sqlx::query(
        r#"
        SELECT id, organization_id, partner_id, invoice_number, issue_date, due_date, status, subtotal, tax_total, total_amount, amount_due
        FROM sales_invoices
        WHERE id = $1 AND organization_id = $2
        "#,
    )
    .bind(id)
    .bind(org_id)
    .fetch_optional(&mut *pool)
    .await?;

    if let Some(invoice_row) = invoice_row {
        let mut sales_invoice = from_row_to_sales_invoice(&invoice_row);

        let line_rows = sqlx::query(
            r#"
            SELECT id, invoice_id, item_id, description, quantity, unit_price, tax_category_id, tax_amount, line_total, sort_order
            FROM sales_invoice_lines
            WHERE invoice_id = $1
            "#,
        )
        .bind(id)
        .fetch_all(&mut *pool)
        .await?;

        sales_invoice.lines = line_rows
            .iter()
            .map(from_row_to_sales_invoice_line)
            .collect();

        Ok(Some(sales_invoice))
    } else {
        Ok(None)
    }
}

pub(crate) async fn update_sales_invoice_totals(
    pool: &mut PgConnection,
    id: Uuid,
    net_amount: Decimal,
    tax_amount: Decimal,
    gross_amount: Decimal,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE sales_invoices
        SET subtotal = $1, tax_total = $2, total_amount = $3
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
