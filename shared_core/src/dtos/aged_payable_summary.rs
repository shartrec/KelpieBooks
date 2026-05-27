/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::dtos::vendor_invoice_list_item::VendorInvoiceListItem;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgedPayableSummary {
    pub partner_id: Uuid,
    pub partner_name: String,
    pub current: i64,
    pub days_30: i64,
    pub days_60: i64,
    pub days_90: i64,
    pub days_90_plus: i64,
    pub total: i64,
    pub invoices: Vec<VendorInvoiceListItem>,
}
