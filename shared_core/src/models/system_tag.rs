/*
 * Copyright (c) 2026. Trevor Campbell and others.
 *
 * This file is part of KelpieBooks.
 *
 * KelpieBooks is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License,or
 * (at your option) any later version.
 *
 * KelpieBooks is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with KelpieBooks; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 *
 * Contributors:
 *      Trevor Campbell
 *
 */

use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

#[derive(Debug, Display, EnumString, EnumIter, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Hash)]
#[strum(serialize_all = "PascalCase")]
pub enum SystemTag {
    CashAtBank,
    AccountsReceivable,
    AccountsPayable,
    RetainedEarnings,
    SalesTaxPayable,
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
            SystemTag::Revenue => "Revenue".to_string(),
            SystemTag::Expense => "Expense".to_string(),
            SystemTag::CostOfGoodsSold => "Cost of Goods Sold".to_string(),
        }
    }
}
