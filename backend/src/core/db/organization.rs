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
};
use shared_core::core::models::organization::Organization;
use uuid::Uuid;

pub(crate) async fn get(
    pool: &mut PgConnection,
    id: Uuid,
) -> Result<Option<Organization>, sqlx::Error> {
    sqlx::query_as!(Organization,
        "SELECT * FROM organizations WHERE id = $1",
        id
    )
        .fetch_optional(pool)
        .await
}

pub(crate) async fn set_lock_date(
    pool: &mut PgConnection,
    id: Uuid,
    date: Option<NaiveDate>,
) -> Result<(), sqlx::Error> {
    sqlx::query!("UPDATE organizations SET locked_until = $1 WHERE id = $2",
        date,
        id
    )
        .execute(pool)
        .await?;
    Ok(())
}
pub(crate) async fn set_audit_mode(
    pool: &mut PgConnection,
    id: Uuid,
    mode: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query!("UPDATE organizations SET strict_audit_mode = $1 WHERE id = $2",
        mode,
        id
    )
        .execute(pool)
        .await?;
    Ok(())
}

pub(crate) async fn create(tx: &mut PgConnection, name: &str) -> Result<Organization, sqlx::Error> {
    let row = sqlx::query_as!(Organization,
        "INSERT INTO organizations (name) VALUES ($1) RETURNING *",
        name
    )
        .fetch_one(tx)
        .await?;
    Ok(row)
}
