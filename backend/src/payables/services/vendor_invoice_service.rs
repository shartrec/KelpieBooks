/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use chrono::{
    Local,
    NaiveDate,
};
use rocket_db_pools::sqlx::{
    self,
    PgConnection,
};
use rust_decimal::{
    dec,
    Decimal,
};
use shared_core::{ledger::{
    models::system_tag::SystemTag,
    requests::transaction::{
        CreateTransactionRequest,
        JournalEntryLine,
    },
}, payables::{
    dtos::{
        vendor_invoice_dto::VendorInvoiceDto,
        vendor_invoice_list_item::VendorInvoiceListItem,
    },
    models::{
        invoice_status::InvoiceStatus,
        vendor_invoice_item::VendorInvoiceItem,
    },
    requests::vendor_invoice::{
        CreateVendorInvoiceRequest,
        UpdateVendorInvoiceRequest,
    },
}, InvoiceId, OrgId, PartnerId};
use sqlx::Acquire;
use uuid::Uuid;

use crate::{
    ledger::{
        db::account as account_db,
        services::account_service,
    },
    payables::db::vendor_invoice as vendor_invoice_db,
    util::ApiError,
};

pub(crate) async fn get_vendor_invoices(
    pool: &mut PgConnection,
    organization_id: OrgId,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    partner_id: Option<PartnerId>,
    min_amount: Option<Decimal>,
    status: Option<InvoiceStatus>,
) -> Result<Vec<VendorInvoiceListItem>, ApiError> {
    let invoices = vendor_invoice_db::get_by_org(
        pool,
        organization_id,
        start_date,
        end_date,
        partner_id,
        min_amount,
        if let Some(status) = &status {
            vec![status]
        } else {
            vec![]
        },
    )
    .await?;
    Ok(invoices)
}

pub(crate) async fn get_vendor_invoice(
    pool: &mut PgConnection,
    organization_id: OrgId,
    id: InvoiceId,
) -> Result<VendorInvoiceDto, ApiError> {
    let invoice = vendor_invoice_db::get(pool, id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Invoice not found.".to_string()))?;
    if invoice.organization_id != organization_id {
        return Err(ApiError::Forbidden(
            "You do not have permission to view this invoice.".to_string(),
        ));
    }
    let items = vendor_invoice_db::get_items(pool, id).await?;
    Ok(VendorInvoiceDto { invoice, items })
}

pub(crate) async fn create_vendor_invoice(
    pool: &mut PgConnection,
    organization_id: OrgId,
    req: &CreateVendorInvoiceRequest,
) -> Result<VendorInvoiceDto, ApiError> {
    if vendor_invoice_db::is_duplicate(pool, organization_id, req.partner_id, &req.invoice_number)
        .await?
    {
        return Err(ApiError::Conflict(
            "Duplicate invoice number for this vendor.".to_string(),
        ));
    }

    let mut tx = pool.begin().await?;

    let total_net: Decimal = req.items.iter().map(|item| item.net_amount).sum();
    let total_tax: Decimal = req.items.iter().map(|item| item.tax_amount).sum();
    let gross_amount = total_net + total_tax;

    let ap_account =
        account_db::get_by_system_tag(&mut tx, organization_id, &SystemTag::AccountsPayable)
            .await?
            .ok_or_else(|| ApiError::NotFound("Accounts Payable account not found.".to_string()))?;
    let tax_account =
        account_db::get_by_system_tag(&mut tx, organization_id, &SystemTag::SalesTaxClearing)
            .await?
            .ok_or_else(|| ApiError::NotFound("Tax account not found.".to_string()))?;

    let mut jels = vec![];
    for item in &req.items {
        if item.net_amount > dec!(0.00) {
            // Will only be 0 if the item is a tax line
            let jel = JournalEntryLine {
                line_id: Uuid::new_v4(),
                account_id: item.account_id,
                debit: item.net_amount,
                credit: dec!(0.00),
                description: item.description.clone(),
            };
            jels.push(jel);
        }
    }

    if total_tax > dec!(0.00) {
        let jel = JournalEntryLine {
            line_id: Uuid::new_v4(),
            account_id: tax_account.id,
            debit: total_tax,
            credit: dec!(0.00),
            description: Some("Tax on vendor invoice".to_string()),
        };
        jels.push(jel);
    }
    let jel = JournalEntryLine {
        line_id: Uuid::new_v4(),
        account_id: ap_account.id,
        debit: dec!(0.00),
        credit: gross_amount,
        description: Some("Vendor invoice".to_string()),
    };
    jels.push(jel);

    let ct_req = CreateTransactionRequest {
        date: Local::now().date_naive(),
        description: Some(format!("Vendor Invoice {}", req.invoice_number)),
        reference: Some(req.invoice_number.clone()),
        entries: jels,
    };
    let transaction_id =
        account_service::create_transaction(&mut tx, organization_id, &ct_req).await?;

    let new_invoice =
        vendor_invoice_db::insert(&mut tx, organization_id, transaction_id, req).await?;

    for item in &req.items {
        vendor_invoice_db::insert_item(&mut tx, new_invoice.id, item).await?;
    }

    tx.commit().await?;

    let created_invoice = get_vendor_invoice(pool, organization_id, new_invoice.id).await?;
    Ok(created_invoice)
}

pub(crate) async fn update_vendor_invoice(
    pool: &mut PgConnection,
    organization_id: OrgId,
    id: InvoiceId,
    req: &UpdateVendorInvoiceRequest,
) -> Result<VendorInvoiceDto, ApiError> {
    let invoice = vendor_invoice_db::get(pool, id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Invoice not found.".to_string()))?;
    if invoice.organization_id != organization_id {
        return Err(ApiError::Forbidden(
            "You do not have permission to update this invoice.".to_string(),
        ));
    }
    let updated_invoice = vendor_invoice_db::update(pool, id, req).await?;
    let returned_invoice = get_vendor_invoice(pool, organization_id, updated_invoice.id).await?;
    Ok(returned_invoice)
}

pub(crate) async fn update_vendor_invoice_items(
    pool: &mut PgConnection,
    organization_id: OrgId,
    id: InvoiceId,
    items: &Vec<VendorInvoiceItem>,
) -> Result<Vec<VendorInvoiceItem>, ApiError> {
    let invoice = vendor_invoice_db::get(pool, id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Invoice not found.".to_string()))?;
    if invoice.organization_id != organization_id {
        return Err(ApiError::Forbidden(
            "You do not have permission to update this invoice.".to_string(),
        ));
    }

    let mut tx = pool.begin().await?;

    vendor_invoice_db::delete_items(&mut tx, id).await?;
    for item in items {
        vendor_invoice_db::insert_item(&mut tx, id, item).await?;
    }

    let total_net: Decimal = items.iter().map(|item| item.net_amount).sum();
    let total_tax: Decimal = items.iter().map(|item| item.tax_amount).sum();
    let gross_amount = total_net + total_tax;

    vendor_invoice_db::update_totals(&mut tx, id, total_net, total_tax, gross_amount).await?;

    tx.commit().await?;

    let updated_items = vendor_invoice_db::get_items(pool, id).await?;
    Ok(updated_items)
}
