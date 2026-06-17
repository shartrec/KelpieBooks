/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */


use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::sales::models::sales_invoice_item::SalesInvoiceLine;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSalesInvoiceRequest {
    pub partner_id: Uuid,
    pub invoice_number: String,
    pub issue_date: NaiveDate,
    pub due_date: NaiveDate,
    pub lines: Vec<SalesInvoiceLine>,
}
