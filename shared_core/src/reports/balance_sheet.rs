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

use crate::dtos::account_with_balance::AccountWithBalance;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
