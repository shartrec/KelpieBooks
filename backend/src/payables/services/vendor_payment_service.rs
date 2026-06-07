/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use rocket_db_pools::sqlx::{
    self,
    PgConnection,
};
use shared_core::{
    ledger::{
        models::system_tag::SystemTag,
        requests::transaction::{
            CreateTransactionRequest,
            JournalEntryLine,
        },
    },
    payables::{
        models::vendor_payment::VendorPayment,
        requests::vendor_payment::CreateVendorPaymentRequest,
    },
};
use sqlx::Acquire;
use uuid::Uuid;

use crate::{
    ledger::{
        db::account as account_db,
        services::account_service,
    },
    payables::db::{
        vendor_invoice as vendor_invoice_db,
        vendor_payment as vendor_payment_db,
        vendor_payment_allocation as vendor_payment_allocation_db,
    },
    util::ApiError,
};

pub(crate) async fn get_vendor_invoice_payments(
    pool: &mut PgConnection,
    organization_id: Uuid,
    invoice_id: Uuid,
) -> Result<Vec<VendorPayment>, ApiError> {
    let invoice = vendor_invoice_db::get(pool, invoice_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Invoice not found.".to_string()))?;
    if invoice.organization_id != organization_id {
        return Err(ApiError::Forbidden(
            "You do not have permission to view this invoice.".to_string(),
        ));
    }
    let payments = vendor_payment_db::get_all_by_invoice(pool, invoice_id).await?;
    Ok(payments)
}

pub(crate) async fn create_vendor_payment(
    pool: &mut PgConnection,
    organization_id: Uuid,
    req: &CreateVendorPaymentRequest,
) -> Result<VendorPayment, ApiError> {
    let mut tx = pool.begin().await?;

    let ap_account = account_db::get_by_system_tag(
        &mut tx,
        organization_id,
        SystemTag::AccountsPayable.to_string().as_str(),
    )
    .await?
    .ok_or_else(|| ApiError::NotFound("Accounts Payable account not found.".to_string()))?;

    let jels = vec![
        JournalEntryLine {
            line_id: Uuid::new_v4(),
            account_id: ap_account.id,
            debit: req.amount,
            credit: 0,
            description: Some(format!("Payment to vendor {}", req.partner_id)),
        },
        JournalEntryLine {
            line_id: Uuid::new_v4(),
            account_id: req.bank_account_id,
            debit: 0,
            credit: req.amount,
            description: Some(format!("Payment to vendor {}", req.partner_id)),
        },
    ];

    let ct_req = CreateTransactionRequest {
        date: req.payment_date,
        description: Some(format!("Payment to vendor {}", req.partner_id)),
        reference: req.reference.clone(),
        entries: jels,
    };
    let transaction_id =
        account_service::create_transaction(&mut tx, organization_id, &ct_req).await?;

    let new_payment =
        vendor_payment_db::insert(&mut tx, organization_id, transaction_id, req).await?;

    for allocation in &req.allocations {
        vendor_payment_allocation_db::insert(&mut tx, new_payment.id, allocation).await?;
        vendor_invoice_db::update_amount_remaining(
            &mut tx,
            allocation.vendor_invoice_id,
            -allocation.allocated_amount,
        )
        .await?;
    }

    tx.commit().await?;

    Ok(new_payment)
}
