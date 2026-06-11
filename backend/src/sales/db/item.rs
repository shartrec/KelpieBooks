/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use rocket_db_pools::sqlx::{self, PgConnection};
use shared_core::sales::models::item::{Item, UnitOfMeasure};
use uuid::Uuid;

pub(crate) async fn get_active_uoms(conn: &mut PgConnection) -> Result<Vec<UnitOfMeasure>, sqlx::Error> {
    sqlx::query_as::<_, UnitOfMeasure>(
        "SELECT id, code, name, is_active FROM units_of_measure WHERE is_active = true ORDER BY name ASC"
    )
        .fetch_all(conn)
        .await
}

pub async fn all(conn: &mut PgConnection) -> Result<Vec<Item>, sqlx::Error> {
    sqlx::query_as::<_, Item>("SELECT * FROM items")
        .fetch_all(conn)
        .await
}

pub async fn get(conn: &mut PgConnection, id: Uuid) -> Result<Option<Item>, sqlx::Error> {
    sqlx::query_as::<_, Item>("SELECT * FROM items WHERE id = $1")
        .bind(id)
        .fetch_optional(conn)
        .await
}

pub async fn create(conn: &mut PgConnection, item: &Item) -> Result<Item, sqlx::Error> {
    sqlx::query_as::<_, Item>(
        r#"INSERT INTO items (
               id, code,name, description, item_type, uom_id, unit_price, income_account_id, tax_category_id, is_active)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *"#)
        .bind(&item.id)
        .bind(&item.code)
        .bind(&item.name)
        .bind(&item.description)
        .bind(&item.item_type)
        .bind(&item.uom_id)
        .bind(&item.unit_price)
        .bind(&item.income_account_id)
        .bind(&item.tax_category_id)
        .bind(&item.is_active)
        .fetch_one(conn)
        .await
}

pub async fn update(conn: &mut PgConnection, id: Uuid, item: &Item) -> Result<Item, sqlx::Error> {
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
             WHERE id = $10 RETURNING *"#)
        .bind(&item.code)
        .bind(&item.name)
        .bind(&item.description)
        .bind(&item.item_type)
        .bind(&item.uom_id)
        .bind(&item.unit_price)
        .bind(&item.income_account_id)
        .bind(&item.tax_category_id)
        .bind(&item.is_active)
        .bind(&item.id)

    .fetch_one(conn)
    .await
}

pub async fn delete(conn: &mut PgConnection, id: Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM items WHERE id = $1")
        .bind(id)
        .execute(conn)
        .await?;
    Ok(result.rows_affected())
}
