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
        sales_invoice_item::SalesInvoiceLine,
    },
    requests::sales_invoice,
};
use shared_core::sales::requests::sales_invoice::CreateSalesInvoiceRequest;
use crate::{
    sales::services::sales_invoice_service,
    security::{
        ManageSales,
        RequirePrivilege,
        UseSales,
    },
    util::{
        types::{PathDate, PathUuid, FormSalesInvoiceStatus},
        ApiError,
    },
    DbKelpie,
};

pub(crate) fn routes() -> Vec<Route> {
    routes![
        get_sales_invoices,
        get_sales_invoice,
        create_sales_invoice,
        update_sales_invoice_lines,
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
        sales_invoice_service::create_draft_invoice(&mut pool, user.organization_id, &req)
            .await?;
    Ok(Json(new_invoice))
}

#[put("/api/sales-invoices/<id>/lines", data = "<req>")]
async fn update_sales_invoice_lines(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<ManageSales>,
    id: PathUuid,
    req: Json<Vec<SalesInvoiceLine>>,
) -> Result<Json<SalesInvoice>, ApiError> {
    let user = guard.0;
    let updated_invoice =
        sales_invoice_service::update_invoice_lines(&mut pool, user.organization_id, *id, &req)
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
) -> Result<Json<Vec<shared_core::sales::dtos::sales_invoice_list_item::SalesInvoiceListItem>>, ApiError> {
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
