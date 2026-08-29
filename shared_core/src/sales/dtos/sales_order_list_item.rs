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
use crate::PartnerId;
use crate::sales::models::{
    fulfillment_status::FulfillmentStatus,
    payment_status::PaymentStatus,
    sales_document_status::SalesDocumentStatus,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SalesOrderListItem {
    pub id: Uuid,
    pub order_number: String,
    pub partner_id: PartnerId,
    pub partner_name: String,
    pub order_date: NaiveDate,
    pub warehouse_name: String,
    pub fulfillment_status: FulfillmentStatus,
    pub payment_status: PaymentStatus,
    pub document_status: SalesDocumentStatus,
    pub due_date: NaiveDate,
    pub subtotal: Decimal,
    pub tax_total: Decimal,
    pub total_amount: Decimal,
    pub amount_remaining: Decimal,
}
