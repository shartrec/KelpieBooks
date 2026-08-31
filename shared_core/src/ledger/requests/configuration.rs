/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use std::collections::HashMap;

use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    ledger::models::system_tag::SystemTag,
    AccountId,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateConfigurationRequest {
    pub system_accounts: HashMap<SystemTag, AccountId>,
    pub strict_audit_mode: bool,
}
