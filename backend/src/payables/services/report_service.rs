/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use std::collections::HashMap;

use chrono::NaiveDate;
use rust_decimal::dec;
use shared_core::payables::{
    dtos::aged_payable_summary::AgedPayableSummary,
    models::invoice_status::InvoiceStatus,
};
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    payables::db::vendor_invoice::get_by_org,
    util::ApiError,
};

pub(crate) async fn get_aged_payables(
    pool: &mut PgConnection,
    organization_id: Uuid,
    date: NaiveDate,
) -> Result<Vec<AgedPayableSummary>, ApiError> {
    let invoices = get_by_org(
        pool,
        organization_id,
        None,
        None,
        None,
        None,
        vec![&InvoiceStatus::Open, &InvoiceStatus::PartiallyPaid],
    )
    .await?;

    let mut summary_map: HashMap<Uuid, AgedPayableSummary> = HashMap::new();

    for invoice in invoices {
        let summary = summary_map
            .entry(invoice.partner_id)
            .or_insert_with(|| AgedPayableSummary {
                partner_id: invoice.partner_id,
                partner_name: invoice.partner_name.clone(),
                current: dec!(0.00),
                days_30: dec!(0.00),
                days_60: dec!(0.00),
                days_90: dec!(0.00),
                days_90_plus: dec!(0.00),
                total: dec!(0.00),
                invoices: Vec::new(),
            });

        let days_overdue = (date - invoice.due_date).num_days();

        if days_overdue <= 0 {
            summary.current += invoice.amount_remaining;
        } else if days_overdue <= 30 {
            summary.days_30 += invoice.amount_remaining;
        } else if days_overdue <= 60 {
            summary.days_60 += invoice.amount_remaining;
        } else if days_overdue <= 90 {
            summary.days_90 += invoice.amount_remaining;
        } else {
            summary.days_90_plus += invoice.amount_remaining;
        }
        summary.total += invoice.amount_remaining;
        summary.invoices.push(invoice);
    }

    Ok(summary_map.into_values().collect())
}
