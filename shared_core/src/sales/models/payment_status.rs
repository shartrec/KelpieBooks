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
/// Tracks financial settlement & Accounts Receivable status
#[derive(
    Debug, Display, EnumString, EnumIter, Serialize, Deserialize, PartialEq, Eq, Clone, Copy,
)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(
    feature = "backend",
    sqlx(type_name = "payment_status", rename_all = "snake_case")
)]
pub enum PaymentStatus {
    Unpaid,        // Invoice issued, $0 received
    PartiallyPaid, // Deposit or partial payment received
    Paid,          // Fully settled
    Refunded,      // Voided / Returned
}

impl Default for PaymentStatus {
    fn default() -> Self {
        Self::Unpaid
    }
}