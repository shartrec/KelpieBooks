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
use shared_core::OrgId;

pub(crate) async fn get_active_uoms(
    conn: &mut PgConnection,
    org_id: OrgId,
) -> Result<Vec<UnitOfMeasure>, sqlx::Error> {
    sqlx::query_as!(
        UnitOfMeasure,
        r#"SELECT id, organization_id as org_id ,code, name, is_active FROM units_of_measure
                             WHERE organization_id = $1 AND is_active = true ORDER BY name ASC"#,
        *org_id,
    )
    .fetch_all(conn)
    .await
}

pub async fn all(
    conn: &mut PgConnection,
    org_id: OrgId,
    search_term: Option<String>,
    item_type: Option<ItemType>,
    include_inactive: bool,
    limit: u32,
) -> Result<Vec<Item>, sqlx::Error> {
    let mut query = sqlx::QueryBuilder::<sqlx::Postgres>::new(
        r#"SELECT id,
               organization_id,
               code,
               name,
               description,
               item_type,
               uom_id,
               unit_price,
               purchase_unit_cost,
               income_account_id,
               tax_category_id,
               is_active,
               created_at
        FROM items
        WHERE organization_id =
        "#,
    );

    query.push_bind(org_id);

    if let Some(term) = search_term {
        let term = format!("%{}%", term);
        query.push(" AND (code ILIKE ");
        query.push_bind(term.clone());
        query.push(" OR name ILIKE ");
        query.push_bind(term);
        query.push(" )");
    }

    if let Some(it_type) = item_type {
        query.push(" AND item_type = ");
        query.push_bind(it_type);
    }

    if !include_inactive {
        query.push(" AND is_active = true");
    }

    query.push(" ORDER BY code");

    query.push(" LIMIT ");
    query.push_bind(limit as i32);

    query.build_query_as::<Item>().fetch_all(conn).await
}

pub async fn get(
    conn: &mut PgConnection,
    org_id: OrgId,
    id: Uuid,
) -> Result<Option<Item>, sqlx::Error> {
    sqlx::query_as!(
        Item,
        r#"SELECT id,
               organization_id as org_id,
               code,
               name,
               description,
               item_type AS "item_type: ItemType",
               uom_id,
               unit_price,
               purchase_unit_cost as unit_cost,
               income_account_id,
               tax_category_id,
               is_active,
               created_at
        FROM items WHERE id = $1 AND organization_id = $2
        "#,
        id,
        *org_id,
    )
    .fetch_optional(conn)
    .await
}

pub async fn create(
    conn: &mut PgConnection,
    org_id: OrgId,
    item: &CreateItemRequest,
) -> Result<Item, sqlx::Error> {
    sqlx::query_as!(
        Item,
        r#"INSERT INTO items (
               id, organization_id, code, name, description, item_type, uom_id, unit_price, purchase_unit_cost, income_account_id, tax_category_id, is_active)
               VALUES ($1, $2, $3, $4, $5, $6:: item_type, $7, $8, $9, $10, $11, $12) RETURNING
                   id,
                   organization_id as org_id,
                   code,
                   name,
                   description,
                   item_type AS "item_type: ItemType",
                   uom_id,
                   unit_price,
                   purchase_unit_cost as unit_cost,
                   income_account_id,
                   tax_category_id,
                   is_active,
                   created_at
               "#,
        Uuid::new_v4(),
        *org_id,
        item.code,
        item.name,
        item.description,
        item.item_type as ItemType,
        item.uom_id,
        item.unit_price,
        item.unit_cost,
        *item.income_account_id,
        item.tax_category_id,
        true,
        )
        .fetch_one(conn)
        .await
}

pub async fn update(
    conn: &mut PgConnection,
    org_id: OrgId,
    id: Uuid,
    item: &Item,
) -> Result<Item, sqlx::Error> {
    sqlx::query_as!(
        Item,
        r#"UPDATE items SET
                 code = $1,
                 name = $2,
                 description = $3,
                 item_type = $4::item_type,
                 uom_id = $5,
                 unit_price = $6,
                 purchase_unit_cost = $7,
                 income_account_id = $8,
                 tax_category_id = $9,
                 is_active = $10
             WHERE id = $11 AND organization_id = $12 RETURNING
                id,
                organization_id as org_id,
                code,
                name,
                description,
                item_type AS "item_type: ItemType",
                uom_id,
                unit_price,
                purchase_unit_cost as unit_cost,
                income_account_id,
                tax_category_id,
                is_active,
                created_at
           "#,
        item.code,
        item.name,
        item.description,
        item.item_type as ItemType,
        item.uom_id,
        item.unit_price,
        item.unit_cost,
        *item.income_account_id,
        item.tax_category_id,
        item.is_active,
        id,
        *org_id,
    )
    .fetch_one(conn)
    .await
}

pub async fn delete(conn: &mut PgConnection, org_id: OrgId, id: Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        "DELETE FROM items WHERE id = $1 AND organization_id = $2",
        &id,
        *org_id
    )
    .execute(conn)
    .await?;
    Ok(result.rows_affected())
}

pub async fn is_uom_in_use(conn: &mut PgConnection, uom_id: Uuid) -> Result<bool, sqlx::Error> {
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS (SELECT 1 FROM items WHERE uom_id = $1)",
        &uom_id
    )
    .fetch_one(conn)
    .await?;
    Ok(exists.unwrap_or(false))
}
