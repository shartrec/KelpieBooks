/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use serde::{
    Deserialize,
    Serialize,
};

use crate::ledger::dtos::account_with_balance::AccountWithBalance;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BalanceSheet {
    pub assets: Vec<AccountWithBalance>,
    pub liabilities: Vec<AccountWithBalance>,
    pub equity: Vec<AccountWithBalance>,
    pub total_assets: i64,
    pub total_liabilities: i64,
    pub total_equity: i64,
    pub net_income: i64,
}

impl Default for BalanceSheet {
    fn default() -> Self {
        Self {
            assets: vec![],
            liabilities: vec![],
            equity: vec![],
            total_assets: 0,
            total_liabilities: 0,
            total_equity: 0,
            net_income: 0,
        }
    }
}
