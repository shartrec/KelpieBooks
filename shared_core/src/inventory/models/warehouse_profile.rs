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
use chrono::{
    DateTime,
    Utc,
};
use rust_decimal::Decimal;
use serde::{
    Deserialize,
    Serialize,
};
use crate::{ItemId, LocationEntryId, OrgId, WarehouseBalId, WarehouseId};
// =============================================================================
// 1. Warehouse Physical Profile Extensions
// =============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct ItemWarehouseProfile {
    pub item_id: ItemId,
    pub organization_id: OrgId,
    pub weight_kg: Option<Decimal>,
    pub length_cm: Option<Decimal>,
    pub width_cm: Option<Decimal>,
    pub height_cm: Option<Decimal>,
    pub reorder_point: Option<Decimal>,
    pub safety_stock: Option<Decimal>,
    pub updated_at: Option<DateTime<Utc>>,
}

// =============================================================================
// 3. Stock Ledger Balances
// =============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct WarehouseInventoryBalance {
    pub id: WarehouseBalId,
    pub organization_id: OrgId,
    pub item_id: ItemId,
    pub warehouse_id: WarehouseId,
    pub location_id: LocationEntryId,
    pub quantity_on_hand: Decimal,
    pub quantity_allocated: Decimal,
    pub unit_cost: Decimal,
    pub updated_at: Option<DateTime<Utc>>,
}
