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
use chrono::{NaiveDate, Utc};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct JournalEntryLine {
    pub account_id: Uuid,
    pub debit: i64,
    pub credit: i64,
    pub description: Option<String>,
}
impl Default for JournalEntryLine {
    fn default() -> Self {
        Self {
            account_id: Uuid::nil(),
            debit: 0,
            credit: 0,
            description: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateTransactionRequest {
    pub date: NaiveDate,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub entries: Vec<JournalEntryLine>,
}

impl Default for CreateTransactionRequest {
    fn default() -> Self {
        Self {
            date: Utc::now().date_naive(),
            description: None,
            reference: None,
            entries: vec![],
        }
    }
}

