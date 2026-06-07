/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Display, EnumString, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[strum(serialize_all = "PascalCase")]
pub enum AccountCategory {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}
