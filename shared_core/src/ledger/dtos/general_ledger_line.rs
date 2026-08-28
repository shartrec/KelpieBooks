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
use crate::{AccountId, JournalEntryId, TransactionId};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeneralLedgerLine {
    pub transaction_id: TransactionId,
    pub journal_entry_id: JournalEntryId,
    pub date: NaiveDate,
    pub account_id: AccountId,
    pub account_name: String,
    pub code: String,
    pub description: Option<String>,
    pub debit: Decimal,
    pub credit: Decimal,
    pub balance: Decimal,
}
