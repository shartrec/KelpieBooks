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

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
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

#[derive(Debug, Display, EnumString, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
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

impl SystemTag {}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub date: NaiveDate,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalEntry {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub account_id: Uuid,
    pub debit: i64,
    pub credit: i64,
    pub description: Option<String>,
}
