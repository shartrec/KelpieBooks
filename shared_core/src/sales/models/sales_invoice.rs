/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::{
    NaiveDate,
};
use rust_decimal::Decimal;
use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

use crate::sales::models::{
    invoice_status::InvoiceStatus,
    sales_invoice_item::SalesInvoiceLine,
};
use crate::sales::models::invoice_address::InvoiceAddress;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SalesInvoice {
    pub id: Uuid,
    pub org_id: Uuid,
    pub partner_id: Uuid,
    pub invoice_number: String,
    pub issue_date: NaiveDate,
    pub due_date: NaiveDate,
    pub status: InvoiceStatus,
    pub subtotal: Decimal,
    pub tax_total: Decimal,
    pub total_amount: Decimal,
    pub amount_due: Decimal,
    // Optional references to saved partner addresses used to populate the snapshots
    pub billing_address_id: Option<Uuid>,
    pub shipping_address_id: Option<Uuid>,

    // Snapshots stored on the invoice (overridable by user per-invoice)
    pub bill_to: InvoiceAddress,
    pub ship_to: InvoiceAddress,
    pub lines: Vec<SalesInvoiceLine>,
}

impl SalesInvoice {
    pub fn calculate(&mut self) {
        let mut gross_amount = Decimal::ZERO;
        let mut tax_amount = Decimal::ZERO;

        for line in &mut self.lines {
            gross_amount += line.line_total;
            tax_amount += line.tax_amount;
        }

        self.total_amount = gross_amount;
        self.tax_total = tax_amount;
        self.subtotal = gross_amount - tax_amount;
    }
}
