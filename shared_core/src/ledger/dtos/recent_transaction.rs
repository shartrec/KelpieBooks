/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::NaiveDate;
use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecentTransaction {
    pub id: Uuid,
    pub account_id: Uuid,
    pub date: NaiveDate,
    pub description: String,
    pub amount: i64,
}
