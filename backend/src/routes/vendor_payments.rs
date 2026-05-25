/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::routes::security::AuthenticatedUser;
use crate::util::ApiError;
use crate::DbKelpie;
use rocket::serde::json::Json;
use rocket::{post, routes, Route};
use rocket_db_pools::Connection;
use shared_core::models::vendor_payment::VendorPayment;
use shared_core::requests::vendor_payment::CreateVendorPaymentRequest;
use crate::services::vendor_payment_service;

pub(crate) fn routes() -> Vec<Route> {
    routes![
        create_vendor_payment,
    ]
}

#[post("/api/vendor-payments", data = "<req>")]
async fn create_vendor_payment(
    mut pool: Connection<DbKelpie>,
    user: AuthenticatedUser,
    req: Json<CreateVendorPaymentRequest>,
) -> Result<Json<VendorPayment>, ApiError> {
    let new_payment = vendor_payment_service::create_vendor_payment(&mut pool, user.organization_id, &req).await?;
    Ok(Json(new_payment))
}
