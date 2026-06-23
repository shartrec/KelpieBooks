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
use shared_core::sales::models::item::UnitOfMeasure;
use uuid::Uuid;

pub async fn all(conn: &mut PgConnection, org_id: Uuid) -> Result<Vec<UnitOfMeasure>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM units_of_measure WHERE organization_id = $1 ORDER BY code")
        .bind(org_id)
        .fetch_all(conn)
        .await
}

pub async fn get(
    conn: &mut PgConnection,
    id: Uuid,
    org_id: Uuid,
) -> Result<Option<UnitOfMeasure>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM units_of_measure WHERE id = $1 AND organization_id = $2")
        .bind(id)
        .bind(org_id)
        .fetch_optional(conn)
        .await
}

pub async fn create(
    conn: &mut PgConnection,
    org_id: Uuid,
    uom: &UnitOfMeasure,
) -> Result<UnitOfMeasure, sqlx::Error> {
    sqlx::query_as(
        "INSERT INTO units_of_measure (id, organization_id, code, name, is_active) VALUES ($1, $2, $3, $4, $5) RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(org_id)
    .bind(&uom.code)
    .bind(&uom.name)
    .bind(uom.is_active)
    .fetch_one(conn)
    .await
}

pub async fn update(
    conn: &mut PgConnection,
    id: Uuid,
    org_id: Uuid,
    uom: &UnitOfMeasure,
) -> Result<UnitOfMeasure, sqlx::Error> {
    sqlx::query_as(
        "UPDATE units_of_measure SET code = $1, name = $2, is_active = $3 WHERE id = $4 AND organization_id = $5 RETURNING *",
    )
    .bind(&uom.code)
    .bind(&uom.name)
    .bind(uom.is_active)
    .bind(id)
    .bind(org_id)
    .fetch_one(conn)
    .await
}

pub async fn delete(
    conn: &mut PgConnection,
    id: Uuid,
    org_id: Uuid,
) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
    sqlx::query("DELETE FROM units_of_measure WHERE id = $1 AND organization_id = $2")
        .bind(id)
        .bind(org_id)
        .execute(conn)
        .await
}
