/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::payables::services::vendor_payment_service;
use crate::security::{ManageVendorInvoices, RequirePrivilege};
use crate::util::ApiError;
use crate::DbKelpie;
use rocket::serde::json::Json;
use rocket::{post, routes, Route};
use rocket_db_pools::Connection;
use shared_core::payables::models::vendor_payment::VendorPayment;
use shared_core::payables::requests::vendor_payment::CreateVendorPaymentRequest;

pub(crate) fn routes() -> Vec<Route> {
    routes![create_vendor_payment]
}

#[post("/api/vendor-payments", data = "<req>")]
async fn create_vendor_payment(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<ManageVendorInvoices>,
    req: Json<CreateVendorPaymentRequest>,
) -> Result<Json<VendorPayment>, ApiError> {
    let user = guard.0;
    let new_payment =
        vendor_payment_service::create_vendor_payment(&mut pool, user.organization_id, &req)
            .await?;
    Ok(Json(new_payment))
}
