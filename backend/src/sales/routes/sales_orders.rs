/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use rocket::{get, http::Status, post, routes, serde::json::Json, Route, State};
use rocket::http::ContentType;
use rocket_db_pools::Connection;
use shared_core::sales::{
    dtos::sales_order_dto::SalesOrderDto,
    models::sales_order::SalesOrder,
    requests::sales_order::CreateSalesOrderRequest,
};

use crate::{sales::services::sales_order_service, security::{
    ManageSales,
    RequirePrivilege,
    UseSales,
}, util::{
    types::{
        FormSalesOrderStatus,
        PathUuid,
    },
    ApiError,
}, DbKelpie, TemplateConfig};
use crate::sales::reports;
use crate::util::reports::DownloadFile;

pub(crate) fn routes() -> Vec<Route> {
    routes![
        list_sales_orders,
        get_sales_order,
        create_sales_order,
        confirm_sales_order,
        cancel_sales_order,
        print_sales_invoice,
        print_picking_list,
    ]
}

#[get("/api/sales-orders?<status>")]
async fn list_sales_orders(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UseSales>,
    status: Option<FormSalesOrderStatus>,
) -> Result<Json<Vec<SalesOrder>>, ApiError> {
    let user = guard.0;
    let status_list = match status {
        Some(s) => vec![*s],
        None => vec![],
    };

    let orders = sales_order_service::list_sales_orders(
        &mut pool,
        user.organization_id,
        None,
        None,
        None,
        None,
        status_list,
    )
    .await?;
    Ok(Json(orders))
}

#[get("/api/sales-orders/<id>")]
async fn get_sales_order(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UseSales>,
    id: PathUuid,
) -> Result<Json<SalesOrderDto>, ApiError> {
    let user = guard.0;
    let order = sales_order_service::get_sales_order(&mut pool, *id, user.organization_id).await?;
    Ok(Json(order))
}

#[post("/api/sales-orders", data = "<req>")]
async fn create_sales_order(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<ManageSales>,
    req: Json<CreateSalesOrderRequest>,
) -> Result<Json<SalesOrder>, ApiError> {
    let user = guard.0;
    let order = sales_order_service::create_order(&mut pool, user.organization_id, &req).await?;
    Ok(Json(order))
}

#[post("/api/sales-orders/<id>/confirm")]
async fn confirm_sales_order(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<ManageSales>,
    id: PathUuid,
) -> Result<Json<SalesOrderDto>, ApiError> {
    let user = guard.0;
    let order =
        sales_order_service::confirm_order(&mut pool, *id, user.organization_id, user.user_id)
            .await?;
    Ok(Json(order))
}

#[post("/api/sales-orders/<id>/cancel")]
async fn cancel_sales_order(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<ManageSales>,
    id: PathUuid,
) -> Result<Status, ApiError> {
    let user = guard.0;
    sales_order_service::cancel_order(&mut pool, *id, user.organization_id).await?;
    Ok(Status::NoContent)
}

#[get("/api/sales-orders/<id>/print-invoice")]
async fn print_sales_invoice(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UseSales>,
    config: &State<TemplateConfig>,
    id: PathUuid,
) -> Result<DownloadFile, ApiError> {
    let user = guard.0;

    if let Some(order) = crate::sales::db::sales_order::get_sales_order(&mut pool, *id, user.organization_id).await? {
        let name = format!("Invoice-{}.pdf", order.order.order_number);

        let invoice_pdf =
            reports::invoice::generate_invoice(&mut pool, user, config, *id).await?;
        Ok(DownloadFile::new(invoice_pdf, name, ContentType::PDF))
    } else {
        Err(ApiError::NotFound(format!("Order {} not found", *id).into()))
    }
}

#[get("/api/sales-orders/<id>/print-picklist")]
async fn print_picking_list(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UseSales>,
    config: &State<TemplateConfig>,
    id: PathUuid,
) -> Result<DownloadFile, ApiError> {
    let user = guard.0;
    if let Some(order) = crate::sales::db::sales_order::get_sales_order(&mut pool, *id, user.organization_id).await? {
        let name = format!("Picklist-{}.pdf", order.order.order_number);
        let picklist_pdf =
            reports::invoice::generate_picklist(&mut pool, user, config, *id).await?;
        Ok(DownloadFile::new(picklist_pdf, name, ContentType::PDF))
    } else {
        Err(ApiError::NotFound(format!("Order {} not found", *id).into()))
    }
}
