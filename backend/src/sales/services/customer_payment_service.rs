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
use rust_decimal::dec;
use shared_core::{
    ledger::{
        models::system_tag::SystemTag,
        requests::transaction::{
            CreateTransactionRequest,
            JournalEntryLine,
        },
    },
    sales::{
        models::customer_payment::CustomerPayment,
        requests::customer_payment::CreateCustomerPaymentRequest,
    },
};
use sqlx::Acquire;
use uuid::Uuid;

use crate::{
    ledger::{
        db::account as account_db,
        services::account_service,
    },
    sales::db::{
        sales_invoice as sales_invoice_db,
        customer_payment as customer_payment_db,
        customer_payment_allocation as customer_payment_allocation_db,
    },
    util::ApiError,
};

pub(crate) async fn get_customer_invoice_payments(
    pool: &mut PgConnection,
    organization_id: Uuid,
    invoice_id: Uuid,
) -> Result<Vec<CustomerPayment>, ApiError> {
    let _invoice = sales_invoice_db::get(pool, invoice_id, organization_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Invoice not found.".to_string()))?;
    let payments = customer_payment_db::get_all_by_invoice(pool, invoice_id).await?;
    Ok(payments)
}

pub(crate) async fn create_customer_payment(
    pool: &mut PgConnection,
    organization_id: Uuid,
    req: &CreateCustomerPaymentRequest,
) -> Result<CustomerPayment, ApiError> {
    let mut tx = pool.begin().await?;

    let ap_account =
        account_db::get_by_system_tag(&mut tx, organization_id, &SystemTag::AccountsReceivable)
            .await?
            .ok_or_else(|| ApiError::NotFound("Accounts Receivable account not found.".to_string()))?;

    let jels = vec![
        JournalEntryLine {
            line_id: Uuid::new_v4(),
            account_id: ap_account.id,
            debit: req.amount,
            credit: dec!(0.00),
            description: Some(format!("Payment by invoice {}", req.partner_id)),
        },
        JournalEntryLine {
            line_id: Uuid::new_v4(),
            account_id: req.bank_account_id,
            debit: dec!(0.00),
            credit: req.amount,
            description: Some(format!("Payment by invoice {}", req.partner_id)),
        },
    ];

    let ct_req = CreateTransactionRequest {
        date: req.payment_date,
        description: Some(format!("Payment to customer {}", req.partner_id)),
        reference: req.reference.clone(),
        entries: jels,
    };
    let transaction_id =
        account_service::create_transaction(&mut tx, organization_id, &ct_req).await?;

    let new_payment =
        customer_payment_db::insert(&mut tx, organization_id, transaction_id, req).await?;

    for allocation in &req.allocations {
        customer_payment_allocation_db::insert(&mut tx, new_payment.id, allocation).await?;
        sales_invoice_db::update_amount_remaining(
            &mut tx,
            allocation.sales_invoice_id,
            -allocation.allocated_amount,
        )
        .await?;
    }

    tx.commit().await?;

    Ok(new_payment)
}
