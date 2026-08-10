/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use rocket::{
    get,
    http::Status,
    post,
    routes,
    serde::json::Json,
    Route,
};
use rocket_db_pools::Connection;
use shared_core::sales::{
    dtos::sales_order_list_item::SalesOrderListItem,
    models::{
        sales_invoice::SalesInvoice,
        sales_order::SalesOrder,
    },
    requests::sales_order::CreateSalesOrderRequest,
};

use crate::{
    sales::services::sales_order_service,
    security::{
        ManageSales,
        RequirePrivilege,
        UseSales,
    },
    util::{
        types::{
            FormSalesOrderStatus,
            PathUuid,
        },
        ApiError,
    },
    DbKelpie,
};

pub(crate) fn routes() -> Vec<Route> {
    routes![
        list_sales_orders,
        get_sales_order,
        create_sales_order,
        confirm_sales_order,
        cancel_sales_order,
    ]
}

#[get("/api/sales-orders?<status>")]
async fn list_sales_orders(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UseSales>,
    status: Option<FormSalesOrderStatus>,
) -> Result<Json<Vec<SalesOrderListItem>>, ApiError> {
    let user = guard.0;
    let orders = sales_order_service::list_sales_orders(
        &mut pool,
        user.organization_id,
        status.map(|s| *s),
    )
    .await?;
    Ok(Json(orders))
}

#[get("/api/sales-orders/<id>")]
async fn get_sales_order(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UseSales>,
    id: PathUuid,
) -> Result<Json<SalesOrder>, ApiError> {
    let user = guard.0;
    let order =
        sales_order_service::get_sales_order(&mut pool, *id, user.organization_id).await?;
    Ok(Json(order))
}

#[post("/api/sales-orders", data = "<req>")]
async fn create_sales_order(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<ManageSales>,
    req: Json<CreateSalesOrderRequest>,
) -> Result<Json<SalesOrder>, ApiError> {
    let user = guard.0;
    let order =
        sales_order_service::create_order(&mut pool, user.organization_id, &req).await?;
    Ok(Json(order))
}

#[post("/api/sales-orders/<id>/confirm")]
async fn confirm_sales_order(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<ManageSales>,
    id: PathUuid,
) -> Result<Json<SalesInvoice>, ApiError> {
    let user = guard.0;
    let invoice =
        sales_order_service::confirm_order(&mut pool, *id, user.organization_id, user.user_id).await?;
    Ok(Json(invoice))
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
