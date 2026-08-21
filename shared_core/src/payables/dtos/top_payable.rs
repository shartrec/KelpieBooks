/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[cfg_attr(feature = "backend", sqlx(rename_all = "snake_case"))]
pub struct TopPayable {
    pub partner_name: String,
    pub due_date: NaiveDate,
    pub amount: Decimal,
}
