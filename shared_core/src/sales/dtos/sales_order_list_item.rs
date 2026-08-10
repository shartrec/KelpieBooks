/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

use crate::sales::models::sales_order_status::SalesOrderStatus;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SalesOrderListItem {
    pub id: Uuid,
    pub order_number: String,
    pub partner_name: String,
    pub order_date: NaiveDate,
    pub warehouse_name: String,
    pub status: SalesOrderStatus,
    pub total_amount: Decimal,
}
