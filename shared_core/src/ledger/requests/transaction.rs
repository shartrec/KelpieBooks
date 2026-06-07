/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::{
    NaiveDate,
    Utc,
};
use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct JournalEntryLine {
    #[serde(default = "Uuid::new_v4")]
    pub line_id: Uuid, // A unique ID for the frontend to use as a key
    pub account_id: Uuid,
    pub debit: i64,
    pub credit: i64,
    pub description: Option<String>,
}

// Custom default to ensure a new UUID is generated each time
impl Default for JournalEntryLine {
    fn default() -> Self {
        Self {
            line_id: Uuid::new_v4(),
            account_id: Uuid::nil(),
            debit: 0,
            credit: 0,
            description: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateTransactionRequest {
    pub date: NaiveDate,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub entries: Vec<JournalEntryLine>,
}

impl Default for CreateTransactionRequest {
    fn default() -> Self {
        Self {
            date: Utc::now().date_naive(),
            description: None,
            reference: None,
            entries: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReverseTransactionRequest {
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateTransactionRequest {
    pub date: NaiveDate,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub entries: Vec<JournalEntryLine>,
}
