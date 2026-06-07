/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::{
    DateTime,
    NaiveDate,
    Utc,
};
use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub date: NaiveDate,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub created_at: DateTime<Utc>,
}
