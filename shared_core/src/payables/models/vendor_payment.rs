/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::{
    DateTime,
    NaiveDate,
    Utc,
};
use rust_decimal::Decimal;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    AccountId,
    OrgId,
    PartnerId,
    PaymentId,
    TransactionId,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VendorPayment {
    pub id: PaymentId,
    pub organization_id: OrgId,
    pub partner_id: PartnerId,

    // Links this payment back to the transaction that cleared the cash account
    pub transaction_id: Option<TransactionId>,

    pub payment_date: NaiveDate,
    pub paid_from_account: AccountId, // e.g., "EFT", "Check", "Card"
    pub amount: Decimal,
    pub reference: Option<String>,

    pub created_at: DateTime<Utc>,
}
