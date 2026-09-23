/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use rust_decimal::Decimal;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    ItemId,
    LocationEntryId,
    OrderItemId,
};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct PickLineItem {
    pub order_line_id: OrderItemId,
    pub item_id: ItemId,
    pub name: String,
    pub code: String,
    pub location_id: Option<LocationEntryId>,
    pub location: Option<String>,
    pub uom: Option<String>,
    pub ordered_qty: Decimal,
    pub picked_qty: Decimal,
}
