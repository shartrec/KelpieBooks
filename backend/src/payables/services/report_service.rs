/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use crate::payables::db::vendor_invoice::get_by_org;
use crate::util::ApiError;
use chrono::NaiveDate;
use shared_core::dtos::aged_payable_summary::AgedPayableSummary;
use shared_core::models::invoice_status::InvoiceStatus;
use sqlx::PgConnection;
use std::collections::HashMap;
use uuid::Uuid;

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
        Some(format!("{},{}", InvoiceStatus::Open.as_str(), InvoiceStatus::PartiallyPaid.as_str())),
    )
        .await?;

    let mut summary_map: HashMap<Uuid, AgedPayableSummary> = HashMap::new();

    for invoice in invoices {
        let summary = summary_map
            .entry(invoice.partner_id)
            .or_insert_with(|| AgedPayableSummary {
                partner_id: invoice.partner_id,
                partner_name: invoice.partner_name.clone(),
                current: 0,
                days_30: 0,
                days_60: 0,
                days_90: 0,
                days_90_plus: 0,
                total: 0,
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
