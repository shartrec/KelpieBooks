/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 * (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use rust_decimal::Decimal;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    ItemId,
    LocationEntryId,
    PartnerId,
    WarehouseId,
};
// =============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SequenceType {
    Numeric,
    Alphabetic,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NumericRange {
    pub start: i32,
    pub end: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlphaRange {
    pub start: String, // e.g., "A"
    pub end: String,   // e.g., "C"
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdjustmentReason {
    CycleCount,
    Damage,
    Scrap,
    AuditCorrection,
    FoundStock,
    Other,
}

// =============================================================================
// 1. Bulk Location Generator DTOs (Request)
// =============================================================================
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    pub item_id: ItemId,
    pub item_code: String,
    pub item_name: String,
    pub uom_code: String,
    pub quantity_on_hand: Decimal,
    pub quantity_allocated: Decimal,
    pub quantity_available: Decimal, // Calculated via (on_hand - allocated)
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LocationContentsResponse {
    pub location_id: LocationEntryId,
    pub display_label: String,
    pub warehouse_code: String,
    pub is_picking_location: bool,
    pub contents: Vec<LocationContentItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct ItemLocationBalanceDto {
    pub warehouse_id: WarehouseId,
    pub warehouse_code: String,
    pub warehouse_name: String,
    pub location_id: LocationEntryId,
    pub location_display_label: String,
    pub is_picking_location: bool,
    pub quantity_on_hand: Option<Decimal>,
    pub quantity_allocated: Option<Decimal>,
    pub quantity_available: Option<Decimal>, // Calculated: on_hand - allocated
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemStockBalancesResponse {
    pub item_id: ItemId,
    pub total_on_hand: Option<Decimal>,
    pub total_allocated: Option<Decimal>,
    pub total_available: Option<Decimal>,
    pub location_balances: Vec<ItemLocationBalanceDto>,
}

// =============================================================================

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TransferItemLine {
    pub item_id: ItemId,
    pub quantity: Decimal,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct InterLocationTransferRequest {
    pub source_location_id: LocationEntryId,
    pub destination_location_id: LocationEntryId,
    pub items_to_move: Vec<TransferItemLine>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiveItemLine {
    pub item_id: ItemId,
    pub location_id: LocationEntryId,
    pub quantity: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiveStockRequest {
    pub warehouse_id: WarehouseId,
    pub vendor_id: Option<PartnerId>,
    pub po_number: Option<String>,
    pub notes: Option<String>,
    pub items: Vec<ReceiveItemLine>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdjustStockItemLine {
    pub location_id: LocationEntryId,
    pub item_id: ItemId,
    /// Can be positive (increase stock) or negative (decrease stock)
    pub quantity_delta: Decimal,
    pub reason: AdjustmentReason,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockAdjustmentRequest {
    pub warehouse_id: WarehouseId,
    pub items: Vec<AdjustStockItemLine>,
}
