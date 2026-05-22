/*
 * Copyright (c) 2026-2026. Trevor Campbell and others.
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

use crate::models::account_category::AccountCategory;
use crate::models::system_tag::SystemTag;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A DTO that combines account data with its calculated balance.
/// This is the structure that will be sent to the frontend.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountWithBalance {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub code: String,
    pub name: String,
    pub category: AccountCategory,
    pub is_group: bool,
    pub is_bank_account: bool,
    pub system_tag: Option<SystemTag>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub balance: i64,
}

impl AccountWithBalance {
    pub fn calculate_totals(accounts: &[Self]) -> (i64, i64) {
        let mut debit_sum = 0;
        let mut credit_sum = 0;

        for acc in accounts.iter() {
            if !acc.is_group {
                match acc.category {
                    AccountCategory::Asset | AccountCategory::Expense => {
                        if acc.balance >= 0 {
                            debit_sum += acc.balance;
                        } else {
                            credit_sum += acc.balance.abs();
                        }
                    }
                    AccountCategory::Liability
                    | AccountCategory::Equity
                    | AccountCategory::Revenue => {
                        if acc.balance <= 0 {
                            credit_sum += acc.balance.abs();
                        } else {
                            debit_sum += acc.balance;
                        }
                    }
                }
            }
        }
        (debit_sum, credit_sum)
    }
}
