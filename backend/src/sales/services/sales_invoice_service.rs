/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use rocket_db_pools::sqlx::{self, PgConnection, Row};
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
use shared_core::sales::models::invoice_address::InvoiceAddress;

pub(crate) async fn create_draft_invoice(
    pool: &mut PgConnection,
    org_id: Uuid,
    req: &CreateSalesInvoiceRequest,
) -> Result<SalesInvoice, ApiError> {
    let mut tx = pool.begin().await?;

    // Get next invoice number
    let inv_number =  get_next_invoice_number(&mut tx, org_id, "sales-invoice").await?;

    // Resolve address snapshots: prefer provided snapshots; if empty and IDs provided, load from DB
    let resolve_snapshot = |
        snap: &InvoiceAddress,
        maybe_id: Option<Uuid>,
    | -> (Option<Uuid>, InvoiceAddress) {
        // Determine if snapshot is effectively empty (all fields None or empty strings)
        let is_empty = snap.name.as_deref().map_or(true, |s| s.trim().is_empty())
            && snap.attention.as_deref().map_or(true, |s| s.trim().is_empty())
            && snap.line1.as_deref().map_or(true, |s| s.trim().is_empty())
            && snap.line2.as_deref().map_or(true, |s| s.trim().is_empty())
            && snap.city.as_deref().map_or(true, |s| s.trim().is_empty())
            && snap.region.as_deref().map_or(true, |s| s.trim().is_empty())
            && snap.postal_code.as_deref().map_or(true, |s| s.trim().is_empty())
            && snap.country.as_deref().map_or(true, |s| s.trim().is_empty());
        (maybe_id, if is_empty { InvoiceAddress::default() } else { snap.clone() })
    };

    let (billing_id, mut bill_to_snap) = resolve_snapshot(&req.bill_to, req.billing_address_id);
    let (shipping_id, mut ship_to_snap) = resolve_snapshot(&req.ship_to, req.shipping_address_id);

    // Helper to load partner address and copy fields into snapshot if snapshot empty
    async fn load_address_into_snapshot(
        tx: &mut PgConnection,
        org_id: Uuid,
        partner_id: Uuid,
        address_id: Uuid,
        target: &mut InvoiceAddress,
    ) -> Result<(), ApiError> {
        let row = sqlx::query(
            r#"
            SELECT id, organization_id, partner_id,
                   address_line1, address_line2, city, state_province, postal_code, country
            FROM partner_addresses
            WHERE id = $1 AND organization_id = $2 AND partner_id = $3
            "#,
        )
        .bind(address_id)
        .bind(org_id)
        .bind(partner_id)
        .fetch_optional(&mut *tx)
        .await?;

        match row {
            Some(r) => {
                // Only fill missing fields, allow frontend overrides to take precedence
                if target.line1.as_deref().map_or(true, |s| s.is_empty()) {
                    target.line1 = Some(r.get::<String, _>("address_line1"));
                }
                if target.line2.as_deref().map_or(true, |s| s.is_empty()) {
                    target.line2 = r.try_get::<String, _>("address_line2").ok();
                }
                if target.city.as_deref().map_or(true, |s| s.is_empty()) {
                    target.city = r.try_get::<String, _>("city").ok();
                }
                if target.region.as_deref().map_or(true, |s| s.is_empty()) {
                    target.region = r.try_get::<String, _>("state_province").ok();
                }
                if target.postal_code.as_deref().map_or(true, |s| s.is_empty()) {
                    target.postal_code = r.try_get::<String, _>("postal_code").ok();
                }
                if target.country.as_deref().map_or(true, |s| s.is_empty()) {
                    target.country = r.try_get::<String, _>("country").ok();
                }
                Ok(())
            }
            None => Err(ApiError::BadRequest(
                "Invalid address selection for this partner/organization".to_string(),
            )),
        }
    }

    if let Some(id) = billing_id {
        load_address_into_snapshot(&mut tx, org_id, req.partner_id, id, &mut bill_to_snap).await?;
    }
    if let Some(id) = shipping_id {
        load_address_into_snapshot(&mut tx, org_id, req.partner_id, id, &mut ship_to_snap).await?;
    }

    let mut invoice = sales_invoice_db::create_draft_invoice(
        &mut tx,
        org_id,
        req.partner_id,
        &inv_number,
        req.issue_date,
        req.due_date,
        billing_id,
        shipping_id,
        bill_to_snap.name.as_deref(),
        bill_to_snap.attention.as_deref(),
        bill_to_snap.line1.as_deref(),
        bill_to_snap.line2.as_deref(),
        bill_to_snap.city.as_deref(),
        bill_to_snap.region.as_deref(),
        bill_to_snap.postal_code.as_deref(),
        bill_to_snap.country.as_deref(),
        ship_to_snap.name.as_deref(),
        ship_to_snap.attention.as_deref(),
        ship_to_snap.line1.as_deref(),
        ship_to_snap.line2.as_deref(),
        ship_to_snap.city.as_deref(),
        ship_to_snap.region.as_deref(),
        ship_to_snap.postal_code.as_deref(),
        ship_to_snap.country.as_deref(),
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
