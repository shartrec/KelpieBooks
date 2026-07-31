/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use rust_decimal::Decimal;
use serde::Deserialize;
use uuid::Uuid;

use crate::sales::models::item::ItemType;

#[derive(Debug, Clone, Deserialize)]
pub struct ItemPayload {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub item_type: ItemType,
    pub uom_id: Uuid,
    pub unit_price: Decimal, // 💡 Scaled to 4 decimal places internally
    pub income_account_id: Uuid,
    pub tax_category_id: Option<Uuid>,
    pub is_active: bool,
}
