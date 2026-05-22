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
use crate::services::vendor_invoice_payment_service;
use crate::util::types::PathUuid;
use crate::util::ApiError;
use crate::DbKelpie;
use rocket::serde::json::Json;
use rocket::{get, post, routes, Route};
use rocket_db_pools::Connection;
use shared_core::models::vendor_invoice_payment::VendorInvoicePayment;
use shared_core::requests::vendor_invoice_payment::CreateVendorInvoicePaymentRequest;

pub(crate) fn routes() -> Vec<Route> {
    routes![
        get_vendor_invoice_payments,
        create_vendor_invoice_payment,
    ]
}

#[get("/api/vendor-invoices/<invoice_id>/payments")]
async fn get_vendor_invoice_payments(
    mut pool: Connection<DbKelpie>,
    user: AuthenticatedUser,
    invoice_id: PathUuid,
) -> Result<Json<Vec<VendorInvoicePayment>>, ApiError> {
    let payments =
        vendor_invoice_payment_service::get_vendor_invoice_payments(&mut pool, user.organization_id, *invoice_id).await?;
    Ok(Json(payments))
}

#[post("/api/vendor-invoice-payments", data = "<req>")]
async fn create_vendor_invoice_payment(
    mut pool: Connection<DbKelpie>,
    user: AuthenticatedUser,
    req: Json<CreateVendorInvoicePaymentRequest>,
) -> Result<Json<VendorInvoicePayment>, ApiError> {
    let new_payment = vendor_invoice_payment_service::create_vendor_invoice_payment(&mut pool, user.organization_id, &req).await?;
    Ok(Json(new_payment))
}
