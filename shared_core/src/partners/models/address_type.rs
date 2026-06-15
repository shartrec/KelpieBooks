/*
 * Copyright (c) 2024.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use serde::{
    Deserialize,
    Serialize,
};
use strum::{
    Display,
    EnumIter,
    EnumString,
};

#[derive(
    Debug, Display, EnumString, EnumIter, Serialize, Deserialize, PartialEq, Eq, Clone, Copy,
)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(
    feature = "backend",
    sqlx(type_name = "address_type", rename_all = "snake_case")
)]
pub enum AddressType {
    Billing,
    Shipping,
    General,
}

impl Default for AddressType {
    fn default() -> Self {
        Self::General
    }
}
