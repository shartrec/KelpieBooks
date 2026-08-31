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
use crate::{AccountId, TaxCategoryId, UomId};
use crate::sales::models::item::ItemType;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateItemRequest {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub item_type: ItemType,
    pub uom_id: UomId,
    pub unit_price: Decimal,
    pub unit_cost: Decimal,
    pub income_account_id: AccountId,
    pub tax_category_id: Option<TaxCategoryId>,
}
