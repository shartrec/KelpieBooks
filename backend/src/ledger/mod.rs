/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

// backend/src/ledger/mod.rs

#[cfg(feature = "ledger")]
pub(crate) mod db;
#[cfg(feature = "ledger")]
pub(crate) mod routes;
#[cfg(feature = "ledger")]
pub(crate) mod services;
pub(crate) mod reports;
