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
    OrderId,
    OrgId,
    PaymentId,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CustomerPaymentAllocation {
    pub id: AllocationId,
    pub organization_id: OrgId,
    pub sales_order_id: OrderId,
    pub customer_payment_id: PaymentId,
    pub allocated_amount: Decimal,
    pub created_at: DateTime<Utc>,
}
