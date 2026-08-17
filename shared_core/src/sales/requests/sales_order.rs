/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::NaiveDate;
use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

use crate::sales::models::{
    invoice_address::InvoiceAddress,
    sales_order_item::SalesOrderItem,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSalesOrderRequest {
    pub partner_id: Uuid,
    pub warehouse_id: Uuid,
    pub order_date: NaiveDate,
    pub due_date: NaiveDate,
    pub lines: Vec<SalesOrderItem>,

    // Optional references to saved partner addresses used to populate the snapshots
    pub billing_address_id: Option<Uuid>,
    pub shipping_address_id: Option<Uuid>,

    // Snapshots stored on the order (overridable by user per-order)
    pub bill_to: InvoiceAddress,
    pub ship_to: InvoiceAddress,
}
