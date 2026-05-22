/*
 * Copyright (c) 2026-2026. Trevor Campbell and others.
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

use crate::db::{account as account_db, vendor_invoice as vendor_invoice_db, vendor_invoice_payment as vendor_invoice_payment_db};
use crate::services::account_service;
use crate::util::ApiError;
use rocket_db_pools::sqlx::{self, PgConnection};
use shared_core::models::system_tag::SystemTag;
use shared_core::models::vendor_invoice_payment::VendorInvoicePayment;
use shared_core::requests::transaction::{CreateTransactionRequest, JournalEntryLine};
use shared_core::requests::vendor_invoice_payment::CreateVendorInvoicePaymentRequest;
use sqlx::Acquire;
use uuid::Uuid;

pub async fn get_vendor_invoice_payments(
    pool: &mut PgConnection,
    organization_id: Uuid,
    invoice_id: Uuid,
) -> Result<Vec<VendorInvoicePayment>, ApiError> {
    let invoice = vendor_invoice_db::get(pool, invoice_id).await?.ok_or_else(|| ApiError::NotFound("Invoice not found.".to_string()))?;
    if invoice.organization_id != organization_id {
        return Err(ApiError::Forbidden("You do not have permission to update this invoice.".to_string()));
    }
    let payments = vendor_invoice_payment_db::get_all_by_invoice(pool, invoice_id).await?;
    Ok(payments)
}

pub async fn create_vendor_invoice_payment(
    pool: &mut PgConnection,
    organization_id: Uuid,
    req: &CreateVendorInvoicePaymentRequest,
) -> Result<VendorInvoicePayment, ApiError> {
    let mut tx = pool.begin().await?;

    let invoice = vendor_invoice_db::get(&mut tx, req.vendor_invoice_id).await?.ok_or_else(|| ApiError::NotFound("Invoice not found.".to_string()))?;
    if invoice.organization_id != organization_id {
        return Err(ApiError::Forbidden("You do not have permission to update this invoice.".to_string()));
    }

    let ap_account = account_db::get_by_system_tag(&mut tx, organization_id, SystemTag::AccountsPayable.to_string().as_str()).await?.ok_or_else(|| ApiError::NotFound("Accounts Payable account not found.".to_string()))?;

    let jels = vec![
        JournalEntryLine {
            line_id: Uuid::new_v4(),
            account_id: ap_account.id,
            debit: req.amount,
            credit: 0,
            description: Some(format!("Payment for invoice {}", invoice.invoice_number)),
        },
        JournalEntryLine {
            line_id: Uuid::new_v4(),
            account_id: req.bank_account_id,
            debit: 0,
            credit: req.amount,
            description: Some(format!("Payment for invoice {}", invoice.invoice_number)),
        },
    ];

    let ct_req = CreateTransactionRequest {
        date: req.payment_date,
        description: Some(format!("Payment for invoice {}", invoice.invoice_number)),
        reference: req.reference.clone(),
        entries: jels,
    };
    let transaction_id = account_service::create_transaction(&mut tx, organization_id, ct_req).await?;

    let new_payment = vendor_invoice_payment_db::insert(&mut tx, organization_id, transaction_id, req).await?;

    vendor_invoice_db::update_amount_remaining(&mut tx, req.vendor_invoice_id, -req.amount).await?;

    tx.commit().await?;

    Ok(new_payment)
}
