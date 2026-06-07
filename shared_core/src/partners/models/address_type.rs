/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

#[derive(
    Debug, Display, EnumString, EnumIter, Serialize, Deserialize, PartialEq, Eq, Clone, Copy,
)]
#[strum(serialize_all = "PascalCase")]
pub enum AddressType {
    Billing,
    Shipping,
    General,
}

impl AddressType {
    pub fn iterator() -> impl Iterator<Item = Self> {
        Self::iter()
    }
    pub fn display_name(&self) -> String {
        match self {
            AddressType::Billing => "Billing".to_string(),
            AddressType::Shipping => "Shipping".to_string(),
            AddressType::General => "General".to_string(),
        }
    }
}
impl Default for AddressType {
    fn default() -> Self {
        Self::General
    }
}
