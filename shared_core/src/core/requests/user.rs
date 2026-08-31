/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use serde::{
    Deserialize,
    Serialize,
};

use crate::RoleId;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub full_name: String,
    pub display_name: Option<String>,
    pub password: String,
    pub role_id: Option<RoleId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub email: String,
    pub full_name: String,
    pub display_name: Option<String>,
    pub role_id: Option<RoleId>,
}
