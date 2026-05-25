/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::models::account_category::AccountCategory;
use crate::models::system_tag::SystemTag;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccountRequest {
    pub name: String,
    pub code: String,
    pub category: AccountCategory,
    pub parent_id: Option<Uuid>,
    pub is_group: bool,
    pub is_bank_account: bool,
    pub system_tag: Option<SystemTag>,
}
impl Default for CreateAccountRequest {
    fn default() -> Self {
        Self {
            name: String::new(),
            code: String::new(),
            category: AccountCategory::Asset,
            parent_id: None,
            is_group: false,
            is_bank_account: false,
            system_tag: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAccountRequest {
    pub name: String,
    pub code: String,
    pub category: AccountCategory,
    // parent_id is often excluded from simple updates due to complexity.
    pub is_group: bool,
    pub is_bank_account: bool,
    pub system_tag: Option<SystemTag>,
}
