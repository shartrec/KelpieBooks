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
};
use shared_core::sales::{
    models::item::{
        Item,
        ItemType,
        UnitOfMeasure,
    },
    requests::item::CreateItemRequest,
};
use uuid::Uuid;

pub(crate) async fn get_active_uoms(
    conn: &mut PgConnection,
    org_id: Uuid,
) -> Result<Vec<UnitOfMeasure>, sqlx::Error> {
    sqlx::query_as::<_, UnitOfMeasure>(
        r#"SELECT id, organization_id ,code, name, is_active FROM units_of_measure
                             WHERE organization_id = $1 AND is_active = true ORDER BY name ASC"#,
    )
    .bind(org_id)
    .fetch_all(conn)
    .await
}

pub async fn all(
    conn: &mut PgConnection,
    org_id: Uuid,
    search_term: Option<String>,
    item_type: Option<ItemType>,
    include_inactive: bool,
    limit: u32,
) -> Result<Vec<Item>, sqlx::Error> {
    let mut query = "SELECT * FROM items WHERE organization_id = $1".to_string();
    let mut i = 2;

    if search_term.is_some() {
        query.push_str(&format!(" AND (code ILIKE ${} OR name ILIKE ${})", i, i));
        i += 1;
    }

    if item_type.is_some() {
        query.push_str(&format!(" AND item_type = ${}", i));
        i += 1;
    }

    if !include_inactive {
        query.push_str(" AND is_active = true");
    }

    query.push_str(&format!(" LIMIT ${}", i));

    let mut query_builder = sqlx::query_as::<_, Item>(&query).bind(org_id);

    if let Some(term) = search_term {
        query_builder = query_builder.bind(format!("%{}%", term));
    }

    if let Some(item_type) = item_type {
        query_builder = query_builder.bind(item_type);
    }

    query_builder = query_builder.bind(limit as i32);

    query_builder.fetch_all(conn).await
}

pub async fn get(
    conn: &mut PgConnection,
    org_id: Uuid,
    id: Uuid,
) -> Result<Option<Item>, sqlx::Error> {
    sqlx::query_as::<_, Item>("SELECT * FROM items WHERE id = $1 AND organization_id = $2")
        .bind(id)
        .bind(org_id)
        .fetch_optional(conn)
        .await
}

pub async fn create(
    conn: &mut PgConnection,
    org_id: Uuid,
    item: &CreateItemRequest,
) -> Result<Item, sqlx::Error> {
    sqlx::query_as::<_, Item>(
        r#"INSERT INTO items (
               id, organization_id, code, name, description, item_type, uom_id, unit_price, income_account_id, tax_category_id, is_active)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) RETURNING *"#)
        .bind(Uuid::new_v4())
        .bind(&org_id)
        .bind(&item.code)
        .bind(&item.name)
        .bind(&item.description)
        .bind(&item.item_type)
        .bind(&item.uom_id)
        .bind(&item.unit_price)
        .bind(&item.income_account_id)
        .bind(&item.tax_category_id)
        .bind(true)
        .fetch_one(conn)
        .await
}

pub async fn update(
    conn: &mut PgConnection,
    id: Uuid,
    org_id: Uuid,
    item: &Item,
) -> Result<Item, sqlx::Error> {
    sqlx::query_as::<_, Item>(
        r#"UPDATE items SET
                 code = $1,
                 name = $2,
                 description = $3,
                 item_type = $4,
                 uom_id = $5,
                 unit_price = $6,
                 income_account_id = $7,
                 tax_category_id = $8,
                 is_active = $9
             WHERE id = $10 AND organization_id = $11 RETURNING *"#,
    )
    .bind(&item.code)
    .bind(&item.name)
    .bind(&item.description)
    .bind(&item.item_type)
    .bind(&item.uom_id)
    .bind(&item.unit_price)
    .bind(&item.income_account_id)
    .bind(&item.tax_category_id)
    .bind(&item.is_active)
    .bind(&id)
    .bind(&org_id)
    .fetch_one(conn)
    .await
}

pub async fn delete(conn: &mut PgConnection, id: Uuid, org_id: Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM items WHERE id = $1 AND organization_id = $11 ")
        .bind(&id)
        .bind(&org_id)
        .execute(conn)
        .await?;
    Ok(result.rows_affected())
}

pub async fn is_uom_in_use(conn: &mut PgConnection, uom_id: Uuid) -> Result<bool, sqlx::Error> {
    let exists: bool = sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM items WHERE uom_id = $1)")
        .bind(uom_id)
        .fetch_one(conn)
        .await?;
    Ok(exists)
}
