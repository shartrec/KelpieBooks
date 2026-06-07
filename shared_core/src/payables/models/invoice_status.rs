/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

#[derive(
    Debug, Display, EnumString, EnumIter, Serialize, Deserialize, PartialEq, Eq, Clone, Copy,
)]
#[strum(serialize_all = "PascalCase")]
pub enum InvoiceStatus {
    Open,
    Paid,
    PartiallyPaid,
    Void,
}

impl InvoiceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Open => "Open",
            Self::Paid => "Paid",
            Self::PartiallyPaid => "PartiallyPaid",
            Self::Void => "Void",
        }
    }
}

impl Default for InvoiceStatus {
    fn default() -> Self {
        Self::Open
    }
}
