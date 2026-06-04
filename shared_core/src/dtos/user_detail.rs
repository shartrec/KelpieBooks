/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A Data Transfer Object representing the user details that are safe
/// to send to the frontend. This struct explicitly omits sensitive
/// information like the password hash.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserDetail {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    pub display_name: Option<String>,
    pub role: Option<String>,
    pub organization_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthUserDetail {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    pub display_name: Option<String>,
    pub role: Option<String>,
    pub organization_id: Uuid,
    pub privileges: Vec<String>
}
