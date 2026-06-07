/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeneralLedgerLine {
    pub transaction_id: Uuid,
    pub journal_entry_id: Uuid,
    pub date: NaiveDate,
    pub account_id: Uuid,
    pub account_name: String,
    pub description: Option<String>,
    pub debit: i64,
    pub credit: i64,
    pub balance: i64,
}
