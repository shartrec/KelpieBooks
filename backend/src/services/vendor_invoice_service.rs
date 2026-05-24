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
use crate::db::{account as account_db, vendor_invoice as vendor_invoice_db};
use crate::services::account_service;
use crate::util::ApiError;
use chrono::{Local, NaiveDate};
use rocket_db_pools::sqlx::{self, PgConnection};
use shared_core::dtos::vendor_invoice_list_item::VendorInvoiceListItem;
use shared_core::models::invoice_status::InvoiceStatus;
use shared_core::models::system_tag::SystemTag;
use shared_core::models::vendor_invoice::VendorInvoice;
use shared_core::models::vendor_invoice_item::VendorInvoiceItem;
use shared_core::requests::transaction::{CreateTransactionRequest, JournalEntryLine};
use shared_core::requests::vendor_invoice::{CreateVendorInvoiceRequest, UpdateVendorInvoiceRequest};
use sqlx::Acquire;
use uuid::Uuid;

pub async fn get_vendor_invoices(
    pool: &mut PgConnection,
    organization_id: Uuid,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    partner_id: Option<Uuid>,
    min_amount: Option<i64>,
    status: Option<String>,
) -> Result<Vec<VendorInvoiceListItem>, ApiError> {
    let invoices = vendor_invoice_db::get_by_org(pool, organization_id, start_date, end_date, partner_id, min_amount, status).await?;
    Ok(invoices)
}

pub async fn get_vendor_invoice(
    pool: &mut PgConnection,
    organization_id: Uuid,
    id: Uuid,
) -> Result<VendorInvoice, ApiError> {
    let mut invoice = vendor_invoice_db::get(pool, id).await?.ok_or_else(|| ApiError::NotFound("Invoice not found.".to_string()))?;
    if invoice.organization_id != organization_id {
        return Err(ApiError::Forbidden("You do not have permission to view this invoice.".to_string()));
    }
    invoice.items = vendor_invoice_db::get_items(pool, id).await?;
    Ok(invoice)
}

pub async fn create_vendor_invoice(
    pool: &mut PgConnection,
    organization_id: Uuid,
    req: &CreateVendorInvoiceRequest,
) -> Result<VendorInvoice, ApiError> {
    if vendor_invoice_db::is_duplicate(pool, organization_id, req.partner_id, &req.invoice_number).await? {
        return Err(ApiError::Conflict("Duplicate invoice number for this vendor.".to_string()));
    }

    let mut tx = pool.begin().await?;

    let total_net: i64 = req.items.iter().map(|item| item.net_amount).sum();
    let total_tax: i64 = req.items.iter().map(|item| item.tax_amount).sum();
    let gross_amount = total_net + total_tax;

    let ap_account = account_db::get_by_system_tag(&mut tx, organization_id, SystemTag::AccountsPayable.to_string().as_str()).await?.ok_or_else(|| ApiError::NotFound("Accounts Payable account not found.".to_string()))?;
    let tax_account = account_db::get_by_system_tag(&mut tx, organization_id, SystemTag::SalesTaxClearing.to_string().as_str()).await?.ok_or_else(|| ApiError::NotFound("Tax account not found.".to_string()))?;

    let mut jels = vec![];
    for item in &req.items {
        if item.net_amount > 0 { // Will only be 0 if the item is a tax line
            let jel = JournalEntryLine {
                line_id: Uuid::new_v4(),
                account_id: item.account_id,
                debit: item.net_amount,
                credit: 0,
                description: Some(item.description.clone()),
            };
            jels.push(jel);
        }
    }

    if total_tax > 0 {
        let jel = JournalEntryLine {
            line_id: Uuid::new_v4(),
            account_id: tax_account.id,
            debit: total_tax,
            credit: 0,
            description: Some("Tax on vendor invoice".to_string()),
        };
        jels.push(jel);
    }
    let jel = JournalEntryLine {
        line_id: Uuid::new_v4(),
        account_id: ap_account.id,
        debit: 0,
        credit: gross_amount,
        description: Some("Vendor invoice".to_string()),
    };
    jels.push(jel);

    let ct_req = CreateTransactionRequest{
        date: Local::now().date_naive(),
        description: Some(format!("Vendor Invoice {}", req.invoice_number)),
        reference: Some(req.invoice_number.clone()),
        entries: jels,
    };
    let transaction_id = account_service::create_transaction(&mut tx, organization_id, &ct_req).await?;

    let mut new_invoice = vendor_invoice_db::insert(&mut tx, organization_id, transaction_id, req).await?;
    new_invoice.status = if new_invoice.amount_remaining == 0 {
        InvoiceStatus::Paid
    } else {
        InvoiceStatus::Open
    };
    vendor_invoice_db::update_status(&mut tx, new_invoice.id, new_invoice.status).await?;


    for item in &req.items {
        vendor_invoice_db::insert_item(&mut tx, new_invoice.id, item).await?;
    }

    tx.commit().await?;

    let created_invoice = get_vendor_invoice(pool, organization_id, new_invoice.id).await?;
    Ok(created_invoice)
}

pub async fn update_vendor_invoice(
    pool: &mut PgConnection,
    organization_id: Uuid,
    id: Uuid,
    req: &UpdateVendorInvoiceRequest,
) -> Result<VendorInvoice, ApiError> {
    let invoice = vendor_invoice_db::get(pool, id).await?.ok_or_else(|| ApiError::NotFound("Invoice not found.".to_string()))?;
    if invoice.organization_id != organization_id {
        return Err(ApiError::Forbidden("You do not have permission to update this invoice.".to_string()));
    }
    let updated_invoice = vendor_invoice_db::update(pool, id, req).await?;
    let returned_invoice = get_vendor_invoice(pool, organization_id, updated_invoice.id).await?;
    Ok(returned_invoice)
}

pub async fn update_vendor_invoice_items(
    pool: &mut PgConnection,
    organization_id: Uuid,
    id: Uuid,
    items: &Vec<VendorInvoiceItem>,
) -> Result<Vec<VendorInvoiceItem>, ApiError> {
    let invoice = vendor_invoice_db::get(pool, id).await?.ok_or_else(|| ApiError::NotFound("Invoice not found.".to_string()))?;
    if invoice.organization_id != organization_id {
        return Err(ApiError::Forbidden("You do not have permission to update this invoice.".to_string()));
    }

    let mut tx = pool.begin().await?;

    vendor_invoice_db::delete_items(&mut tx, id).await?;
    for item in items {
        vendor_invoice_db::insert_item(&mut tx, id, item).await?;
    }

    let total_net: i64 = items.iter().map(|item| item.net_amount).sum();
    let total_tax: i64 = items.iter().map(|item| item.tax_amount).sum();
    let gross_amount = total_net + total_tax;

    vendor_invoice_db::update_totals(&mut tx, id, total_net, total_tax, gross_amount).await?;

    let status = if gross_amount == 0 {
        InvoiceStatus::Paid
    } else if gross_amount > invoice.amount_remaining {
        InvoiceStatus::PartiallyPaid
    } else {
        InvoiceStatus::Open
    };
    vendor_invoice_db::update_status(&mut tx, id, status).await?;

    tx.commit().await?;

    let updated_items = vendor_invoice_db::get_items(pool, id).await?;
    Ok(updated_items)
}
