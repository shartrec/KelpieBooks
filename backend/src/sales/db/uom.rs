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
use shared_core::{
    sales::models::item::UnitOfMeasure,
    OrgId,
    UomId,
};

pub async fn all(
    conn: &mut PgConnection,
    org_id: OrgId,
) -> Result<Vec<UnitOfMeasure>, sqlx::Error> {
    sqlx::query_as!(
        UnitOfMeasure,
        r#"SELECT id, organization_id as org_id, code, name, is_active
            FROM units_of_measure
            WHERE organization_id = $1 ORDER BY code"#,
        *org_id,
    )
    .fetch_all(conn)
    .await
}

pub async fn get(
    conn: &mut PgConnection,
    org_id: OrgId,
    id: UomId,
) -> Result<Option<UnitOfMeasure>, sqlx::Error> {
    sqlx::query_as!(
        UnitOfMeasure,
        r#"SELECT id, organization_id as org_id, code, name, is_active
            FROM units_of_measure
            WHERE id = $1 AND organization_id = $2 ORDER BY code"#,
        *id,
        *org_id,
    )
    .fetch_optional(conn)
    .await
}

pub async fn create(
    conn: &mut PgConnection,
    org_id: OrgId,
    uom: &UnitOfMeasure,
) -> Result<UnitOfMeasure, sqlx::Error> {
    sqlx::query_as!(
        UnitOfMeasure,
        r#"INSERT INTO units_of_measure (organization_id, code, name, is_active) VALUES ($1, $2, $3, $4)
                      RETURNING id, organization_id as org_id, code, name, is_active"#,
        *org_id,
        &uom.code,
        &uom.name,
        uom.is_active,
    )
    .fetch_one(conn)
    .await
}

pub async fn update(
    conn: &mut PgConnection,
    org_id: OrgId,
    id: UomId,
    uom: &UnitOfMeasure,
) -> Result<UnitOfMeasure, sqlx::Error> {
    sqlx::query_as!(
        UnitOfMeasure,
        r#"UPDATE units_of_measure SET code = $1, name = $2, is_active = $3 WHERE id = $4 AND organization_id = $5
                      RETURNING id, organization_id as org_id, code, name, is_active"#,
        uom.code,
        uom.name,
        uom.is_active,
        *id,
        *org_id,
    )
    .fetch_one(conn)
    .await
}

pub async fn delete(
    conn: &mut PgConnection,
    org_id: OrgId,
    id: UomId,
) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
    sqlx::query!(
        "DELETE FROM units_of_measure WHERE id = $1 AND organization_id = $2",
        *id,
        *org_id,
    )
    .execute(conn)
    .await
}
