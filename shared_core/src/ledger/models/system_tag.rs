/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

#[derive(
    Debug, Display, EnumString, EnumIter, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Hash,
)]
#[strum(serialize_all = "PascalCase")]
pub enum SystemTag {
    CashAtBank,
    AccountsReceivable,
    AccountsPayable,
    RetainedEarnings,
    SalesTaxPayable,
    SalesTaxClearing,
    Revenue,
    Expense,
    CostOfGoodsSold,
}

impl SystemTag {
    pub fn iterator() -> impl Iterator<Item = Self> {
        Self::iter()
    }

    pub fn display_name(&self) -> String {
        match self {
            SystemTag::CashAtBank => "Cash at Bank".to_string(),
            SystemTag::AccountsReceivable => "Accounts Receivable".to_string(),
            SystemTag::AccountsPayable => "Accounts Payable".to_string(),
            SystemTag::RetainedEarnings => "Retained Earnings".to_string(),
            SystemTag::SalesTaxPayable => "Sales Tax Payable".to_string(),
            SystemTag::SalesTaxClearing => "Tax Clearing".to_string(),
            SystemTag::Revenue => "Revenue".to_string(),
            SystemTag::Expense => "Expense".to_string(),
            SystemTag::CostOfGoodsSold => "Cost of Goods Sold".to_string(),
        }
    }
}
