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
use rust_decimal::Decimal;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    ItemId,
    LocationEntryId,
    OrderId,
    OrderItemId,
    ShipmentId,
    WarehouseId,
};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Shipment {
    pub id: ShipmentId,
    pub order_id: OrderId,
    pub order_number: String,
    pub shipment_number: String,
    pub carrier: String,
    pub tracking_number: String,
    pub shipped_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub lines: Vec<ShipmentItem>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ShipmentItem {
    pub id: ShipmentId,
    pub warehouse_id: WarehouseId,
    pub location_id: LocationEntryId,
    pub order_line_id: OrderItemId,
    pub item_id: ItemId,
    pub qty: Decimal,
}
