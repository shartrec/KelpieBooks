/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::NaiveDate;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    payables::models::vendor_invoice_item::VendorInvoiceItem,
    InvoiceId,
    PartnerId,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateVendorInvoiceRequest {
    pub partner_id: PartnerId,
    pub invoice_number: String,
    pub issue_date: NaiveDate,
    pub due_date: NaiveDate,
    pub notes: Option<String>,
    pub items: Vec<VendorInvoiceItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateVendorInvoiceRequest {
    pub id: InvoiceId,
    pub invoice_number: String,
    pub issue_date: NaiveDate,
    pub due_date: NaiveDate,
    pub notes: Option<String>,
}
