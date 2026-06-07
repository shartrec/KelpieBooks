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
use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct JournalEntry {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub account_id: Uuid,
    pub debit: i64,
    pub credit: i64,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}
