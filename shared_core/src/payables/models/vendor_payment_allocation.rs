/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::{
    DateTime,
    Utc,
};
use rust_decimal::Decimal;
use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;
use crate::OrgId;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VendorPaymentAllocation {
    pub id: Uuid,
    pub organization_id: OrgId,
    pub vendor_invoice_id: Uuid,
    pub vendor_payment_id: Uuid,
    pub allocated_amount: Decimal,
    pub created_at: DateTime<Utc>,
}
