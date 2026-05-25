/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDate;
use crate::models::vendor_payment_allocation::VendorPaymentAllocation;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateVendorPaymentRequest {
    pub partner_id: Uuid,
    pub payment_date: NaiveDate,
    pub bank_account_id: Uuid,
    pub amount: i64,
    pub reference: Option<String>,
    pub allocations: Vec<VendorPaymentAllocation>,
}
