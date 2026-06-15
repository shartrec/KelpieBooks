/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use rocket::{get, routes, Route};
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use shared_core::sales::models::tax::TaxCategory;
use crate::DbKelpie;
use crate::sales::db::tax as tax_db;
use crate::security::{RequirePrivilege, UseSales};
use crate::util::ApiError;

pub(crate) fn routes() -> Vec<Route> {
    routes![get_tax_categories]
}

#[get("/api/sales/tax-categories")]
async fn get_tax_categories(
    mut pool: Connection<DbKelpie>,
    _guard: RequirePrivilege<UseSales>,
) -> Result<Json<Vec<TaxCategory>>, ApiError> {
    let tax_categories = tax_db::get_active_tax_categories(&mut pool).await?;
    Ok(Json(tax_categories))
}
