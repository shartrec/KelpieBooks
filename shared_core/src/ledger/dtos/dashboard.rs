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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FinancialHealth {
    pub net_profit_ytd: i64,
    pub bank_balance: i64,
    pub accounts_receivable: i64,
    pub accounts_payable: i64,
}
