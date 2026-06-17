/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::sales::models::invoice_status::InvoiceStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesInvoiceListItem {
    pub id: Uuid,
    pub partner_id: Uuid,
    pub partner_name: String,
    pub invoice_number: String,
    pub status: InvoiceStatus,
    pub issue_date: NaiveDate,
    pub due_date: NaiveDate,
    pub net_amount: Decimal,
    pub tax_amount: Decimal,
    pub gross_amount: Decimal,
}
