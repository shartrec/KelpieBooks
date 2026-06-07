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
use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VendorPayment {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub partner_id: Uuid,

    // Links this payment back to the transaction that cleared the cash account
    pub transaction_id: Option<Uuid>,

    pub payment_date: NaiveDate,
    pub paid_from_account: Uuid, // e.g., "EFT", "Check", "Card"
    pub amount: i64,             // Total paid value in cents
    pub reference: Option<String>,

    pub created_at: DateTime<Utc>,
}
