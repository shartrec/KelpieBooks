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

use crate::core::models::auth::SystemPrivilege;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateRoleRequest {
    pub name: String,
    pub privileges: Vec<SystemPrivilege>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRoleRequest {
    pub name: String,
    pub privileges: Vec<SystemPrivilege>,
}
