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

use crate::core::models::role::Role;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserWithOrg {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub email: String,
    pub full_name: String,
    pub display_name: Option<String>,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub organisation_name: String,
    pub strict_audit_mode: bool,
    pub role: Option<Role>,
}
