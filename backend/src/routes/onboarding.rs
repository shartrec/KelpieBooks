/*
 * Copyright (c) 2026-2026. Trevor Campbell and others.
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

use crate::db;
use crate::db::chart_of_accounts::ChartOfAccountsTemplate;
use crate::util::ApiError;
use crate::DbKelpie;
use rocket::serde::json::Json;
use rocket::{post, routes, Route};
use rocket_db_pools::Connection;
use shared_core::onboarding::OnboardingRequest;
use sqlx::Acquire;
use std::fs;

pub(crate) fn routes() -> Vec<Route> {
    routes![onboard]
}

#[post("/api/onboarding", data = "<request>")]
async fn onboard(
    mut pool: Connection<DbKelpie>,
    request: Json<OnboardingRequest>,
) -> Result<&'static str, ApiError> {
    // 1. Read and parse the chart of accounts template.
    // This is done outside the transaction. In the future, the template name
    // could be part of the OnboardingRequest.
    let toml_str = fs::read_to_string("templates/service.toml")
        .map_err(|e| {
            log::error!("Failed to read chart of accounts template: {}", e);
            ApiError::Error("Server configuration error: Could not load chart of accounts.".to_string())
        })?;
    let template: ChartOfAccountsTemplate = toml::from_str(&toml_str)
        .map_err(|e| {
            log::error!("Failed to parse chart of accounts template: {}", e);
            ApiError::Error("Server configuration error: Invalid chart of accounts format.".to_string())
        })?;

    // 2. Start the database transaction.
    let mut tx = pool.begin().await?;

    // 3. Create the organization.
    let org = db::organization::create(&mut tx, &request.organization_name).await?;

    // 4. Create the user.
    // In a real app, you MUST hash the password.
    let _user = db::user::insert(&mut tx, org.id, request.user_email.clone(), request.user_password.clone()).await?;

    // 5. Import the chart of accounts for the new organization.
    db::chart_of_accounts::import_default_accounts(&mut tx, org.id, template.accounts).await?;

    // 6. Commit the transaction.
    tx.commit().await?;

    Ok("Organization, user, and chart of accounts created successfully.")
}
