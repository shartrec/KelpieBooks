/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use rust_decimal::Decimal;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    payables::dtos::vendor_invoice_list_item::VendorInvoiceListItem,
    PartnerId,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgedPayableSummary {
    pub partner_id: PartnerId,
    pub partner_name: String,
    pub current: Decimal,
    pub days_30: Decimal,
    pub days_60: Decimal,
    pub days_90: Decimal,
    pub days_90_plus: Decimal,
    pub total: Decimal,
    pub invoices: Vec<VendorInvoiceListItem>,
}
