/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use rocket::{
    post,
    routes,
    serde::json::Json,
    Route,
};
use rocket_db_pools::Connection;
use shared_core::requests::onboard::OnboardingRequest;

use crate::{
    services::onboarding,
    util::ApiError,
    DbKelpie,
};

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
