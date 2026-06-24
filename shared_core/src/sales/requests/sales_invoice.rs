/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */


use crate::sales::models::invoice_address::InvoiceAddress;
use crate::sales::models::sales_invoice_item::SalesInvoiceItem;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSalesInvoiceRequest {
    pub partner_id: Uuid,
    pub invoice_number: String,
    pub issue_date: NaiveDate,
    pub due_date: NaiveDate,
    pub lines: Vec<SalesInvoiceItem>,

    // Optional references to saved partner addresses used to populate the snapshots
    pub billing_address_id: Option<Uuid>,
    pub shipping_address_id: Option<Uuid>,

    // Snapshots stored on the invoice (overridable by user per-invoice)
    pub bill_to: InvoiceAddress,
    pub ship_to: InvoiceAddress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSalesInvoiceRequest {
    pub id: Uuid,
    pub issue_date: NaiveDate,
    pub due_date: NaiveDate,

    // Optional references to saved partner addresses used to populate the snapshots
    pub billing_address_id: Option<Uuid>,
    pub shipping_address_id: Option<Uuid>,

    // Snapshots stored on the invoice (overridable by user per-invoice)
    pub bill_to: InvoiceAddress,
    pub ship_to: InvoiceAddress,
}
