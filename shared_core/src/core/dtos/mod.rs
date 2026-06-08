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

pub mod organization;
pub mod user_detail;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiErrorMessage {
    pub error: String,
}
