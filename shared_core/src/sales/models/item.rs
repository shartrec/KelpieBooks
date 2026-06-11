/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(
    feature = "backend",
    sqlx(type_name = "item_type_enum", rename_all = "snake_case")
)]
pub enum ItemType {
    Stocked,
    NonStocked,
    Service,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct UnitOfMeasure {
    pub id: Uuid,
    pub code: String,  // e.g., "EA", "HR"
    pub name: String,  // e.g., "Each", "Hour"
    pub is_active: bool,
}

// 💡 The master model representing your item row
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct Item {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub item_type: ItemType,
    pub uom_id: Uuid,                        // 💡 Linked Unit of Measure ID
    pub unit_price: i64,                    // 💡 Scaled to 4 decimal places (e.g. 1245 = $0.1245)
    pub income_account_id: Uuid,
    pub tax_category_id: Option<Uuid>,
    pub is_active: bool,
    pub created_at: Option<DateTime<Utc>>,
}
