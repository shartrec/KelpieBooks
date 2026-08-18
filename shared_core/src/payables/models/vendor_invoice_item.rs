/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VendorInvoiceItem {
    pub id: Uuid,
    pub vendor_invoice_id: Uuid,
    pub account_id: Uuid, // Target GL Expense Account
    pub description: Option<String>,
    pub net_amount: Decimal,
    pub tax_amount: Decimal,
    pub total_amount: Decimal,
    pub created_at: DateTime<Utc>,
}
