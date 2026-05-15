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

pub mod user_with_org;

pub use user_with_org::UserWithOrg;

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub strict_audit_mode: bool,
    pub created_at: DateTime<Utc>,
    pub locked_until: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub email: String,
    pub full_name: String,
    pub display_name: Option<String>,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Display, EnumString, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[strum(serialize_all = "PascalCase")]
pub enum AccountCategory {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

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


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Account {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub code: String,
    pub name: String,
    pub category: AccountCategory,
    pub is_group: bool,
    pub system_tag: Option<SystemTag>,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub date: NaiveDate,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct JournalEntry {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub account_id: Uuid,
    pub debit: i64,
    pub credit: i64,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}
