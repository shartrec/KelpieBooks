/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use serde::{Deserialize, Serialize};
use crate::payables::models::vendor_invoice::VendorInvoice;
use crate::payables::models::vendor_invoice_item::VendorInvoiceItem;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VendorInvoiceDto {
    pub invoice: VendorInvoice,
    pub items: Vec<VendorInvoiceItem>,
}
