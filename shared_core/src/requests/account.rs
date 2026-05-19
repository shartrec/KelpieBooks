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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccountRequest {
    pub name: String,
    pub code: String,
    pub category: AccountCategory,
    pub parent_id: Option<Uuid>,
    pub is_group: bool,
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
    pub system_tag: Option<SystemTag>,
}
