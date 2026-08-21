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
use strum::{
    Display,
    EnumIter,
    EnumString,
};

/// Tracks physical stock dispatch / delivery state
#[derive(
    Debug, Display, EnumString, EnumIter, Serialize, Deserialize, PartialEq, Eq, Clone, Copy,
)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(
    feature = "backend",
    sqlx(type_name = "fulfillment_status", rename_all = "snake_case")
)]
pub enum FulfillmentStatus {
    Unfulfilled,        // Nothing shipped yet / Service pending
    PartiallyFulfilled, // Partial shipment
    Fulfilled,          // Shipped / Delivered / Service Completed
    NotRequired,        // Pure digital/service lines with no delivery step
}

impl Default for FulfillmentStatus {
    fn default() -> Self {
        Self::Unfulfilled
    }
}
