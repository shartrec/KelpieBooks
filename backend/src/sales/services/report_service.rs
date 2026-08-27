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
    models::sales_document_status::SalesDocumentStatus,
};
use sqlx::PgConnection;
use uuid::Uuid;
use shared_core::OrgId;
use crate::{
    sales::db::sales_order::list_sales_orders,
    util::ApiError,
};

pub(crate) async fn get_trial_balance(
    pool: &mut PgConnection,
    organization_id: OrgId,
    date: NaiveDate,
) -> Result<Vec<AgedReceivableSummary>, ApiError> {
    let orders = list_sales_orders(
        pool,
        organization_id,
        None,
        None,
        None,
        None,
        Some(vec![SalesDocumentStatus::Draft, SalesDocumentStatus::Open]),
    )
    .await?;

    let mut summary_map: HashMap<Uuid, AgedReceivableSummary> = HashMap::new();

    for order in orders {
        let summary =
            summary_map
                .entry(order.partner_id)
                .or_insert_with(|| AgedReceivableSummary {
                    partner_id: order.partner_id,
                    partner_name: order.partner_name.clone().unwrap_or("".to_string()),
                    current: dec!(0.00),
                    days_30: dec!(0.00),
                    days_60: dec!(0.00),
                    days_90: dec!(0.00),
                    days_90_plus: dec!(0.00),
                    total: dec!(0.00),
                    orders: Vec::new(),
                });

        let days_overdue = (date - order.due_date).num_days();

        if days_overdue <= 0 {
            summary.current += order.amount_remaining;
        } else if days_overdue <= 30 {
            summary.days_30 += order.amount_remaining;
        } else if days_overdue <= 60 {
            summary.days_60 += order.amount_remaining;
        } else if days_overdue <= 90 {
            summary.days_90 += order.amount_remaining;
        } else {
            summary.days_90_plus += order.amount_remaining;
        }
        summary.total += order.amount_remaining;
        summary.orders.push(order);
    }

    Ok(summary_map.into_values().collect())
}
