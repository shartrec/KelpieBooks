/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 * (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// =============================================================================
// 1. Bulk Location Generator DTOs (Request)
// =============================================================================

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SequenceType {
    Numeric,
    Alphabetic,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct NumericRange {
    pub start: i32,
    pub end: i32,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AlphaRange {
    pub start: String, // e.g., "A"
    pub end: String,   // e.g., "C"
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct BulkLocationGenerateRequest {
    pub zone: String,
    pub is_picking_location: bool,
    pub naming_format: String, // e.g., "{zone}-{aisle}-{shelf}-{bin}"

    // Ranges for coordinate loops
    pub aisles: Option<NumericRange>,
    pub shelves: Option<AlphaRange>,
    pub bins: Option<NumericRange>,
}

// =============================================================================
// 2. Location-Centric Contents DTOs (Response)
// =============================================================================

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LocationContentItem {
    pub item_id: Uuid,
    pub item_code: String,
    pub item_name: String,
    pub uom_code: String,
    pub quantity_on_hand: Decimal,
    pub quantity_allocated: Decimal,
    pub quantity_available: Decimal, // Calculated via (on_hand - allocated)
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LocationContentsResponse {
    pub location_id: Uuid,
    pub display_label: String,
    pub warehouse_code: String,
    pub is_picking_location: bool,
    pub contents: Vec<LocationContentItem>,
}

// =============================================================================
// 3. Inter-Location Stock Transfer DTOs (Request)
// =============================================================================

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TransferItemLine {
    pub item_id: Uuid,
    pub quantity: Decimal,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct InterLocationTransferRequest {
    pub source_location_id: Uuid,
    pub destination_location_id: Uuid,
    pub items_to_move: Vec<TransferItemLine>,
}