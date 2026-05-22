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
