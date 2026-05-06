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
use crate::services::period_end_service;
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
    user: AuthenticatedUser,
    year_end: String,
) -> Result<&'static str, ApiError> {
    let year_end_date = NaiveDate::parse_from_str(&year_end, "%Y-%m-%d")
        .map_err(|_| ApiError::Invalid("Invalid year end date".to_string()))?;

    period_end_service::close_financial_year(&mut pool, user.organization_id, year_end_date).await?;

    Ok("Financial year closed successfully.")
}
