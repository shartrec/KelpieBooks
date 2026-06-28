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
use shared_core::sales::{
    models::customer_payment::CustomerPayment,
    requests::customer_payment::CreateCustomerPaymentRequest,
};

use crate::{
    sales::services::customer_payment_service,
    security::{
        ManageSales,
        RequirePrivilege,
    },
    util::ApiError,
    DbKelpie,
};

pub(crate) fn routes() -> Vec<Route> {
    routes![create_customer_payment]
}

#[post("/api/customer-payments", data = "<req>")]
async fn create_customer_payment(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<ManageSales>,
    req: Json<CreateCustomerPaymentRequest>,
) -> Result<Json<CustomerPayment>, ApiError> {
    let user = guard.0;
    let new_payment =
        customer_payment_service::create_customer_payment(&mut pool, user.organization_id, &req)
            .await?;
    Ok(Json(new_payment))
}
