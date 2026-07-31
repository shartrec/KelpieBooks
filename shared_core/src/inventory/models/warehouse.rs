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
use uuid::Uuid;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct Warehouse {
    pub id: Uuid,
    #[cfg_attr(feature = "backend", sqlx(rename = "organization_id"))]
    pub org_id: Uuid,
    pub code: String, // e.g., "WH-SYD"
    pub name: String,
    pub is_active: bool,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct WarehouseLocation {
    pub id: Uuid,
    #[cfg_attr(feature = "backend", sqlx(rename = "organization_id"))]
    pub org_id: Uuid,
    pub warehouse_id: Uuid,
    pub zone: String,          // e.g., "Bulk"
    pub aisle: String,         // e.g., "A1"
    pub shelf: String,         // e.g., "S3"
    pub bin: String,           // e.g., "B02"
    pub display_label: String, // e.g., "A1-S3-B02"
    pub is_picking_location: bool,
    pub created_at: Option<DateTime<Utc>>,
}
