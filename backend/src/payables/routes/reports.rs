/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use chrono::NaiveDate;
use rocket::{get, routes, Route};
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use shared_core::dtos::aged_payable_summary::AgedPayableSummary;
use crate::DbKelpie;
use crate::payables::services::report_service;
use crate::security::{RequirePrivilege, UseVendorInvoices};
use crate::util::ApiError;

pub(crate) fn routes() -> Vec<Route> {
    routes![
        get_aged_payables,
    ]
}

#[get("/api/reports/aged-payables?<date>")]
async fn get_aged_payables(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UseVendorInvoices>,
    date: String,
) -> Result<Json<Vec<AgedPayableSummary>>, ApiError> {
    let user = guard.0;
    let report_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|_| ApiError::Invalid("Invalid date".to_string()))?;

    let report =
        report_service::get_aged_payables(&mut pool, user.organization_id, report_date).await?;
    Ok(Json(report))
}

