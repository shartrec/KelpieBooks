/*
 * Copyright (c) 2026. Trevor Campbell and others.
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
use crate::services::{vendor_invoice_service, vendor_payment_service};
use crate::util::types::{PathDate, PathUuid};
use crate::util::ApiError;
use crate::DbKelpie;
use rocket::serde::json::Json;
use rocket::{get, post, put, routes, Route};
use rocket_db_pools::Connection;
use shared_core::dtos::vendor_invoice_list_item::VendorInvoiceListItem;
use shared_core::models::vendor_invoice::VendorInvoice;
use shared_core::models::vendor_invoice_item::VendorInvoiceItem;
use shared_core::models::vendor_payment::VendorPayment;
use shared_core::requests::vendor_invoice::{CreateVendorInvoiceRequest, UpdateVendorInvoiceRequest};

pub(crate) fn routes() -> Vec<Route> {
    routes![
        get_vendor_invoices,
        get_vendor_invoice,
        get_vendor_invoice_payments,
        create_vendor_invoice,
        update_vendor_invoice,
        update_vendor_invoice_items,
    ]
}

#[get("/api/vendor-invoices?<start_date>&<end_date>&<partner_id>&<min_amount>&<status>")]
async fn get_vendor_invoices(
    mut pool: Connection<DbKelpie>,
    user: AuthenticatedUser,
    start_date: Option<PathDate>,
    end_date: Option<PathDate>,
    partner_id: Option<PathUuid>,
    min_amount: Option<i64>,
    status: Option<String>,
) -> Result<Json<Vec<VendorInvoiceListItem>>, ApiError> {
    let invoices =
        vendor_invoice_service::get_vendor_invoices(&mut pool, user.organization_id, start_date.map(|d| *d), end_date.map(|d| *d), partner_id.map(|u| *u), min_amount, status).await?;
    Ok(Json(invoices))
}

#[get("/api/vendor-invoices/<id>")]
async fn get_vendor_invoice(
    mut pool: Connection<DbKelpie>,
    user: AuthenticatedUser,
    id: PathUuid,
) -> Result<Json<VendorInvoice>, ApiError> {
    let invoice =
        vendor_invoice_service::get_vendor_invoice(&mut pool, user.organization_id, *id).await?;
    Ok(Json(invoice))
}

#[get("/api/vendor-invoices/<invoice_id>/payments")]
async fn get_vendor_invoice_payments(
    mut pool: Connection<DbKelpie>,
    user: AuthenticatedUser,
    invoice_id: PathUuid,
) -> Result<Json<Vec<VendorPayment>>, ApiError> {
    let payments =
        vendor_payment_service::get_vendor_invoice_payments(&mut pool, user.organization_id, *invoice_id).await?;
    Ok(Json(payments))
}

#[post("/api/vendor-invoices", data = "<req>")]
async fn create_vendor_invoice(
    mut pool: Connection<DbKelpie>,
    user: AuthenticatedUser,
    req: Json<CreateVendorInvoiceRequest>,
) -> Result<Json<VendorInvoice>, ApiError> {
    let new_invoice = vendor_invoice_service::create_vendor_invoice(&mut pool, user.organization_id, &req).await?;
    Ok(Json(new_invoice))
}

#[put("/api/vendor-invoices/<id>", data = "<req>")]
async fn update_vendor_invoice(
    mut pool: Connection<DbKelpie>,
    user: AuthenticatedUser,
    id: PathUuid,
    req: Json<UpdateVendorInvoiceRequest>,
) -> Result<Json<VendorInvoice>, ApiError> {
    let updated_invoice = vendor_invoice_service::update_vendor_invoice(&mut pool, user.organization_id, *id, &req).await?;
    Ok(Json(updated_invoice))
}

#[put("/api/vendor-invoices/<id>/items", data = "<req>")]
async fn update_vendor_invoice_items(
    mut pool: Connection<DbKelpie>,
    user: AuthenticatedUser,
    id: PathUuid,
    req: Json<Vec<VendorInvoiceItem>>,
) -> Result<Json<Vec<VendorInvoiceItem>>, ApiError> {
    let updated_items = vendor_invoice_service::update_vendor_invoice_items(&mut pool, user.organization_id, *id, &req).await?;
    Ok(Json(updated_items))
}
