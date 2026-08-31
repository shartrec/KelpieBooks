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

use crate::{
    OrgId,
    RoleId,
    UserId,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub organization_id: OrgId,
    pub email: String,
    pub full_name: String,
    pub display_name: Option<String>,
    pub password_hash: String,
    pub role_id: Option<RoleId>,
    pub created_at: DateTime<Utc>,
}
