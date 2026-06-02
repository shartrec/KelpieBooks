/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use serde::{Deserialize, Serialize};

pub mod account_with_balance;
pub mod aged_payable_summary;
pub mod dashboard;
pub mod expense_breakdown;
pub mod general_ledger_line;
pub mod journal_entry_detail;
pub mod journal_entry_with_balance;
pub mod organization;
pub mod partner_list_item;
pub mod recent_transaction;
pub mod top_payable;
pub mod transaction_detail;
pub mod user_detail;
pub mod vendor_invoice_list_item;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiErrorMessage {
    pub error: String,
}

