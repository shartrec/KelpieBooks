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

use crate::OrgId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Organization {
    pub id: OrgId,
    pub name: String,
    pub strict_audit_mode: bool,
    pub created_at: DateTime<Utc>,
    pub locked_until: Option<NaiveDate>,
}
