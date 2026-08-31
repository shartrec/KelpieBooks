/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::DateTime;
use rust_decimal::Decimal;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    AccountId,
    JournalEntryId,
    TransactionId,
};

/// A DTO representing a journal entry line, including the name of the account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JournalEntryDetail {
    pub id: JournalEntryId,
    pub transaction_id: TransactionId,
    pub account_id: AccountId,
    pub account_name: String, // The joined account name
    pub debit: Decimal,
    pub credit: Decimal,
    pub description: Option<String>,
    pub created_at: DateTime<chrono::Utc>,
}
