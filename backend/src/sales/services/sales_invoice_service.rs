/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use rocket_db_pools::sqlx::{self, PgConnection};
use shared_core::sales::models::{
    sales_invoice::SalesInvoice,
    sales_invoice_item::SalesInvoiceLine,
};
use sqlx::Acquire;
use uuid::Uuid;

use crate::{
    sales::db::sales_invoice as sales_invoice_db,
    util::ApiError,
};
use shared_core::sales::requests::create_sales_invoice_request::CreateSalesInvoiceRequest;
use crate::core::db::sequences::get_next_invoice_number;

pub(crate) async fn create_draft_invoice(
    pool: &mut PgConnection,
    org_id: Uuid,
    req: &CreateSalesInvoiceRequest,
) -> Result<SalesInvoice, ApiError> {
    let mut tx = pool.begin().await?;

    // Get next invoice number
    let inv_number =  get_next_invoice_number(&mut tx, org_id, "sales-invoice").await?;

    let mut invoice = sales_invoice_db::create_draft_invoice(
        &mut tx,
        org_id,
        req.partner_id,
        &inv_number,
        req.issue_date,
        req.due_date,
    )
    .await?;

    for line in &req.lines {
        if line.item_id == Uuid::nil() {
            continue;
        }
        sales_invoice_db::insert_sales_invoice_line(&mut tx, invoice.id, org_id, line).await?;
    }

    invoice.lines = req.lines.clone();
    invoice.calculate();

    sales_invoice_db::update_sales_invoice_totals(
        &mut tx,
        invoice.id,
        invoice.subtotal,
        invoice.tax_total,
        invoice.total_amount,
    )
    .await?;

    tx.commit().await?;

    Ok(invoice)
}

pub(crate) async fn update_invoice_lines(
    pool: &mut PgConnection,
    org_id: Uuid,
    invoice_id: Uuid,
    lines: &[SalesInvoiceLine],
) -> Result<SalesInvoice, ApiError> {

    let mut invoice = sales_invoice_db::get_sales_invoice_with_lines(pool, invoice_id, org_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Invoice not found.".to_string()))?;

    if invoice.org_id != org_id {
        return Err(ApiError::Forbidden(
            "You do not have permission to update this invoice.".to_string(),
        ));
    }

    let mut tx = pool.begin().await?;
    sales_invoice_db::delete_sales_invoice_lines(&mut tx, invoice_id).await?;
    for line in lines {
        sales_invoice_db::insert_sales_invoice_line(&mut tx, invoice_id, org_id, line).await?;
    }

    invoice.lines = lines.to_vec();
    invoice.calculate();

    sales_invoice_db::update_sales_invoice_totals(
        &mut tx,
        invoice.id,
        invoice.subtotal,
        invoice.tax_total,
        invoice.total_amount,
    )
    .await?;

    tx.commit().await?;

    Ok(invoice)
}

pub(crate) async fn get_sales_invoice(
    pool: &mut PgConnection,
    org_id: Uuid,
    invoice_id: Uuid,
) -> Result<SalesInvoice, ApiError> {
    let invoice = sales_invoice_db::get_sales_invoice_with_lines(pool, invoice_id, org_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Invoice not found.".to_string()))?;

    if invoice.org_id != org_id {
        return Err(ApiError::Forbidden(
            "You do not have permission to view this invoice.".to_string(),
        ));
    }

    Ok(invoice)
}
