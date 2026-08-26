/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use rocket_db_pools::sqlx::{
    self,
    PgConnection,
    Row,
};
use rust_decimal::Decimal;
use shared_core::{
    inventory::models::stock_balance::{
        ReferenceType,
        TransactionType,
    },
    sales::{
        dtos::sales_order_dto::SalesOrderDto,
        models::{
            sales_document_status::SalesDocumentStatus,
            sales_order::SalesOrder,
        },
        requests::sales_order::CreateSalesOrderRequest,
    },
};
use sqlx::Acquire;
use uuid::Uuid;

use crate::{
    core::db::sequences::{
        get_next_order_number,
        SeqType,
    },
    inventory::db::{
        inventory as inventory_db,
        location as location_db,
        stock_transaction::{
            log_transaction,
            NewStockTransaction,
        },
    },
    sales::db::{
        item as item_db,
        sales_order as sales_order_db,
    },
    util::ApiError,
};

pub(crate) async fn create_order(
    pool: &mut PgConnection,
    org_id: Uuid,
    req: &CreateSalesOrderRequest,
) -> Result<SalesOrder, ApiError> {
    let mut tx = pool.begin().await?;

    // Generate the next order number using the SalesOrder sequence
    let order_number = get_next_order_number(&mut tx, org_id, &SeqType::SalesOrder).await?;

    let mut order = sales_order_db::create_draft_order(&mut tx, req, org_id, &order_number).await?;

    for line in &req.lines {
        if line.item_id == Uuid::nil() {
            continue;
        }
        sales_order_db::insert_sales_order_line(&mut tx, line, order.id).await?;
    }

    let lines = req.lines.clone();
    order.calculate(&lines);

    sales_order_db::update_sales_order_totals(
        &mut tx,
        order.id,
        org_id,
        order.subtotal,
        order.tax_total,
        order.total_amount,
        order.amount_remaining,
    )
    .await?;

    tx.commit().await?;

    Ok(order)
}

pub(crate) async fn get_sales_order(
    pool: &mut PgConnection,
    id: Uuid,
    org_id: Uuid,
) -> Result<SalesOrderDto, ApiError> {
    let mut order = sales_order_db::get_sales_order(pool, id, org_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Sales order not found.".to_string()))?;

    // Inject quantity_available for stocked line items
    for line in &mut order.items {
        let item = item_db::get(pool, org_id, line.item_id).await?;
        if let Some(item) = item {
            if item.is_stocked() {
                let balances =
                    inventory_db::get_item_stock_balances(pool, org_id, line.item_id).await?;
                // Sum available across only the locations in the order's warehouse
                let warehouse_available: Decimal = balances
                    .location_balances
                    .iter()
                    .filter(|b| b.warehouse_id == order.order.warehouse_id)
                    .map(|b| b.quantity_available.unwrap_or(Decimal::ZERO))
                    .sum();
                line.quantity_available = Some(warehouse_available);
            }
        }
    }

    Ok(order)
}

pub(crate) async fn list_sales_orders(
    pool: &mut PgConnection,
    org_id: Uuid,
    start_date: Option<chrono::NaiveDate>,
    end_date: Option<chrono::NaiveDate>,
    partner_id: Option<Uuid>,
    min_amount: Option<rust_decimal::Decimal>,
    statuses: Vec<SalesDocumentStatus>,
) -> Result<Vec<SalesOrder>, ApiError> {
    let items = sales_order_db::list_sales_orders(
        pool,
        org_id,
        start_date,
        end_date,
        partner_id,
        min_amount,
        Some(statuses),
    )
    .await?;
    Ok(items)
}

pub(crate) async fn confirm_order(
    pool: &mut PgConnection,
    id: Uuid,
    org_id: Uuid,
    user_id: Uuid,
) -> Result<SalesOrderDto, ApiError> {
    let mut tx = pool.begin().await?;

    // Load and verify order status
    let order = sales_order_db::get_sales_order(&mut tx, id, org_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Sales order not found.".to_string()))?;

    if order.order.document_status != SalesDocumentStatus::Draft {
        return Err(ApiError::BadRequest(
            "Only Draft orders can be confirmed.".to_string(),
        ));
    }

    let warehouse_id = order.order.warehouse_id.clone();
    // For each stocked line: adjust allocated quantity and log a stock transaction
    for line in &order.items {
        let item = item_db::get(&mut tx, org_id, line.item_id).await?;
        if let Some(item) = item {
            if item.is_stocked() {

                let item_bal = inventory_db::get_first_balance_for_item_warehouse(
                    &mut tx,
                    org_id,
                    line.item_id,
                    warehouse_id
                ).await?;

                if let Some(bal) = item_bal {
                    inventory_db::adjust_allocated(
                        &mut tx,
                        org_id,
                        warehouse_id,
                        bal.location_id,
                        line.item_id,
                        line.quantity,
                    )
                        .await?;

                    log_transaction(
                        &mut tx,
                        NewStockTransaction {
                            organization_id: org_id,
                            warehouse_id: order.order.warehouse_id,
                            location_id: bal.location_id,
                            item_id: line.item_id,
                            transaction_type: TransactionType::Allocation,
                            quantity_change: line.quantity,
                            reference_type: Some(ReferenceType::SalesOrder),
                            reference_id: Some(order.order.id),
                            notes: Some("Allocated on sales order confirmation"),
                            created_by: user_id,
                        },
                    )
                        .await?;
                } else {
                    return Err(ApiError::BadRequest(
                        format!("Item {} has no stocking location configured", item.name),
                    ));
                }
            }
        }
    }

    // Mark order as Confirmed
    sales_order_db::update_sales_order_status(&mut tx, id, org_id, SalesDocumentStatus::Open)
        .await?;

    tx.commit().await?;

    Ok(order)
}

pub(crate) async fn cancel_order(
    pool: &mut PgConnection,
    id: Uuid,
    org_id: Uuid,
) -> Result<(), ApiError> {
    let order = sales_order_db::get_sales_order(pool, id, org_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Sales order not found.".to_string()))?;

    if order.order.document_status != SalesDocumentStatus::Open
        || order.order.document_status != SalesDocumentStatus::Draft
    {
        return Err(ApiError::BadRequest(
            "Only Draft or Open orders can be cancelled.".to_string(),
        ));
    }

    sales_order_db::update_sales_order_status(pool, id, org_id, SalesDocumentStatus::Cancelled)
        .await?;

    Ok(())
}
