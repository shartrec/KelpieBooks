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
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SalesInvoiceLine {
    pub id: Uuid,
    pub invoice_id: Uuid,
    pub item_id: Uuid,
    pub name: String,
    pub description: String,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub tax_category_id: Option<Uuid>,
    pub tax_rate: Decimal, // Added tax_rate field
    pub tax_amount: Decimal,
    pub line_total: Decimal,
    pub sort_order: i32,
}