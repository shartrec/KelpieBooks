/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::NaiveDate;
use rocket_db_pools::sqlx::{self, PgConnection, Row};
use rust_decimal::prelude::Zero;
use rust_decimal::Decimal;
use shared_core::sales::{
    dtos::sales_invoice_list_item::SalesInvoiceListItem,
    models::{
        invoice_address::InvoiceAddress, invoice_status::InvoiceStatus,
        sales_invoice::SalesInvoice, sales_invoice_item::SalesInvoiceItem,
    },
};
use uuid::Uuid;

fn from_row_to_sales_invoice(row: &sqlx::postgres::PgRow) -> SalesInvoice {
    let baddr = InvoiceAddress {
        name: row.get("bill_to_name"),
        attention: row.get("bill_to_attention"),
        address_line1: row.get("bill_to_line1"),
        address_line2: row.get("bill_to_line2"),
        city: row.get("bill_to_city"),
        state_province: row.get("bill_to_region"),
        postal_code: row.get("bill_to_postal_code"),
        country: row.get("bill_to_country"),
    };
    let saddr = InvoiceAddress {
        name: row.get("ship_to_name"),
        attention: row.get("ship_to_attention"),
        address_line1: row.get("ship_to_line1"),
        address_line2: row.get("ship_to_line2"),
        city: row.get("ship_to_city"),
        state_province: row.get("ship_to_region"),
        postal_code: row.get("ship_to_postal_code"),
        country: row.get("ship_to_country"),
    };

    SalesInvoice {
        id: row.get("id"),
        org_id: row.get("organization_id"),
        partner_id: row.get("partner_id"),
        invoice_number: row.get("invoice_number"),
        issue_date: row.get("issue_date"),
        amount_due: row.get("amount_due"),
        due_date: row.get("due_date"),
        status: row.get("status"),
        billing_address_id: row.get("billing_address_id"),
        shipping_address_id: row.get("shipping_address_id"),
        bill_to: baddr,
        ship_to: saddr,

        subtotal: row.get("subtotal"),
        tax_total: row.get("tax_total"),
        total_amount: row.get("total_amount"),
        lines: vec![], // Lines are fetched separately
    }
}

