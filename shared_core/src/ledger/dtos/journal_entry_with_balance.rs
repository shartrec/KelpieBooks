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
use uuid::Uuid;

/// A DTO representing a journal entry with its running balance at that point in time.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JournalEntryWithBalance {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub account_id: Uuid,
    pub date: NaiveDate,
    pub description: Option<String>,
    pub debit: Decimal,
    pub credit: Decimal,
    pub running_balance: Decimal,
}
