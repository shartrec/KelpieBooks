/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::payables::models::{
    invoice_status::InvoiceStatus,
    vendor_invoice_item::VendorInvoiceItem
};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VendorInvoice {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub partner_id: Uuid,
    pub transaction_id: Option<Uuid>,
    pub invoice_number: String,
    pub status: InvoiceStatus,
    pub issue_date: NaiveDate,
    pub due_date: NaiveDate,
    pub net_amount: i64,
    pub tax_amount: i64,
    pub gross_amount: i64,
    pub amount_remaining: i64,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub items: Vec<VendorInvoiceItem>,
}

impl VendorInvoice {
    pub fn is_overdue(&self, today: NaiveDate) -> bool {
        self.status != InvoiceStatus::Paid
            && self.status != InvoiceStatus::Void
            && self.due_date < today
    }
}
