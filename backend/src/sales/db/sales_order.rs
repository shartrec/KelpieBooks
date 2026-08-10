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
use shared_core::sales::{
    dtos::sales_order_list_item::SalesOrderListItem,
    models::{
        invoice_address::InvoiceAddress,
        sales_order::SalesOrder,
        sales_order_item::SalesOrderItem,
        sales_order_status::SalesOrderStatus,
    },
    requests::sales_order::CreateSalesOrderRequest,
};
use uuid::Uuid;

fn from_row_to_sales_order(row: &sqlx::postgres::PgRow) -> SalesOrder {
    let bill_to = InvoiceAddress {
        name: row.get("bill_to_name"),
        attention: row.get("bill_to_attention"),
        address_line1: row.get("bill_to_line1"),
        address_line2: row.get("bill_to_line2"),
        city: row.get("bill_to_city"),
        state_province: row.get("bill_to_region"),
        postal_code: row.get("bill_to_postal_code"),
        country: row.get("bill_to_country"),
    };
    let ship_to = InvoiceAddress {
        name: row.get("ship_to_name"),
        attention: row.get("ship_to_attention"),
        address_line1: row.get("ship_to_line1"),
        address_line2: row.get("ship_to_line2"),
        city: row.get("ship_to_city"),
        state_province: row.get("ship_to_region"),
        postal_code: row.get("ship_to_postal_code"),
        country: row.get("ship_to_country"),
    };

    SalesOrder {
        id: row.get("id"),
        org_id: row.get("organization_id"),
        partner_id: row.get("partner_id"),
        warehouse_id: row.get("warehouse_id"),
        warehouse_name: row.try_get("warehouse_name").unwrap_or_default(),
        order_number: row.get("order_number"),
        order_date: row.get("order_date"),
        status: row.get("status"),
        billing_address_id: row.get("billing_address_id"),
        shipping_address_id: row.get("shipping_address_id"),
        bill_to,
        ship_to,
        subtotal: row.get("subtotal"),
        tax_total: row.get("tax_total"),
        total_amount: row.get("total_amount"),
        lines: vec![], // populated separately
    }
}

fn from_row_to_sales_order_item(row: &sqlx::postgres::PgRow) -> SalesOrderItem {
    SalesOrderItem {
        id: row.get("id"),
        order_id: row.get("order_id"),
        item_id: row.get("item_id"),
        code: row.get("code"),
        name: row.get("name"),
        description: row.get("description"),
        quantity: row.get("quantity"),
        unit_price: row.get("unit_price"),
        tax_category_id: row.get("tax_category_id"),
        tax_rate: row.get("tax_rate"),
        tax_amount: row.get("tax_amount"),
        net_amount: row.get("net_amount"),
        sort_order: row.get("sort_order"),
        quantity_available: None, // populated by service layer
    }
}

fn from_row_to_sales_order_list_item(row: &sqlx::postgres::PgRow) -> SalesOrderListItem {
    SalesOrderListItem {
        id: row.get("id"),
        order_number: row.get("order_number"),
        partner_name: row.get("partner_name"),
        order_date: row.get("order_date"),
        warehouse_name: row.get("warehouse_name"),
        status: row.get("status"),
        total_amount: row.get("total_amount"),
    }
}

pub(crate) async fn create_draft_order(
    conn: &mut PgConnection,
    request: &CreateSalesOrderRequest,
    org_id: Uuid,
    order_number: &str,
) -> Result<SalesOrder, sqlx::Error> {
    let row = sqlx::query(
        r#"
        INSERT INTO sales_orders (
            organization_id, partner_id, warehouse_id, order_number, order_date, status,
            billing_address_id, shipping_address_id,
            bill_to_name, bill_to_attention, bill_to_line1, bill_to_line2,
            bill_to_city, bill_to_region, bill_to_postal_code, bill_to_country,
            ship_to_name, ship_to_attention, ship_to_line1, ship_to_line2,
            ship_to_city, ship_to_region, ship_to_postal_code, ship_to_country,
            subtotal, tax_total, total_amount
        )
        VALUES (
            $1, $2, $3, $4, $5, $6,
            $7, $8,
            $9, $10, $11, $12, $13, $14, $15, $16,
            $17, $18, $19, $20, $21, $22, $23, $24,
            $25, $26, $27
        )
        RETURNING
            id, organization_id, partner_id, warehouse_id, order_number, order_date, status,
            billing_address_id, shipping_address_id,
            bill_to_name, bill_to_attention, bill_to_line1, bill_to_line2,
            bill_to_city, bill_to_region, bill_to_postal_code, bill_to_country,
            ship_to_name, ship_to_attention, ship_to_line1, ship_to_line2,
            ship_to_city, ship_to_region, ship_to_postal_code, ship_to_country,
            subtotal, tax_total, total_amount
        "#,
    )
    .bind(org_id)
    .bind(request.partner_id)
    .bind(request.warehouse_id)
    .bind(order_number)
    .bind(request.order_date)
    .bind(SalesOrderStatus::Open)
    .bind(request.billing_address_id)
    .bind(request.shipping_address_id)
    .bind(&request.bill_to.name)
    .bind(&request.bill_to.attention)
    .bind(&request.bill_to.address_line1)
    .bind(&request.bill_to.address_line2)
    .bind(&request.bill_to.city)
    .bind(&request.bill_to.state_province)
    .bind(&request.bill_to.postal_code)
    .bind(&request.bill_to.country)
    .bind(&request.ship_to.name)
    .bind(&request.ship_to.attention)
    .bind(&request.ship_to.address_line1)
    .bind(&request.ship_to.address_line2)
    .bind(&request.ship_to.city)
    .bind(&request.ship_to.state_province)
    .bind(&request.ship_to.postal_code)
    .bind(&request.ship_to.country)
    .bind(Decimal::ZERO)
    .bind(Decimal::ZERO)
    .bind(Decimal::ZERO)
    .fetch_one(conn)
    .await?;

    Ok(from_row_to_sales_order(&row))
}

