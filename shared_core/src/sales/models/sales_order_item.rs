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
use crate::{ItemId, OrderId, OrderItemId, TaxCategoryId};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct SalesOrderItem {
    pub id: OrderItemId,
    pub order_id: OrderId,
    pub item_id: ItemId,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub tax_category_id: Option<TaxCategoryId>,
    pub tax_rate: Decimal,
    pub tax_amount: Decimal,
    pub net_amount: Decimal,
    pub sort_order: i32,
    /// Computed at read time from warehouse stock balances — not stored in the database.
    #[cfg_attr(feature = "backend", sqlx(default))]
    pub quantity_available: Option<Decimal>,
}
