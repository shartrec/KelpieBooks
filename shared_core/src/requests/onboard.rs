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

/// Represents the data required to create a new organization and its first user.
#[derive(Debug, Serialize, Deserialize)]
pub struct OnboardingRequest {
    pub organization_name: String,
    pub user_email: String,
    pub user_password: String,
    pub user_full_name: String,
    pub user_display_name: Option<String>,
    pub coa_template_id: String,
}
