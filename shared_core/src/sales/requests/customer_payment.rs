/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    sales::models::customer_payment_allocation::CustomerPaymentAllocation,
    AccountId,
    PartnerId,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateCustomerPaymentRequest {
    pub partner_id: PartnerId,
    pub payment_date: NaiveDate,
    pub bank_account_id: AccountId,
    pub amount: Decimal,
    pub reference: Option<String>,
    pub allocations: Vec<CustomerPaymentAllocation>,
}
