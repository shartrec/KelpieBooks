/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use serde::{
    Deserialize,
    Serialize,
};

use crate::sales::models::{
    order_address::OrderAddress,
    sales_order::{
        SalesOrder,
        SalesOrderItem,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SalesOrderDto {
    pub order: SalesOrder,

    pub bill_to: OrderAddress,
    pub ship_to: OrderAddress,

    pub items: Vec<SalesOrderItem>,
}
