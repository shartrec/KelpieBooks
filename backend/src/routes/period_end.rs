/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::ledger::services::period_end_service;
use crate::security::{ManageAccounts, RequirePrivilege};
use crate::util::ApiError;
use crate::DbKelpie;
use chrono::NaiveDate;
use rocket::{post, routes, Route};
use rocket_db_pools::Connection;

pub(crate) fn routes() -> Vec<Route> {
    routes![close_financial_year]
}

#[post("/api/period-end/close-year?<year_end>")]
async fn close_financial_year(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<ManageAccounts>,
    year_end: String,
) -> Result<&'static str, ApiError> {
    let user = guard.0;
    let year_end_date = NaiveDate::parse_from_str(&year_end, "%Y-%m-%d")
        .map_err(|_| ApiError::Invalid("Invalid year end date".to_string()))?;

    period_end_service::close_financial_year(&mut pool, user.organization_id, year_end_date)
        .await?;

    Ok("Financial year closed successfully.")
}
