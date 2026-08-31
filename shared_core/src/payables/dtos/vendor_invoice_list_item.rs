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

use crate::{
    payables::models::invoice_status::InvoiceStatus,
    InvoiceId,
    PartnerId,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[cfg_attr(feature = "backend", sqlx(rename_all = "snake_case"))]
pub struct VendorInvoiceListItem {
    pub id: InvoiceId,
    pub partner_id: PartnerId,
    pub partner_name: String,
    pub invoice_number: String,
    pub issue_date: NaiveDate,
    pub due_date: NaiveDate,
    pub net_amount: Decimal,
    pub tax_amount: Decimal,
    pub gross_amount: Decimal,
    pub amount_remaining: Decimal,
    pub status: InvoiceStatus,
}
