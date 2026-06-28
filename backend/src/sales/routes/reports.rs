/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use chrono::NaiveDate;
use rocket::{
    get,
    routes,
    serde::json::Json,
    Route,
};
use rocket_db_pools::Connection;
use shared_core::sales::dtos::aged_receivable_summary::AgedReceivableSummary;
use crate::{
    security::{
        RequirePrivilege,
        UseSales,
    },
    util::ApiError,
    DbKelpie,
};
use crate::sales::services::report_service;

pub(crate) fn routes() -> Vec<Route> {
    routes![trial_balance,]
}

#[get("/api/reports/aged-receivables?<date>")]
async fn trial_balance(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UseSales>,
    date: String,
) -> Result<Json<Vec<AgedReceivableSummary>>, ApiError> {
    let user = guard.0;
    let report_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|_| ApiError::BadRequest("Invalid date".to_string()))?;

    let report =
        report_service::get_trial_balance(&mut pool, user.organization_id, report_date).await?;
    Ok(Json(report))
}
