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
use shared_core::sales::models::item::UnitOfMeasure;
use crate::core::routes::security::AuthenticatedUser;
use crate::DbKelpie;
use crate::sales::db::item as item_db;
use crate::security::{RequirePrivilege, UseSales};
use crate::util::ApiError;

pub(crate) fn routes() -> Vec<Route> {
    routes![get_uoms]
}

#[get("/api/sales/uoms")]
async fn get_uoms(
    mut pool: Connection<DbKelpie>,
    _guard: RequirePrivilege<UseSales>,
    user: AuthenticatedUser,
) -> Result<Json<Vec<UnitOfMeasure>>, ApiError> {
    let uoms = item_db::get_active_uoms(&mut pool, user.organization_id).await?;
    Ok(Json(uoms))
}
