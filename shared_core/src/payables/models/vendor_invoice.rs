/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::{
    DateTime,
    NaiveDate,
    Utc,
};
use rust_decimal::Decimal;
use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;
use crate::{OrgId, PartnerId};
use crate::payables::models::invoice_status::InvoiceStatus;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VendorInvoice {
    pub id: Uuid,
    pub organization_id: OrgId,
    pub partner_id: PartnerId,
    pub transaction_id: Option<Uuid>,
    pub invoice_number: String,
    pub status: InvoiceStatus,
    pub issue_date: NaiveDate,
    pub due_date: NaiveDate,
    pub net_amount: Decimal,
    pub tax_amount: Decimal,
    pub gross_amount: Decimal,
    pub amount_remaining: Decimal,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl VendorInvoice {
    pub fn is_overdue(&self, today: NaiveDate) -> bool {
        self.status != InvoiceStatus::Paid
            && self.status != InvoiceStatus::Void
            && self.due_date < today
    }
}
