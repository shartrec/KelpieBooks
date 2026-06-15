/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct TaxCategory {
    pub id: Uuid,
    #[cfg_attr(feature = "backend", sqlx(rename = "organization_id"))]
    pub org_id: Uuid,
    pub name: String,
    pub rate: Decimal, // Stored as a scaled integer, e.g., 1000 = 10.00%
    pub is_active: bool,
}
