/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use chrono::NaiveDate;
use rocket_db_pools::sqlx::{
    self,
    PgConnection,
    Row,
};
use rust_decimal::Decimal;
use shared_core::{
    sales::{
        dtos::sales_order_dto::SalesOrderDto,
        models::{
            fulfillment_status::FulfillmentStatus,
            order_address::{
                AddressType,
                OrderAddress,
            },
            payment_status::PaymentStatus,
            sales_document_status::SalesDocumentStatus,
            sales_order::SalesOrder,
            sales_order_item::SalesOrderItem,
        },
        requests::sales_order::CreateSalesOrderRequest,
    },
    AddressId,
    OrderId,
    OrgId,
    PartnerId,
    TaxCategoryId,
};
use sqlx::Acquire;

fn from_row_to_sales_order_list_item(row: &sqlx::postgres::PgRow) -> SalesOrder {
    SalesOrder {
        id: row.get("id"),
        org_id: row.get("org_id"),
        order_number: row.get("order_number"),
        partner_id: row.get("partner_id"),
        partner_name: row.get("partner_name"),
        order_date: row.get("order_date"),
        due_date: row.get("due_date"),
        warehouse_id: row.get("warehouse_id"),
        warehouse_name: row.get("warehouse_name"),
        fulfillment_status: row.get("fulfillment_status"),
        payment_status: row.get("payment_status"),
        document_status: row.get("document_status"),
        subtotal: row.get("subtotal"),
        tax_total: row.get("tax_total"),
        total_amount: row.get("total_amount"),
        amount_remaining: row.get("amount_remaining"),
        billing_address_id: row.get("billing_address_id"),
        shipping_address_id: row.get("shipping_address_id"),
    }
}

pub(crate) async fn create_draft_order(
    conn: &mut PgConnection,
    org_id: OrgId,
    request: &CreateSalesOrderRequest,
    order_number: &str,
) -> Result<SalesOrder, sqlx::Error> {
    let mut tx = conn.begin().await?;

    let row = sqlx::query_as!(
        SalesOrder,
        r#"
        INSERT INTO sales_orders (
            organization_id, partner_id, warehouse_id, order_number, order_date, due_date,
            fulfillment_status, payment_status, document_status,
            billing_address_id, shipping_address_id,
            subtotal, tax_total, total_amount
        )
        VALUES (
            $1, $2, $3, $4, $5, $6,
            $7::fulfillment_status, $8::payment_status, $9::sales_document_status,
            $10, $11,
            $12, $13, $14
        )
        RETURNING
            id, organization_id as org_id, partner_id, null as partner_name, warehouse_id, null as warehouse_name, order_number, order_date, due_date,
            fulfillment_status as "fulfillment_status: FulfillmentStatus",
            payment_status as "payment_status: PaymentStatus",
            document_status as "document_status: SalesDocumentStatus",
            billing_address_id as "billing_address_id: AddressId", shipping_address_id as "shipping_address_id: AddressId",
            subtotal, tax_total, total_amount, amount_remaining
        "#,
        *org_id,
        *request.partner_id,
        *request.warehouse_id,
        order_number,
        request.order_date,
        request.due_date,
        FulfillmentStatus::Unfulfilled as FulfillmentStatus,
        PaymentStatus::Unpaid as PaymentStatus,
        SalesDocumentStatus::Draft as SalesDocumentStatus,
        request.billing_address_id.map(|id| *id),
        request.shipping_address_id.map(|id| *id),
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
    )
        .fetch_one(&mut *tx)

    .await?;

    let bill_to = OrderAddress {
        id: AddressId::default(),
        order_id: row.id,
        name: request.bill_to.name.clone(),
        attention: request.bill_to.attention.clone(),
        line1: request.bill_to.line1.clone(),
        line2: request.bill_to.line2.clone(),
        city: request.bill_to.city.clone(),
        region: request.bill_to.region.clone(),
        postal_code: request.bill_to.postal_code.clone(),
        country: request.bill_to.country.clone(),
    };
    insert_sales_order_address(&mut tx, row.id, &bill_to, AddressType::Billing).await?;

    let ship_to = OrderAddress {
        id: AddressId::default(),
        order_id: row.id,
        name: request.ship_to.name.clone(),
        attention: request.ship_to.attention.clone(),
        line1: request.ship_to.line1.clone(),
        line2: request.ship_to.line2.clone(),
        city: request.ship_to.city.clone(),
        region: request.ship_to.region.clone(),
        postal_code: request.ship_to.postal_code.clone(),
        country: request.ship_to.country.clone(),
    };
    insert_sales_order_address(&mut tx, row.id, &ship_to, AddressType::Shipping).await?;

    tx.commit().await?;

    Ok(row)
}

