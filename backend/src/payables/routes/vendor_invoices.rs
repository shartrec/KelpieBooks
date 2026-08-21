/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use rocket::{
    get,
    post,
    put,
    routes,
    serde::json::Json,
    Route,
};
use rocket_db_pools::Connection;
use rust_decimal::Decimal;
use shared_core::payables::{
    dtos::{
        vendor_invoice_dto::VendorInvoiceDto,
        vendor_invoice_list_item::VendorInvoiceListItem,
    },
    models::{
        vendor_invoice_item::VendorInvoiceItem,
        vendor_payment::VendorPayment,
    },
    requests::vendor_invoice::{
        CreateVendorInvoiceRequest,
        UpdateVendorInvoiceRequest,
    },
};

use crate::{
    payables::services::{
        vendor_invoice_service,
        vendor_payment_service,
    },
    security::{
        ManageVendorInvoices,
        RequirePrivilege,
        UseVendorInvoices,
    },
    util::{
        types::{
            FormInvoiceStatus,
            PathDate,
            PathUuid,
        },
        ApiError,
    },
    DbKelpie,
};

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
    guard: RequirePrivilege<UseVendorInvoices>,
    start_date: Option<PathDate>,
    end_date: Option<PathDate>,
    partner_id: Option<PathUuid>,
    min_amount: Option<Decimal>,
    status: Option<FormInvoiceStatus>,
) -> Result<Json<Vec<VendorInvoiceListItem>>, ApiError> {
    let user = guard.0;
    let invoices = vendor_invoice_service::get_vendor_invoices(
        &mut pool,
        user.organization_id,
        start_date.map(|d| *d),
        end_date.map(|d| *d),
        partner_id.map(|u| *u),
        min_amount,
        status.map(|s| *s),
    )
    .await?;
    Ok(Json(invoices))
}

#[get("/api/vendor-invoices/<id>")]
async fn get_vendor_invoice(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UseVendorInvoices>,
    id: PathUuid,
) -> Result<Json<VendorInvoiceDto>, ApiError> {
    let user = guard.0;
    let invoice =
        vendor_invoice_service::get_vendor_invoice(&mut pool, user.organization_id, *id).await?;
    Ok(Json(invoice))
}

#[get("/api/vendor-invoices/<invoice_id>/payments")]
async fn get_vendor_invoice_payments(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UseVendorInvoices>,
    invoice_id: PathUuid,
) -> Result<Json<Vec<VendorPayment>>, ApiError> {
    let user = guard.0;
    let payments = vendor_payment_service::get_vendor_invoice_payments(
        &mut pool,
        user.organization_id,
        *invoice_id,
    )
    .await?;
    Ok(Json(payments))
}

#[post("/api/vendor-invoices", data = "<req>")]
async fn create_vendor_invoice(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<ManageVendorInvoices>,
    req: Json<CreateVendorInvoiceRequest>,
) -> Result<Json<VendorInvoiceDto>, ApiError> {
    let user = guard.0;
    let new_invoice =
        vendor_invoice_service::create_vendor_invoice(&mut pool, user.organization_id, &req)
            .await?;
    Ok(Json(new_invoice))
}

#[put("/api/vendor-invoices/<id>", data = "<req>")]
async fn update_vendor_invoice(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<ManageVendorInvoices>,
    id: PathUuid,
    req: Json<UpdateVendorInvoiceRequest>,
) -> Result<Json<VendorInvoiceDto>, ApiError> {
    let user = guard.0;
    let updated_invoice =
        vendor_invoice_service::update_vendor_invoice(&mut pool, user.organization_id, *id, &req)
            .await?;
    Ok(Json(updated_invoice))
}

#[put("/api/vendor-invoices/<id>/items", data = "<req>")]
async fn update_vendor_invoice_items(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<ManageVendorInvoices>,
    id: PathUuid,
    req: Json<Vec<VendorInvoiceItem>>,
) -> Result<Json<Vec<VendorInvoiceItem>>, ApiError> {
    let user = guard.0;
    let updated_items = vendor_invoice_service::update_vendor_invoice_items(
        &mut pool,
        user.organization_id,
        *id,
        &req,
    )
    .await?;
    Ok(Json(updated_items))
}
