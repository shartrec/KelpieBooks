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
use shared_core::sales::models::tax::TaxCategory;
use shared_core::{OrgId, TaxCategoryId};

pub async fn all(conn: &mut PgConnection, org_id: OrgId) -> Result<Vec<TaxCategory>, sqlx::Error> {
    sqlx::query_as!(
        TaxCategory,
        r#"SELECT c.id, c.organization_id as org_id, c.name, c.description, c.is_active FROM tax_categories c
                       WHERE organization_id = $1"#,
            *org_id,
        )
        .fetch_all(conn)
        .await
}

pub async fn get(
    conn: &mut PgConnection,
    org_id: OrgId,
    id: TaxCategoryId,
) -> Result<Option<TaxCategory>, sqlx::Error> {
    sqlx::query_as!(
        TaxCategory,
        "SELECT c.id, c.organization_id as org_id, c.name, c.description,c.is_active FROM tax_categories c
                       WHERE id = $1 AND organization_id = $2",
            *id,
            *org_id,
        )
        .fetch_optional(conn)
        .await
}

pub async fn create(
    conn: &mut PgConnection,
    org_id: OrgId,
    tax_category: &TaxCategory,
) -> Result<TaxCategory, sqlx::Error> {
    sqlx::query_as!(
        TaxCategory,
        "INSERT INTO tax_categories (organization_id, name, description, is_active) VALUES ($1, $2, $3, $4) RETURNING
                    id, organization_id as org_id, name, description, is_active ",
        *org_id,
        tax_category.name,
        tax_category.description,
        tax_category.is_active,
    )
    .fetch_one(conn)
    .await
}

pub async fn update(
    conn: &mut PgConnection,
    org_id: OrgId,
    id: TaxCategoryId,
    tax_category: &TaxCategory,
) -> Result<TaxCategory, sqlx::Error> {
    sqlx::query_as!(
        TaxCategory,
        "UPDATE tax_categories SET name = $1, description = $2, is_active = $3 WHERE id = $4 AND organization_id = $5 RETURNING
                    id, organization_id as org_id, name, description, is_active ",
        tax_category.name,
        tax_category.description,
        tax_category.is_active,
        *id,
        *org_id,
    )
    .fetch_one(conn)
    .await
}

pub async fn delete(conn: &mut PgConnection, org_id: OrgId, id: TaxCategoryId) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        "DELETE FROM tax_categories WHERE id = $1 AND organization_id = $2",
        *id,
        *org_id,
    )
    .execute(conn)
    .await?;
    Ok(result.rows_affected())
}
