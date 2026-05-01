/*
 * Copyright (c) 2026. Trevor Campbell and others.
 *
 * This file is part of KelpieBooks.
 *
 * KelpieBooks is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License,or
 * (at your option) any later version.
 *
 * KelpieBooks is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with KelpieBooks; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 *
 * Contributors:
 *      Trevor Campbell
 *
 */

use crate::routes::security::AuthenticatedUser;
use crate::services::report_service;
use crate::util::ApiError;
use crate::DbKelpie;
use chrono::NaiveDate;
use rocket::serde::json::Json;
use rocket::{get, routes, Route};
use rocket_db_pools::Connection;
use shared_core::dtos::account_with_balance::AccountWithBalance;
use shared_core::reports::balance_sheet::BalanceSheet;

pub(crate) fn routes() -> Vec<Route> {
    routes![get_profit_loss, get_balance_sheet, get_trial_balance]
}

#[get("/api/reports/profit-loss?<start>&<end>")]
async fn get_profit_loss(
    mut pool: Connection<DbKelpie>,
    user: AuthenticatedUser,
    start: String,
    end: String,
) -> Result<Json<Vec<AccountWithBalance>>, ApiError> {
    let start_date = NaiveDate::parse_from_str(&start, "%Y-%m-%d")
        .map_err(|_| ApiError::Invalid("Invalid start date".to_string()))?;
    let end_date = NaiveDate::parse_from_str(&end, "%Y-%m-%d")
        .map_err(|_| ApiError::Invalid("Invalid end date".to_string()))?;

    let report = report_service::get_profit_loss(&mut pool, user.organization_id, start_date, end_date).await?;
    Ok(Json(report))
}

#[get("/api/reports/balance-sheet?<date>")]
async fn get_balance_sheet(
    mut pool: Connection<DbKelpie>,
    user: AuthenticatedUser,
    date: String,
) -> Result<Json<BalanceSheet>, ApiError> {
    let report_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|_| ApiError::Invalid("Invalid date".to_string()))?;

    let report = report_service::get_balance_sheet(&mut pool, user.organization_id, report_date).await?;
    Ok(Json(report))
}

#[get("/api/reports/trial-balance?<date>")]
async fn get_trial_balance(
    mut pool: Connection<DbKelpie>,
    user: AuthenticatedUser,
    date: String,
) -> Result<Json<Vec<AccountWithBalance>>, ApiError> {
    let report_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|_| ApiError::Invalid("Invalid date".to_string()))?;

    let report = report_service::get_trial_balance(&mut pool, user.organization_id, report_date).await?;
    Ok(Json(report))
}