fn from_row_to_sales_invoice_line(row: &sqlx::postgres::PgRow) -> SalesInvoiceItem {
    SalesInvoiceItem {
        id: row.get("id"),
        invoice_id: row.get("invoice_id"),
        item_id: row.get("item_id"),
        name: row.get("name"),
        description: row.get("description"),
        quantity: row.get("quantity"),
        unit_price: row.get("unit_price"),
        tax_category_id: row.get("tax_category_id"),
        tax_amount: row.get("tax_amount"),
        tax_rate: Decimal::ZERO,
        net_amount: row.get("net_amount"),
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
    // New optional address references
    billing_address_id: Option<Uuid>,
    shipping_address_id: Option<Uuid>,
    // Snapshot fields (all optional text)
    bill_to_name: Option<&str>,
    bill_to_attention: Option<&str>,
    bill_to_line1: Option<&str>,
    bill_to_line2: Option<&str>,
    bill_to_city: Option<&str>,
    bill_to_region: Option<&str>,
    bill_to_postal_code: Option<&str>,
    bill_to_country: Option<&str>,
    ship_to_name: Option<&str>,
    ship_to_attention: Option<&str>,
    ship_to_line1: Option<&str>,
    ship_to_line2: Option<&str>,
    ship_to_city: Option<&str>,
    ship_to_region: Option<&str>,
    ship_to_postal_code: Option<&str>,
    ship_to_country: Option<&str>,
) -> Result<SalesInvoice, sqlx::Error> {
    let row = sqlx::query(
        r#"
        INSERT INTO sales_invoices (
            organization_id, partner_id, invoice_number, issue_date, due_date, status,
            billing_address_id, shipping_address_id,
            bill_to_name, bill_to_attention, bill_to_line1, bill_to_line2, bill_to_city, bill_to_region, bill_to_postal_code, bill_to_country,
            ship_to_name, ship_to_attention, ship_to_line1, ship_to_line2, ship_to_city, ship_to_region, ship_to_postal_code, ship_to_country,
            subtotal, tax_total, total_amount, amount_due
        )
        VALUES (
            $1, $2, $3, $4, $5, $6,
            $7, $8,
            $9, $10, $11, $12, $13, $14, $15, $16,
            $17, $18, $19, $20, $21, $22, $23, $24,
            $25, $26, $27, $28
        )
        RETURNING id, organization_id, partner_id, invoice_number, issue_date, due_date, status,
            billing_address_id, shipping_address_id,
            bill_to_name, bill_to_attention, bill_to_line1, bill_to_line2, bill_to_city, bill_to_region, bill_to_postal_code, bill_to_country,
            ship_to_name, ship_to_attention, ship_to_line1, ship_to_line2, ship_to_city, ship_to_region, ship_to_postal_code, ship_to_country,
            subtotal, tax_total, total_amount, amount_due
        "#,
    )
    .bind(org_id)
    .bind(partner_id)
    .bind(invoice_number)
    .bind(issue_date)
    .bind(due_date)
    .bind(InvoiceStatus::Draft)
    .bind(billing_address_id)
    .bind(shipping_address_id)
    .bind(bill_to_name)
    .bind(bill_to_attention)
    .bind(bill_to_line1)
    .bind(bill_to_line2)
    .bind(bill_to_city)
    .bind(bill_to_region)
    .bind(bill_to_postal_code)
    .bind(bill_to_country)
    .bind(ship_to_name)
    .bind(ship_to_attention)
    .bind(ship_to_line1)
    .bind(ship_to_line2)
    .bind(ship_to_city)
    .bind(ship_to_region)
    .bind(ship_to_postal_code)
    .bind(ship_to_country)
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
    inv_id: Uuid,
    org_id: Uuid,
    line: &SalesInvoiceItem,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO sales_invoice_lines (organization_id, invoice_id, item_id, description, quantity, unit_price, tax_category_id, tax_amount, net_amount, sort_order)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        "#,
    )
    .bind(org_id)
    .bind(inv_id)
    .bind(line.item_id)
    .bind(&line.description)
    .bind(line.quantity)
    .bind(line.unit_price)
    .bind(line.tax_category_id)
    .bind(line.tax_amount)
    .bind(line.net_amount)
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

pub(crate) async fn get(
    pool: &mut PgConnection,
    id: Uuid,
    org_id: Uuid,
) -> Result<Option<SalesInvoice>, sqlx::Error> {
    let invoice_row = sqlx::query(
        r#"
        SELECT id, organization_id, partner_id, invoice_number, issue_date, due_date, status,
            billing_address_id, shipping_address_id,
            bill_to_name, bill_to_attention, bill_to_line1, bill_to_line2, bill_to_city, bill_to_region, bill_to_postal_code, bill_to_country,
            ship_to_name, ship_to_attention, ship_to_line1, ship_to_line2, ship_to_city, ship_to_region, ship_to_postal_code, ship_to_country,
            subtotal, tax_total, total_amount, amount_due
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
        Ok(Some(sales_invoice))
    } else {
        Ok(None)
    }
}

pub(crate) async fn get_sales_invoice_with_lines(
    pool: &mut PgConnection,
    id: Uuid,
    org_id: Uuid,
) -> Result<Option<SalesInvoice>, sqlx::Error> {
    let invoice_row = sqlx::query(
        r#"
        SELECT id, organization_id, partner_id, invoice_number, issue_date, due_date, status,
            billing_address_id, shipping_address_id,
            bill_to_name, bill_to_attention, bill_to_line1, bill_to_line2, bill_to_city, bill_to_region, bill_to_postal_code, bill_to_country,
            ship_to_name, ship_to_attention, ship_to_line1, ship_to_line2, ship_to_city, ship_to_region, ship_to_postal_code, ship_to_country,
            subtotal, tax_total, total_amount, amount_due
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
            SELECT sil.id, invoice_id, item_id, it.name, sil.description, quantity, sil.unit_price, sil.tax_category_id, tax_amount, net_amount, sort_order
            FROM sales_invoice_lines sil, items it
            WHERE sil.item_id = it.id
                AND invoice_id = $1
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
    amount_due: Decimal,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE sales_invoices
        SET subtotal = $1, tax_total = $2, total_amount = $3, amount_due = $3
        WHERE id = $4
        "#,
    )
    .bind(net_amount)
    .bind(tax_amount)
    .bind(gross_amount)
    .bind(amount_due)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

fn from_row_to_sales_invoice_list_item(row: &sqlx::postgres::PgRow) -> SalesInvoiceListItem {
    SalesInvoiceListItem {
        id: row.get("id"),
        partner_id: row.get("partner_id"),
        partner_name: row.get("partner_name"),
        invoice_number: row.get("invoice_number"),
        status: row.get("status"),
        issue_date: row.get("issue_date"),
        due_date: row.get("due_date"),
        // Map schema names to DTO expectations
        net_amount: row.get("subtotal"),
        tax_amount: row.get("tax_total"),
        gross_amount: row.get("total_amount"),
    }
}

pub(crate) async fn update_sales_invoice(
    executor: &mut sqlx::PgConnection,
    org_id: Uuid,
    req: &shared_core::sales::requests::sales_invoice::UpdateSalesInvoiceRequest,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE sales_invoices
        SET
            issue_date = $1,
            due_date = $2,
            billing_address_id = $3,
            shipping_address_id = $4,
            bill_to_name = $5,
            bill_to_attention = $6,
            bill_to_line1 = $7,
            bill_to_line2 = $8,
            bill_to_city = $9,
            bill_to_region = $10,
            bill_to_postal_code = $11,
            bill_to_country = $12,
            ship_to_name = $13,
            ship_to_attention = $14,
            ship_to_line1 = $15,
            ship_to_line2 = $16,
            ship_to_city = $17,
            ship_to_region = $18,
            ship_to_postal_code = $19,
            ship_to_country = $20
        WHERE id = $21 AND organization_id = $22
        "#,
    )
    .bind(req.issue_date)
    .bind(req.due_date)
    .bind(req.billing_address_id)
    .bind(req.shipping_address_id)
    .bind(&req.bill_to.name)
    .bind(&req.bill_to.attention)
    .bind(&req.bill_to.address_line1)
    .bind(&req.bill_to.address_line2)
    .bind(&req.bill_to.city)
    .bind(&req.bill_to.state_province)
    .bind(&req.bill_to.postal_code)
    .bind(&req.bill_to.country)
    .bind(&req.ship_to.name)
    .bind(&req.ship_to.attention)
    .bind(&req.ship_to.address_line1)
    .bind(&req.ship_to.address_line2)
    .bind(&req.ship_to.city)
    .bind(&req.ship_to.state_province)
    .bind(&req.ship_to.postal_code)
    .bind(&req.ship_to.country)
    .bind(req.id)
    .bind(org_id)
    .execute(executor)
    .await?;

    Ok(())
}

pub(crate) async fn list_sales_invoices(
    pool: &mut PgConnection,
    org_id: Uuid,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    partner_id: Option<Uuid>,
    min_amount: Option<Decimal>,
    statuses: Option<Vec<InvoiceStatus>>,
) -> Result<Vec<SalesInvoiceListItem>, sqlx::Error> {
    // Build dynamic WHERE clause
    let mut conditions: Vec<String> = vec!["si.organization_id = $1".to_string()];

    // We'll keep a parallel Vec of bind closures is not possible; instead compute indices manually
    // indices start after $1 (org_id)
    let mut idx = 2;

    if start_date.is_some() {
        conditions.push(format!("si.issue_date >= ${}", idx));
        idx += 1;
    }
    if end_date.is_some() {
        conditions.push(format!("si.issue_date <= ${}", idx));
        idx += 1;
    }
    if partner_id.is_some() {
        conditions.push(format!("si.partner_id = ${}", idx));
        idx += 1;
    }
    if min_amount.is_some() {
        conditions.push(format!("si.total_amount >= ${}", idx));
        idx += 1;
    }
    if let Some(sts) = statuses.as_ref() {
        if !sts.is_empty() {
            let or_clauses: Vec<String> = (0..sts.len())
                .map(|_| {
                    let clause = format!("si.status = ${}", idx);
                    idx += 1;
                    clause
                })
                .collect();
            conditions.push(format!("({})", or_clauses.join(" OR ")));
        }
    }

    let where_sql = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    let base_sql = format!(
        r#"
        SELECT
            si.id,
            si.partner_id,
            p.legal_name AS partner_name,
            si.invoice_number,
            si.status,
            si.issue_date,
            si.due_date,
            si.subtotal,
            si.tax_total,
            si.total_amount,
            si.amount_due
        FROM sales_invoices si
        JOIN partners p ON p.id = si.partner_id
        {}
        ORDER BY si.issue_date DESC, si.invoice_number DESC
        "#,
        where_sql
    );

    let mut query = sqlx::query(&base_sql).bind(org_id);
    // Bind params in the same order as added
    if let Some(sd) = start_date {
        query = query.bind(sd);
    }
    if let Some(ed) = end_date {
        query = query.bind(ed);
    }
    if let Some(pid) = partner_id {
        query = query.bind(pid);
    }
    if let Some(mina) = min_amount {
        query = query.bind(mina)
    }
    if let Some(sts) = statuses.as_ref() {
        if !sts.is_empty() {
            for status in sts {
                query = query.bind(status);
            }
        }
    }

    let rows = query.fetch_all(&mut *pool).await?;
    Ok(rows
        .iter()
        .map(from_row_to_sales_invoice_list_item)
        .collect())
}

pub(crate) async fn update_amount_remaining(
    pool: &mut PgConnection,
    id: Uuid,
    amount: Decimal,
) -> Result<(), sqlx::Error> {
    let mut query = if amount == Decimal::zero() {
        sqlx::query(
            r#"
        UPDATE sales_invoices
        SET amount_due = amount_due + $1, status = $2
        WHERE id = $2
        "#,
        )
    } else {
        sqlx::query(
            r#"
        UPDATE sales_invoices
        SET amount_due = amount_due + $1
        WHERE id = $2
        "#,
        )
    };

    query = query.bind(amount).bind(id);
    if amount == Decimal::zero() {
        query = query.bind(InvoiceStatus::Paid);
    }
    query.execute(pool).await?;
    Ok(())
}
