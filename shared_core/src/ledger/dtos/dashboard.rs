/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use rust_decimal::Decimal;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FinancialHealth {
    pub net_profit_ytd: Decimal,
    pub bank_balance: Decimal,
    pub accounts_receivable: Decimal,
    pub accounts_payable: Decimal,
}
