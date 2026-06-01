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
use crate::services::onboarding;
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

    onboarding::bootstrap_tenant_organization(&mut pool, &request.into_inner()).await?;

    Ok("Organization, user, and chart of accounts created successfully.")
}
