/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::NaiveDate;
use rocket::{
    delete,
    get,
    post,
    put,
    routes,
    serde::json::Json,
    Route,
};
use rocket_db_pools::Connection;
use shared_core::sales::models::tax::{
    TaxCategory,
    TaxRate,
};

use crate::{
    core::routes::security::AuthenticatedUser,
    sales::services::{
        tax_category_service,
        tax_rate_service,
    },
    security::{
        ManageSales,
        RequirePrivilege,
        UseSales,
    },
    util::{
        types::{
            PathDate,
            PathUuid,
        },
        ApiError,
    },
    DbKelpie,
};

pub(crate) fn routes() -> Vec<Route> {
    routes![
        get_tax_categories,
        get_tax_category,
        create_tax_category,
        update_tax_category,
        delete_tax_category,
        get_tax_rates_for_category,
        get_current_tax_rate_for_category_route,
        update_tax_rates_for_category,
    ]
}

#[get("/api/sales/tax-categories")]
async fn get_tax_categories(
    mut pool: Connection<DbKelpie>,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<UseSales>,
) -> Result<Json<Vec<TaxCategory>>, ApiError> {
    let tax_categories =
        tax_category_service::get_tax_categories(&mut pool, user.organization_id).await?;
    Ok(Json(tax_categories))
}

#[get("/api/sales/tax-categories/<id>")]
async fn get_tax_category(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<UseSales>,
) -> Result<Json<TaxCategory>, ApiError> {
    let tax_category = tax_category_service::get_tax_category(&mut pool, *id, user.organization_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Tax Category not found".to_string()))?;
    Ok(Json(tax_category))
}

#[post("/api/sales/tax-categories", data = "<tax_category>")]
async fn create_tax_category(
    mut pool: Connection<DbKelpie>,
    tax_category: Json<TaxCategory>,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<ManageSales>,
) -> Result<Json<TaxCategory>, ApiError> {
    let new_tax_category =
        tax_category_service::create_tax_category(&mut pool, user.organization_id, &tax_category)
            .await?;
    Ok(Json(new_tax_category))
}

#[put("/api/sales/tax-categories/<id>", data = "<tax_category>")]
async fn update_tax_category(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
    tax_category: Json<TaxCategory>,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<ManageSales>,
) -> Result<Json<TaxCategory>, ApiError> {
    let updated_tax_category = tax_category_service::update_tax_category(
        &mut pool,
        *id,
        user.organization_id,
        &tax_category,
    )
    .await?;
    Ok(Json(updated_tax_category))
}

#[delete("/api/sales/tax-categories/<id>")]
async fn delete_tax_category(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<ManageSales>,
) -> Result<&'static str, ApiError> {
    let rows_affected =
        tax_category_service::delete_tax_category(&mut pool, *id, user.organization_id).await?;
    if rows_affected == 0 {
        return Err(ApiError::NotFound("Tax Category not found.".to_string()));
    }
    Ok("Tax Category deleted successfully.")
}

#[get("/api/sales/tax-categories/<id>/rates")]
async fn get_tax_rates_for_category(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<UseSales>,
) -> Result<Json<Vec<TaxRate>>, ApiError> {
    let rates =
        tax_rate_service::get_tax_rates_for_category(&mut pool, *id, user.organization_id).await?;
    Ok(Json(rates))
}

#[get("/api/sales/tax-categories/<id>/current-rate?<effective_date>")]
async fn get_current_tax_rate_for_category_route(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<UseSales>,
    effective_date: Option<PathDate>,
) -> Result<Json<Option<TaxRate>>, ApiError> {
    let date = effective_date
        .map(|d| *d)
        .unwrap_or_else(|| chrono::Local::now().naive_local().date());
    let rate = tax_rate_service::get_current_tax_rate_for_category(
        &mut pool,
        *id,
        user.organization_id,
        date,
    )
    .await?;
    Ok(Json(rate))
}

#[put("/api/sales/tax-categories/<id>/rates", data = "<rates>")]
async fn update_tax_rates_for_category(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
    rates: Json<Vec<TaxRate>>,
    user: AuthenticatedUser,
    _guard: RequirePrivilege<ManageSales>,
) -> Result<&'static str, ApiError> {
    tax_rate_service::update_tax_rates_for_category(&mut pool, *id, user.organization_id, &rates)
        .await?;
    Ok("Tax rates updated successfully.")
}
