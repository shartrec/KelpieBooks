/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use rocket_db_pools::sqlx::{self, PgConnection};
use shared_core::sales::models::tax::TaxCategory;
use uuid::Uuid;

pub(crate) async fn get_active_tax_categories(pool: &mut PgConnection, org_id: Uuid) -> Result<Vec<TaxCategory>, sqlx::Error> {
    sqlx::query_as(
        r#"SELECT c.id, c.organization_id, c.name, c.description, r.rate, c.is_active FROM tax_categories c, tax_rates r
                 WHERE c.organization_id = $1 AND is_active = true
                 AND c.id = r.tax_category_id
                 AND NOW() BETWEEN r.valid_from AND r.valid_to
                 ORDER BY c.name ASC"#
    )
        .bind(org_id)
        .fetch_all(pool)
        .await
}

pub async fn all(conn: &mut PgConnection, org_id: Uuid) -> Result<Vec<TaxCategory>, sqlx::Error> {
    sqlx::query_as(r#"SELECT c.id, c.organization_id, c.name, c.description, 0.00 as rate, c.is_active FROM tax_categories c
                       WHERE organization_id = $1"#)
        .bind(org_id)
        .fetch_all(conn)
        .await
}

pub async fn get(conn: &mut PgConnection, id: Uuid, org_id: Uuid) -> Result<Option<TaxCategory>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM tax_categories WHERE id = $1 AND organization_id = $2")
        .bind(id)
        .bind(org_id)
        .fetch_optional(conn)
        .await
}

pub async fn create(conn: &mut PgConnection, org_id: Uuid, tax_category: &TaxCategory) -> Result<TaxCategory, sqlx::Error> {
    sqlx::query_as(
        "INSERT INTO tax_categories (id, organization_id, name, description, is_active) VALUES ($1, $2, $3, $4, $5) RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(org_id)
    .bind(&tax_category.name)
    .bind(&tax_category.description)
    .bind(tax_category.is_active)
    .fetch_one(conn)
    .await
}

pub async fn update(conn: &mut PgConnection, id: Uuid, org_id: Uuid, tax_category: &TaxCategory) -> Result<TaxCategory, sqlx::Error> {
    sqlx::query_as(
        "UPDATE tax_categories SET name = $1, description = $2, is_active = $3 WHERE id = $4 AND organization_id = $5 RETURNING *",
    )
    .bind(&tax_category.name)
    .bind(&tax_category.description)
    .bind(tax_category.is_active)
    .bind(id)
    .bind(org_id)
    .fetch_one(conn)
    .await
}

pub async fn delete(conn: &mut PgConnection, id: Uuid, org_id: Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM tax_categories WHERE id = $1 AND organization_id = $2")
        .bind(id)
        .bind(org_id)
        .execute(conn)
        .await?;
    Ok(result.rows_affected())
}