pub(crate) async fn insert_sales_order_line(
    conn: &mut PgConnection,
    line: &SalesOrderItem,
    order_id: OrderId,
) -> Result<SalesOrderItem, sqlx::Error> {
    let row = sqlx::query_as!(
        SalesOrderItem,
        r#"
        INSERT INTO sales_order_items (
            order_id, item_id, code, name, description,
            quantity, unit_price, tax_category_id, tax_rate, tax_amount, net_amount, sort_order
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        RETURNING id, order_id, item_id, code, name, description,
            quantity, unit_price, tax_category_id as "tax_category_id: TaxCategoryId",
            tax_rate, tax_amount, net_amount, sort_order, null as "quantity_available: Decimal"
        "#,
        *order_id,
        *line.item_id,
        line.code,
        line.name,
        line.description,
        line.quantity,
        line.unit_price,
        line.tax_category_id.map(|id| *id),
        line.tax_rate,
        line.tax_amount,
        line.net_amount,
        line.sort_order,
    )
    .fetch_one(conn)
    .await?;

    Ok(row)
}
pub(crate) async fn insert_sales_order_address(
    conn: &mut PgConnection,
    order_id: OrderId,
    addr: &OrderAddress,
    address_type: AddressType,
) -> Result<OrderAddress, sqlx::Error> {
    let row = sqlx::query_as!(
        OrderAddress,
        r#"
        INSERT INTO sales_order_addresses (
            order_id, name, attention, line1, line2, city, region, postal_code, country, type
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10::address_type)
        RETURNING id, order_id, name, attention, line1, line2, city, region, postal_code, country
        "#,
        *order_id,
        addr.name,
        addr.attention,
        addr.line1,
        addr.line2,
        addr.city,
        addr.region,
        addr.postal_code,
        addr.country,
        address_type as AddressType,
    )
    .fetch_one(conn)
    .await?;

    Ok(row)
}

pub(crate) async fn get_sales_order_items(
    conn: &mut PgConnection,
    order_id: OrderId,
) -> Result<Vec<SalesOrderItem>, sqlx::Error> {
    let rows = sqlx::query_as!(
        SalesOrderItem,
        r#"
        SELECT id, order_id, item_id, code, name, description,
               quantity, unit_price, tax_category_id as "tax_category_id: TaxCategoryId", tax_rate,
               tax_amount, net_amount, sort_order, null as "quantity_available: Decimal"
        FROM sales_order_items soi
        WHERE order_id = $1
        ORDER BY sort_order
        "#,
        *order_id,
    )
    .fetch_all(&mut *conn)
    .await?;

    Ok(rows)
}

pub(crate) async fn get_sales_order_address(
    conn: &mut PgConnection,
    order_id: OrderId,
    address_type: AddressType,
) -> Result<OrderAddress, sqlx::Error> {
    let address = sqlx::query_as!(
        OrderAddress,
        r#"
        SELECT id, order_id, name, attention, line1, line2, city, region, postal_code, country
        FROM sales_order_addresses
        WHERE order_id = $1 and type = $2::address_type
        "#,
        *order_id,
        address_type as AddressType,
    )
    .fetch_one(&mut *conn)
    .await?;

    Ok(address)
}

pub(crate) async fn get_sales_order(
    conn: &mut PgConnection,
    org_id: OrgId,
    id: OrderId,
) -> Result<Option<SalesOrderDto>, sqlx::Error> {
    let order_row = sqlx::query_as!(
        SalesOrder,
        r#"
        SELECT so.id, so.organization_id as org_id, so.partner_id, so.warehouse_id, so.order_number, so.order_date, so.due_date,
               so.fulfillment_status as "fulfillment_status: FulfillmentStatus",
               so.payment_status as "payment_status: PaymentStatus",
               so.document_status as "document_status: SalesDocumentStatus",
               so.billing_address_id as "billing_address_id: AddressId", so.shipping_address_id as "shipping_address_id: AddressId",
               so.subtotal, so.tax_total, so.total_amount, so.amount_remaining,
               w.name AS warehouse_name,
               p.trade_name as partner_name
        FROM sales_orders so
        JOIN warehouses w ON w.id = so.warehouse_id
        JOIN partners p ON p.id = so.partner_id
        WHERE so.id = $1 AND so.organization_id = $2
        "#,
        *id,
        *org_id,
    )
    .fetch_optional(&mut *conn)
    .await?;

    if let Some(order) = order_row {
        let items = get_sales_order_items(conn, order.id).await?;
        let bill_to = get_sales_order_address(conn, order.id, AddressType::Billing).await?;
        let ship_to = get_sales_order_address(conn, order.id, AddressType::Shipping).await?;
        Ok(Some(SalesOrderDto {
            order,
            bill_to,
            ship_to,
            items,
        }))
    } else {
        Ok(None)
    }
}

pub(crate) async fn list_sales_orders(
    conn: &mut PgConnection,
    org_id: OrgId,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    partner_id: Option<PartnerId>,
    min_amount: Option<Decimal>,
    statuses: Option<Vec<SalesDocumentStatus>>,
) -> Result<Vec<SalesOrder>, sqlx::Error> {
    let mut query = sqlx::QueryBuilder::<sqlx::Postgres>::new(
        r#"
        SELECT
            so.id,
            so.organization_id AS org_id,
            so.partner_id,
            so.warehouse_id,
            so.order_number,
            so.order_date,
            so.due_date,
            so.fulfillment_status,
            so.payment_status,
            so.document_status,
            so.billing_address_id,
            so.shipping_address_id,
            so.subtotal,
            so.tax_total,
            so.total_amount,
            so.amount_remaining,
            w.name AS warehouse_name,
            p.legal_name AS partner_name
        FROM sales_orders so
        JOIN partners p ON p.id = so.partner_id
        JOIN warehouses w ON w.id = so.warehouse_id
        WHERE so.organization_id =
        "#,
    );

    query.push_bind(org_id);

    if let Some(start_date) = start_date {
        query.push(" AND so.issue_date >= ").push_bind(start_date);
    }

    if let Some(end_date) = end_date {
        query.push(" AND so.issue_date <= ").push_bind(end_date);
    }

    if let Some(partner_id) = partner_id {
        query.push(" AND so.partner_id = ").push_bind(*partner_id);
    }

    if let Some(min_amount) = min_amount {
        query.push(" AND so.total_amount >= ").push_bind(min_amount);
    }

    if let Some(statuses) = statuses {
        if !statuses.is_empty() {
            query.push(" AND so.document_status IN (");

            let mut separated = query.separated(", ");

            for status in statuses {
                separated.push_bind(status);
            }

            separated.push_unseparated(")");
        }
    }

    query.push(" ORDER BY so.order_date DESC, so.order_number DESC");

    let rows = query.build().fetch_all(&mut *conn).await?;

    Ok(rows.iter().map(from_row_to_sales_order_list_item).collect())
}

pub(crate) async fn update_sales_order_totals(
    conn: &mut PgConnection,
    org_id: OrgId,
    id: OrderId,
    subtotal: Decimal,
    tax_total: Decimal,
    total_amount: Decimal,
    amount_remaining: Decimal,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE sales_orders
        SET subtotal = $1, tax_total = $2, total_amount = $3, amount_remaining = $4
        WHERE id = $5 AND organization_id = $6
        "#,
        subtotal,
        tax_total,
        total_amount,
        amount_remaining,
        *id,
        *org_id,
    )
    .execute(conn)
    .await?;
    Ok(())
}

pub(crate) async fn update_sales_order_status(
    conn: &mut PgConnection,
    org_id: OrgId,
    id: OrderId,
    new_status: SalesDocumentStatus,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE sales_orders
        SET document_status = $1::sales_document_status
        WHERE id = $2 AND organization_id = $3
        "#,
        new_status as SalesDocumentStatus,
        *id,
        *org_id,
    )
    .execute(conn)
    .await?;
    Ok(())
}

pub(crate) async fn update_amount_remaining(
    pool: &mut PgConnection,
    id: OrderId,
    amount: Decimal,
) -> Result<(), sqlx::Error> {
    let _result = sqlx::query!(
        r#"
        UPDATE sales_orders
            SET amount_remaining = amount_remaining + $1
            WHERE id = $2
        "#,
        amount,
        *id
    )
    .execute(&mut *pool)
    .await?;

    // update the status if amount is zero
    let _result = sqlx::query!(
        r#"
        UPDATE sales_orders
            SET payment_status = $1::payment_status
            WHERE id = $2 and amount_remaining = 0.0
        "#,
        PaymentStatus::Paid as PaymentStatus,
        *id
    )
    .execute(&mut *pool)
    .await?;

    Ok(())
}
