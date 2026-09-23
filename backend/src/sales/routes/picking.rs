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
    routes,
    serde::json::Json,
    Route,
};
use rocket_db_pools::Connection;
use shared_core::{
    sales::{
        dtos::picklist::PickLineItem,
        models::{
            sales_order::SalesOrder,
            shipment::Shipment,
        },
    },
    OrderId,
};
use sqlx::Acquire;

use crate::{
    core::routes::security::AuthenticatedUser,
    sales::services::sales_order_service,
    util::ApiError,
    DbKelpie,
};

pub(crate) fn routes() -> Vec<Route> {
    routes![get_order_header, get_pick_details, submit_pick_details]
}

/// GET /api/orders/<order_number>/pick-details
#[get("/api/orders/<order_number>/header")]
pub async fn get_order_header(
    mut pool: Connection<DbKelpie>,
    user: AuthenticatedUser,
    order_number: &str,
) -> Result<Json<SalesOrder>, ApiError> {
    let items =
        sales_order_service::get_sales_order_header(&mut pool, user.organization_id, order_number)
            .await?;
    Ok(Json(items))
}

#[get("/api/orders/<order_id>/pick-details")]
pub async fn get_pick_details(
    mut pool: Connection<DbKelpie>,
    user: AuthenticatedUser,
    order_id: OrderId,
) -> Result<Json<Vec<PickLineItem>>, ApiError> {
    let items =
        sales_order_service::get_pick_details(&mut pool, user.organization_id, order_id).await?;
    Ok(Json(items))
}

/// POST /api/orders/pick-submit
#[post("/api/orders/pick-submit", format = "json", data = "<payload>")]
pub async fn submit_pick_details(
    mut pool: Connection<DbKelpie>,
    user: AuthenticatedUser,
    payload: Json<Shipment>,
) -> Result<(), ApiError> {
    let mut tx = pool.begin().await.map_err(|e| ApiError::Db(e))?;

    sales_order_service::update_pick_details(&mut tx, user.organization_id, user.user_id, &payload)
        .await?;

    tx.commit().await.map_err(|e| ApiError::Db(e))?;

    Ok(())
}
