/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use chrono::{
    DateTime,
    Utc,
};
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    LocationEntryId,
    OrgId,
    WarehouseId,
};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct Warehouse {
    pub id: WarehouseId,
    pub organization_id: OrgId,
    pub code: String, // e.g., "WH-SYD"
    pub name: String,
    pub is_active: bool,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct WarehouseLocation {
    pub id: LocationEntryId,
    pub organization_id: OrgId,
    pub warehouse_id: WarehouseId,
    pub zone: Option<String>,  // e.g., "Bulk"
    pub aisle: Option<String>, // e.g., "A1"
    pub shelf: Option<String>, // e.g., "S3"
    pub bin: Option<String>,   // e.g., "B02"
    pub display_label: String, // e.g., "A1-S3-B02"
    pub is_picking_location: bool,
    pub created_at: Option<DateTime<Utc>>,
}
