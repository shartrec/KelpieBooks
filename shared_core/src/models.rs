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
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::Type, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[sqlx(type_name = "account_category")]
#[allow(non_camel_case_types)]
pub enum AccountCategory {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

#[derive(Debug, sqlx::Type, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[sqlx(type_name = "system_tag")]
#[allow(non_camel_case_types)]
pub enum SystemTag {
    CASH_AT_BANK,
    ACCOUNTS_RECEIVABLE,
    ACCOUNTS_PAYABLE,
    RETAINED_EARNINGS,
    SALES_TAX_PAYABLE,
    REVENUE,
    EXPENSE,
    COST_OF_GOODS_SOLD,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
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

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub date: NaiveDate,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct JournalEntry {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub account_id: Uuid,
    pub debit: i64,
    pub credit: i64,
    pub description: Option<String>,
}
