/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::{
    DateTime,
    Utc,
};
use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

use crate::ledger::models::{
    account_category::AccountCategory,
    system_tag::SystemTag,
};
use crate::OrgId;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[cfg_attr(
    feature = "backend",
    sqlx(type_name = "accounts", rename_all = "snake_case")
)]
pub struct Account {
    pub id: Uuid,
    pub organization_id: OrgId,
    pub parent_id: Option<Uuid>,
    pub code: String,
    pub name: String,
    pub category: AccountCategory,
    pub is_group: bool,
    pub is_bank_account: bool,
    pub system_tag: Option<SystemTag>,
    pub created_at: DateTime<Utc>,
}
