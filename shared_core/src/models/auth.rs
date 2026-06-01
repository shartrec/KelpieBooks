/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, EnumString, IntoEnumIterator};

#[derive(Debug, Clone, Copy, EnumString, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(type_name = "system_privilege", rename_all = "snake_case"))]
#[derive(AsRefStr)]
pub enum SystemPrivilege {
    org_admin,
    use_accounts,
    manage_accounts,
    use_partners,
    manage_partners,
    use_transactions,
    manage_transactions,
    manage_users,
    manage_organization,
}

impl SystemPrivilege {
    /// Returns the unique translation identifier key used inside Fluent .ftl localization files
    pub fn iterator() -> impl Iterator<Item = Self> {
        Self::iter()
    }

    pub fn name_key(&self) -> String {
        format!("sys-privilege-{}-name", self.as_ref())
    }

    /// Returns the description identifier key for verbose tooltips or help panels
    pub fn description_key(&self) -> String {
        format!("sys-privilege-{}-description", self.as_ref())
    }

    /// Converts variant to its snake_case database value string representation for JWT payloads
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}