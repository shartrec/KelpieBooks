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
use uuid::Uuid;
use chrono::DateTime;

/// A DTO representing a journal entry line, including the name of the account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JournalEntryDetail {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub account_id: Uuid,
    pub account_name: String, // The joined account name
    pub debit: i64,
    pub credit: i64,
    pub description: Option<String>,
    pub created_at: DateTime<chrono::Utc>,
}
