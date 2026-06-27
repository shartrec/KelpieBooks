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
use shared_core::sales::{
    models::{
        sales_invoice::SalesInvoice,
        sales_invoice_item::SalesInvoiceItem,
    },
    requests::{
        sales_invoice,
        sales_invoice::{
            CreateSalesInvoiceRequest,
            UpdateSalesInvoiceRequest,
        },
    },
};
use shared_core::sales::models::customer_payment::CustomerPayment;
use crate::{
    sales::services::sales_invoice_service,
    security::{
        ManageSales,
        RequirePrivilege,
        UseSales,
    },
    util::{
        types::{
            FormSalesInvoiceStatus,
            PathDate,
            PathUuid,
        },
        ApiError,
    },
    DbKelpie,
};
use crate::sales::services::customer_payment_service;

pub(crate) fn routes() -> Vec<Route> {
    routes![
        get_sales_invoices,
        get_sales_invoice,
        get_sales_invoice_payments,
        create_sales_invoice,
        update_sales_invoice,
        update_sales_invoice_items,
    ]
}

#[get("/api/sales-invoices/<id>")]
async fn get_sales_invoice(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UseSales>,
    id: PathUuid,
) -> Result<Json<SalesInvoice>, ApiError> {
    let user = guard.0;
    let invoice =
        sales_invoice_service::get_sales_invoice(&mut pool, user.organization_id, *id).await?;
    Ok(Json(invoice))
}

#[post("/api/sales-invoices", data = "<req>")]
async fn create_sales_invoice(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<ManageSales>,
    req: Json<CreateSalesInvoiceRequest>,
) -> Result<Json<SalesInvoice>, ApiError> {
    let user = guard.0;
    let new_invoice =
        sales_invoice_service::create_invoice(&mut pool, user.organization_id, &req).await?;
    Ok(Json(new_invoice))
}

#[put("/api/sales-invoices/<_id>", data = "<req>")]
async fn update_sales_invoice(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<ManageSales>,
    _id: PathUuid,
    req: Json<UpdateSalesInvoiceRequest>,
) -> Result<Json<SalesInvoice>, ApiError> {
    let user = guard.0;
    let new_invoice =
        sales_invoice_service::update_sales_invoice(&mut pool, user.organization_id, &req).await?;
    Ok(Json(new_invoice))
}

#[put("/api/sales-invoices/<id>/items", data = "<req>")]
async fn update_sales_invoice_items(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<ManageSales>,
    id: PathUuid,
    req: Json<Vec<SalesInvoiceItem>>,
) -> Result<Json<SalesInvoice>, ApiError> {
    let user = guard.0;
    let updated_invoice =
        sales_invoice_service::update_invoice_items(&mut pool, user.organization_id, *id, &req)
            .await?;
    Ok(Json(updated_invoice))
}

#[get("/api/sales-invoices?<start_date>&<end_date>&<partner_id>&<min_amount>&<status>")]
async fn get_sales_invoices(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UseSales>,
    start_date: Option<PathDate>,
    end_date: Option<PathDate>,
    partner_id: Option<PathUuid>,
    min_amount: Option<Decimal>,
    status: Option<FormSalesInvoiceStatus>,
) -> Result<
    Json<Vec<shared_core::sales::dtos::sales_invoice_list_item::SalesInvoiceListItem>>,
    ApiError,
> {
    let user = guard.0;
    let invoices = sales_invoice_service::get_sales_invoices(
        &mut pool,
        user.organization_id,
        start_date.map(|d| *d),
        end_date.map(|d| *d),
        partner_id.map(|p| *p),
        min_amount,
        status.map(|s| vec![*s]),
    )
    .await?;
    Ok(Json(invoices))
}

#[get("/api/sales-invoices/<invoice_id>/payments")]
async fn get_sales_invoice_payments(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UseSales>,
    invoice_id: PathUuid,
) -> Result<Json<Vec<CustomerPayment>>, ApiError> {
    let user = guard.0;
    let payments = customer_payment_service::get_customer_invoice_payments(
        &mut pool,
        user.organization_id,
        *invoice_id,
    )
        .await?;
    Ok(Json(payments))
}
