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
use shared_core::sales::{
    models::{
        sales_invoice::SalesInvoice,
        sales_invoice_item::SalesInvoiceLine,
    },
    requests::create_sales_invoice_request,
};
use shared_core::sales::requests::create_sales_invoice_request::CreateSalesInvoiceRequest;
use crate::{
    sales::services::sales_invoice_service,
    security::{
        ManageSales,
        RequirePrivilege,
        UseSales,
    },
    util::{
        types::PathUuid,
        ApiError,
    },
    DbKelpie,
};

pub(crate) fn routes() -> Vec<Route> {
    routes![
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