pub(crate) async fn insert_sales_order_line(
    conn: &mut PgConnection,
    line: &SalesOrderItem,
    order_id: Uuid,
) -> Result<SalesOrderItem, sqlx::Error> {
    let row = sqlx::query(
        r#"
        INSERT INTO sales_order_items (
            order_id, item_id, code, name, description,
            quantity, unit_price, tax_category_id, tax_rate, tax_amount, net_amount, sort_order
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        RETURNING id, order_id, item_id, code, name, description,
            quantity, unit_price, tax_category_id, tax_rate, tax_amount, net_amount, sort_order
        "#,
    )
    .bind(order_id)
    .bind(line.item_id)
    .bind(&line.code)
    .bind(&line.name)
    .bind(&line.description)
    .bind(line.quantity)
    .bind(line.unit_price)
    .bind(line.tax_category_id)
    .bind(line.tax_rate)
    .bind(line.tax_amount)
    .bind(line.net_amount)
    .bind(line.sort_order)
    .fetch_one(conn)
    .await?;

    Ok(from_row_to_sales_order_item(&row))
}

pub(crate) async fn get_sales_order_items(
    conn: &mut PgConnection,
    order_id: Uuid,
) -> Result<Vec<SalesOrderItem>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT soi.id, soi.order_id, soi.item_id, soi.code, soi.name, soi.description,
               soi.quantity, soi.unit_price, soi.tax_category_id, soi.tax_rate,
               soi.tax_amount, soi.net_amount, soi.sort_order
        FROM sales_order_items soi
        WHERE soi.order_id = $1
        ORDER BY soi.sort_order
        "#,
    )
    .bind(order_id)
    .fetch_all(&mut *conn)
    .await?;

    Ok(rows.iter().map(from_row_to_sales_order_item).collect())
}

pub(crate) async fn get_sales_order(
    conn: &mut PgConnection,
    id: Uuid,
    org_id: Uuid,
) -> Result<Option<SalesOrder>, sqlx::Error> {
    let order_row = sqlx::query(
        r#"
        SELECT so.id, so.organization_id, so.partner_id, so.warehouse_id, so.order_number, so.order_date, so.status,
               so.billing_address_id, so.shipping_address_id,
               so.bill_to_name, so.bill_to_attention, so.bill_to_line1, so.bill_to_line2,
               so.bill_to_city, so.bill_to_region, so.bill_to_postal_code, so.bill_to_country,
               so.ship_to_name, so.ship_to_attention, so.ship_to_line1, so.ship_to_line2,
               so.ship_to_city, so.ship_to_region, so.ship_to_postal_code, so.ship_to_country,
               so.subtotal, so.tax_total, so.total_amount,
               w.name AS warehouse_name
        FROM sales_orders so
        JOIN warehouses w ON w.id = so.warehouse_id
        WHERE so.id = $1 AND so.organization_id = $2
        "#,
    )
    .bind(id)
    .bind(org_id)
    .fetch_optional(&mut *conn)
    .await?;

    if let Some(row) = order_row {
        let mut order = from_row_to_sales_order(&row);
        order.lines = get_sales_order_items(conn, order.id).await?;
        Ok(Some(order))
    } else {
        Ok(None)
    }
}

pub(crate) async fn list_sales_orders(
    conn: &mut PgConnection,
    org_id: Uuid,
    status_filter: Option<SalesOrderStatus>,
) -> Result<Vec<SalesOrderListItem>, sqlx::Error> {
    // When no filter is provided, default to open orders only.
    let status = status_filter.unwrap_or(SalesOrderStatus::Open);

    let rows = sqlx::query(
        r#"
        SELECT
            so.id,
            so.order_number,
            p.legal_name AS partner_name,
            so.order_date,
            w.name AS warehouse_name,
            so.status,
            so.total_amount
        FROM sales_orders so
        JOIN partners p  ON p.id = so.partner_id
        JOIN warehouses w ON w.id = so.warehouse_id
        WHERE so.organization_id = $1
          AND so.status = $2
        ORDER BY so.order_date DESC, so.order_number DESC
        "#,
    )
    .bind(org_id)
    .bind(status)
    .fetch_all(&mut *conn)
    .await?;

    Ok(rows
        .iter()
        .map(from_row_to_sales_order_list_item)
        .collect())
}

pub(crate) async fn update_sales_order_totals(
    conn: &mut PgConnection,
    id: Uuid,
    org_id: Uuid,
    subtotal: Decimal,
    tax_total: Decimal,
    total_amount: Decimal,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE sales_orders
        SET subtotal = $1, tax_total = $2, total_amount = $3
        WHERE id = $4 AND organization_id = $5
        "#,
    )
    .bind(subtotal)
    .bind(tax_total)
    .bind(total_amount)
    .bind(id)
    .bind(org_id)
    .execute(conn)
    .await?;
    Ok(())
}

pub(crate) async fn update_sales_order_status(
    conn: &mut PgConnection,
    id: Uuid,
    org_id: Uuid,
    new_status: SalesOrderStatus,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE sales_orders
        SET status = $1
        WHERE id = $2 AND organization_id = $3
        "#,
    )
    .bind(new_status)
    .bind(id)
    .bind(org_id)
    .execute(conn)
    .await?;
    Ok(())
}
