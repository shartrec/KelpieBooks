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

pub use crate::sales::models::{
    sales_order_item::SalesOrderItem,
};
use crate::sales::models::fulfillment_status::FulfillmentStatus;
use crate::sales::models::payment_status::PaymentStatus;
use crate::sales::models::sales_document_status::SalesDocumentStatus;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SalesOrder {
    pub id: Uuid,
    pub org_id: Uuid,
    pub partner_id: Uuid,
    pub warehouse_id: Uuid,
    pub warehouse_name: Option<String>,
    pub order_number: String,
    pub order_date: NaiveDate,
    pub due_date: NaiveDate,
    pub fulfillment_status: FulfillmentStatus,
    pub payment_status: PaymentStatus,
    pub document_status: SalesDocumentStatus,
    pub subtotal: Decimal,
    pub tax_total: Decimal,
    pub total_amount: Decimal,
    pub amount_remaining: Decimal,
    // Optional references to saved partner addresses used to populate the snapshots
    pub billing_address_id: Option<Uuid>,
    pub shipping_address_id: Option<Uuid>,
}

impl SalesOrder {
    pub fn calculate(&mut self, lines: &Vec<SalesOrderItem>) {

        let amount_paid = self.total_amount - self.amount_remaining;

        let mut net_amount = Decimal::ZERO;
        let mut tax_amount = Decimal::ZERO;

        for line in lines {
            net_amount += line.net_amount;
            tax_amount += line.tax_amount;
        }

        self.subtotal = net_amount;
        self.tax_total = tax_amount;
        self.total_amount = net_amount + tax_amount;

        self.amount_remaining = self.total_amount - amount_paid;
    }
}
