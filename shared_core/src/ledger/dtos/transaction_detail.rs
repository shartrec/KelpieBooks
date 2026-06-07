/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::ledger::dtos::journal_entry_detail::JournalEntryDetail;
use crate::ledger::models::transaction::Transaction;
use serde::{Deserialize, Serialize};

/// A DTO representing a full transaction with all its journal entry lines.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransactionDetail {
    #[serde(flatten)]
    pub transaction: Transaction,
    pub entries: Vec<JournalEntryDetail>,
}
