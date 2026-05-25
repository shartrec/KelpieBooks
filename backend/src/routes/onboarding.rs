/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::db;
use crate::db::chart_of_accounts::ChartOfAccountsTemplate;
use crate::routes::security::hash_pwd;
use crate::util::ApiError;
use crate::DbKelpie;
use rocket::serde::json::Json;
use rocket::{post, routes, Route};
use rocket_db_pools::Connection;
use shared_core::requests::onboard::OnboardingRequest;
use sqlx::Acquire;
use std::fs;

pub(crate) fn routes() -> Vec<Route> {
    routes![register]
}

#[post("/api/register", data = "<request>")]
async fn register(
    mut pool: Connection<DbKelpie>,
    request: Json<OnboardingRequest>,
) -> Result<&'static str, ApiError> {
    // 1. Read and parse the chart of accounts template.
    let toml_str = fs::read_to_string(format!("templates/{}.toml", request.coa_template_id))
        .map_err(|e| {
            log::error!("Failed to read chart of accounts template: {}", e);
            ApiError::Error(
                "Server configuration error: Could not load chart of accounts.".to_string(),
            )
        })?;
    let template: ChartOfAccountsTemplate = toml::from_str(&toml_str).map_err(|e| {
        log::error!("Failed to parse chart of accounts template: {}", e);
        ApiError::Error("Server configuration error: Invalid chart of accounts format.".to_string())
    })?;

    // 2. Start the database transaction.
    let mut tx = pool.begin().await?;

    // 3. Create the organization.
    let org = db::organization::create(&mut tx, &request.organization_name).await?;

    // 4. Create the user.
    let pwd_hash = hash_pwd(&request.user_password)?;
    let _user = db::user::insert(
        &mut tx,
        org.id,
        &request.user_email,
        &pwd_hash,
        &request.user_full_name,
        request.user_display_name.as_deref(),
    )
    .await?;

    // 5. Import the chart of accounts for the new organization.
    db::chart_of_accounts::import_default_accounts(&mut tx, org.id, template.accounts).await?;

    // 6. Commit the transaction.
    tx.commit().await?;

    Ok("Organization, user, and chart of accounts created successfully.")
}
