/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use rust_decimal::{
    dec,
    Decimal,
};
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
    pub total_assets: Decimal,
    pub total_liabilities: Decimal,
    pub total_equity: Decimal,
    pub net_income: Decimal,
}

impl Default for BalanceSheet {
    fn default() -> Self {
        Self {
            assets: vec![],
            liabilities: vec![],
            equity: vec![],
            total_assets: dec!(0.00),
            total_liabilities: dec!(0.00),
            total_equity: dec!(0.00),
            net_income: dec!(0.00),
        }
    }
}
