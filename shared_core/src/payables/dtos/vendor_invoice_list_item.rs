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
use uuid::Uuid;

use crate::payables::models::invoice_status::InvoiceStatus;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VendorInvoiceListItem {
    pub id: Uuid,
    pub partner_id: Uuid,
    pub partner_name: String,
    pub invoice_number: String,
    pub issue_date: NaiveDate,
    pub due_date: NaiveDate,
    pub net_amount: i64,
    pub tax_amount: i64,
    pub gross_amount: i64,
    pub amount_remaining: i64,
    pub status: InvoiceStatus,
}
