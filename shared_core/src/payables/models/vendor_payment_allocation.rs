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

use crate::{
    AllocationId,
    InvoiceId,
    OrgId,
    PaymentId,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VendorPaymentAllocation {
    pub id: AllocationId,
    pub organization_id: OrgId,
    pub vendor_invoice_id: InvoiceId,
    pub vendor_payment_id: PaymentId,
    pub allocated_amount: Decimal,
    pub created_at: DateTime<Utc>,
}
