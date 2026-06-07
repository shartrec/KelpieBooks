/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

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
    pub description: String,
    pub net_amount: i64,
    pub tax_amount: i64,
    pub total_amount: i64,
}
