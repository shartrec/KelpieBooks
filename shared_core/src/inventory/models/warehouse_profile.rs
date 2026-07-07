/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 * (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// =============================================================================
// 1. Warehouse Physical Profile Extensions
// =============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct ItemWarehouseProfile {
    pub item_id: Uuid,
    #[cfg_attr(feature = "backend", sqlx(rename = "organization_id"))]
    pub org_id: Uuid,
    pub weight_kg: Decimal,
    pub length_cm: Decimal,
    pub width_cm: Decimal,
    pub height_cm: Decimal,
    pub reorder_point: Decimal,
    pub safety_stock: Decimal,
    pub updated_at: Option<DateTime<Utc>>,
}

// =============================================================================
// 3. Stock Ledger Balances
// =============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct WarehouseInventoryBalance {
    pub id: Uuid,
    #[cfg_attr(feature = "backend", sqlx(rename = "organization_id"))]
    pub org_id: Uuid,
    pub item_id: Uuid,
    pub warehouse_id: Uuid,
    pub location_id: Uuid,
    pub quantity_on_hand: Decimal,
    pub quantity_allocated: Decimal,
    pub updated_at: Option<DateTime<Utc>>,
}

