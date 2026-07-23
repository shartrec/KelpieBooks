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
use shared_core::sales::{
    dtos::aged_receivable_summary::AgedReceivableSummary,
    models::invoice_status::InvoiceStatus,
};
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    sales::db::sales_invoice::list_sales_invoices,
    util::ApiError,
};

pub(crate) async fn get_trial_balance(
    pool: &mut PgConnection,
    organization_id: Uuid,
    date: NaiveDate,
) -> Result<Vec<AgedReceivableSummary>, ApiError> {
    let invoices = list_sales_invoices(
        pool,
        organization_id,
        None,
        None,
        None,
        None,
        Some(vec![InvoiceStatus::Draft, InvoiceStatus::Open]),
    )
    .await?;

    let mut summary_map: HashMap<Uuid, AgedReceivableSummary> = HashMap::new();

    for invoice in invoices {
        let summary =
            summary_map
                .entry(invoice.partner_id)
                .or_insert_with(|| AgedReceivableSummary {
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
