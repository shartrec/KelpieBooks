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
use shared_core::ledger::models::transaction::Transaction;
use uuid::Uuid;

fn from_row_to_transaction(row: &sqlx::postgres::PgRow) -> Transaction {
    Transaction {
        id: row.get("id"),
        organization_id: row.get("organization_id"),
        date: row.get("date"),
        description: row.get("description"),
        reference: row.get("reference"),
        created_at: row.get("created_at"),
    }
}

pub(crate) async fn get(
    pool: &mut PgConnection,
    id: Uuid,
    org_id: Uuid,
) -> Result<Option<Transaction>, sqlx::Error> {
    sqlx::query("SELECT * FROM transactions WHERE id = $1 AND organization_id = $2")
        .bind(id)
        .bind(org_id)
        .fetch_optional(pool)
        .await
        .map(|row| row.map(|r| from_row_to_transaction(&r)))
}

pub(crate) async fn get_recent_transactions(
    pool: &mut PgConnection,
    organization_id: Uuid,
    limit: i64,
) -> Result<Vec<Transaction>, sqlx::Error> {
    sqlx::query("SELECT * FROM transactions WHERE organization_id = $1 ORDER BY date DESC, created_at DESC LIMIT $2")
        .bind(organization_id)
        .bind(limit)
        .fetch_all(pool)
        .await
        .map(|rows| rows.iter().map(from_row_to_transaction).collect())
}

pub(crate) async fn insert(
    pool: &mut PgConnection,
    organization_id: Uuid,
    date: NaiveDate,
    description: &Option<String>,
    reference: &Option<String>,
) -> Result<Uuid, sqlx::Error> {
    let row = sqlx::query(
        "INSERT INTO transactions (organization_id, date, description, reference) VALUES ($1, $2, $3, $4) RETURNING id"
    )
    .bind(organization_id)
    .bind(date)
    .bind(description)
    .bind(reference)
    .fetch_one(pool)
    .await?;
    Ok(row.get("id"))
}

pub(crate) async fn delete(
    pool: &mut PgConnection,
    id: Uuid,
    org_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM transactions WHERE id = $1 and organization_id = $2")
        .bind(id)
        .bind(org_id)
        .execute(pool)
        .await?;
    Ok(())
}
