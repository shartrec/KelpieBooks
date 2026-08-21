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
};
use rust_decimal::Decimal;
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

pub(crate) async fn get(
    pool: &mut PgConnection,
    id: Uuid,
) -> Result<Option<VendorInvoice>, sqlx::Error> {
    sqlx::query_as!(
        VendorInvoice,
        r#"
        SELECT id, organization_id, partner_id, transaction_id, invoice_number, status as "status: InvoiceStatus", issue_date,
               due_date, net_amount, tax_amount, gross_amount, amount_remaining, notes, created_at, updated_at
        FROM vendor_invoices
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

pub(crate) async fn get_items(
    pool: &mut PgConnection,
    vendor_invoice_id: Uuid,
) -> Result<Vec<VendorInvoiceItem>, sqlx::Error> {
    sqlx::query_as!(
        VendorInvoiceItem,
        r#"
        SELECT *
        FROM vendor_invoice_items
        WHERE vendor_invoice_id = $1
        "#,
        vendor_invoice_id
    )
    .fetch_all(pool)
    .await
}

pub(crate) async fn get_by_org(
    pool: &mut PgConnection,
    organization_id: Uuid,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    partner_id: Option<Uuid>,
    min_amount: Option<Decimal>,
    statuses: Vec<&InvoiceStatus>,
) -> Result<Vec<VendorInvoiceListItem>, sqlx::Error> {
    let mut query = sqlx::QueryBuilder::<sqlx::Postgres>::new(
        r#"
        SELECT
            vi.id,
            vi.partner_id,
            p.legal_name AS partner_name,
            vi.invoice_number,
            vi.issue_date,
            vi.due_date,
            vi.net_amount,
            vi.tax_amount,
            vi.gross_amount,
            vi.amount_remaining,
            vi.status
        FROM vendor_invoices vi
        JOIN partners p ON vi.partner_id = p.id
        WHERE vi.organization_id =
        "#,
    );

    query.push_bind(organization_id);

    if let Some(start_date) = start_date {
        query.push(" AND vi.issue_date >= ").push_bind(start_date);
    }

    if let Some(end_date) = end_date {
        query.push(" AND vi.issue_date <= ").push_bind(end_date);
    }

    if let Some(partner_id) = partner_id {
        query.push(" AND vi.partner_id = ").push_bind(partner_id);
    }

    if let Some(min_amount) = min_amount {
        query.push(" AND vi.gross_amount >= ").push_bind(min_amount);
    }

    if !statuses.is_empty() {
        query.push(" AND vi.status IN (");

        let mut separated = query.separated(", ");

        for status in statuses {
            separated.push_bind(status);
        }

        separated.push_unseparated(")");
    }

    query.push(" ORDER BY vi.issue_date DESC");

    query
        .build_query_as::<VendorInvoiceListItem>()
        .fetch_all(pool)
        .await
}

pub(crate) async fn get_top_payables(
    pool: &mut PgConnection,
    organization_id: Uuid,
    due_date_before: &NaiveDate,
) -> Result<Vec<TopPayable>, sqlx::Error> {
    sqlx::query_as!(
        TopPayable,
        r#"
        SELECT
            p.legal_name as partner_name,
            vi.due_date,
            vi.amount_remaining as amount
        FROM vendor_invoices vi
        JOIN partners p ON vi.partner_id = p.id
        WHERE vi.organization_id = $1
          AND (vi.status = $3 OR vi.status = $4)
          AND vi.due_date <= $2
        ORDER BY vi.amount_remaining DESC
        LIMIT 5
        "#,
        organization_id,
        due_date_before,
        InvoiceStatus::Open as InvoiceStatus,
        InvoiceStatus::PartiallyPaid as InvoiceStatus
    )
    .fetch_all(pool)
    .await
}

pub(crate) async fn insert(
    pool: &mut PgConnection,
    org_id: Uuid,
    transaction_id: Uuid,
    req: &CreateVendorInvoiceRequest,
) -> Result<VendorInvoice, sqlx::Error> {
    let net_amount = req.items.iter().map(|i| i.net_amount).sum::<Decimal>();
    let tax_amount = req.items.iter().map(|i| i.tax_amount).sum::<Decimal>();
    let gross_amount = net_amount + tax_amount;
    let row = sqlx::query_as!(
        VendorInvoice,
        r#"
        INSERT INTO vendor_invoices (organization_id, partner_id, transaction_id, invoice_number, issue_date, due_date, net_amount, tax_amount, gross_amount, amount_remaining, notes)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING id, organization_id, partner_id, transaction_id, invoice_number, status as "status: InvoiceStatus", issue_date, due_date, net_amount, tax_amount, gross_amount, amount_remaining, notes, created_at, updated_at
        "#,
        org_id,
        req.partner_id,
        transaction_id,
        req.invoice_number,
        req.issue_date,
        req.due_date,
        net_amount,
        tax_amount,
        gross_amount,
        gross_amount, // amount_remaining is the same as gross_amount on creation
        req.notes
    )
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub(crate) async fn update(
    pool: &mut PgConnection,
    id: Uuid,
    req: &UpdateVendorInvoiceRequest,
) -> Result<VendorInvoice, sqlx::Error> {
    let row = sqlx::query_as!(
        VendorInvoice,
        r#"
        UPDATE vendor_invoices
        SET invoice_number = $1, issue_date = $2, due_date = $3, notes = $4
        WHERE id = $5
        RETURNING id, organization_id, partner_id, transaction_id, invoice_number, status as "status: InvoiceStatus", issue_date, due_date, net_amount, tax_amount, gross_amount, amount_remaining, notes, created_at, updated_at
        "#,
        req.invoice_number,
        req.issue_date,
        req.due_date,
        req.notes,
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub(crate) async fn update_totals(
    pool: &mut PgConnection,
    id: Uuid,
    net_amount: Decimal,
    tax_amount: Decimal,
    gross_amount: Decimal,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE vendor_invoices
        SET net_amount = $1, tax_amount = $2, gross_amount = $3, amount_remaining = $3
        WHERE id = $4
        "#,
        net_amount,
        tax_amount,
        gross_amount,
        id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn update_amount_remaining(
    pool: &mut PgConnection,
    id: Uuid,
    amount: Decimal,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE vendor_invoices
        SET amount_remaining = amount_remaining + $1
        WHERE id = $2
        "#,
        amount,
        id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn insert_item(
    pool: &mut PgConnection,
    vendor_invoice_id: Uuid,
    item: &VendorInvoiceItem,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO vendor_invoice_items (id, vendor_invoice_id, account_id, description, net_amount, tax_amount, total_amount)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        item.id,
        vendor_invoice_id,
        item.account_id,
        item.description,
        item.net_amount,
        item.tax_amount,
        item.total_amount
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn delete_items(
    pool: &mut PgConnection,
    vendor_invoice_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM vendor_invoice_items
        WHERE vendor_invoice_id = $1
        "#,
        vendor_invoice_id
    )
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
    let count = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) AS "count!"
        FROM vendor_invoices
        WHERE organization_id = $1 AND partner_id = $2 AND invoice_number = $3
        "#,
        organization_id,
        partner_id,
        invoice_number
    )
    .fetch_one(pool)
    .await?;
    Ok(count > 0)
}
