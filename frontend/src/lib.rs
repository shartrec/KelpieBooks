/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use serde::Deserialize;

#[cfg(feature = "inventory")]
pub mod inventory;
#[cfg(feature = "ledger")]
pub mod ledger;
#[cfg(feature = "partners")]
pub mod partners;
#[cfg(feature = "payables")]
pub mod payables;
#[cfg(feature = "sales")]
pub mod sales;

pub mod api;
pub mod contexts;
pub mod core;
pub mod router;
pub mod services;

#[derive(Deserialize)]
pub struct BackendError {
    pub error: String,
}