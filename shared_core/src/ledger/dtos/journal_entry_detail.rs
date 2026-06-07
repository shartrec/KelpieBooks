/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::DateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A DTO representing a journal entry line, including the name of the account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JournalEntryDetail {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub account_id: Uuid,
    pub account_name: String, // The joined account name
    pub debit: i64,
    pub credit: i64,
    pub description: Option<String>,
    pub created_at: DateTime<chrono::Utc>,
}
