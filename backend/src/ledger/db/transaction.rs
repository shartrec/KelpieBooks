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
use shared_core::ledger::models::transaction::Transaction;
use shared_core::{OrgId, TransactionId};

pub(crate) async fn get(
    pool: &mut PgConnection,
    org_id: OrgId,
    id: TransactionId,
) -> Result<Option<Transaction>, sqlx::Error> {
    sqlx::query_as!(
        Transaction,
        "SELECT id, organization_id, date, description, reference, created_at FROM transactions WHERE id = $1 AND organization_id = $2",
        *id,
        *org_id,
    )
    .fetch_optional(pool)
    .await
}

pub(crate) async fn get_recent_transactions(
    pool: &mut PgConnection,
    org_id: OrgId,
    limit: i64,
) -> Result<Vec<Transaction>, sqlx::Error> {
    sqlx::query_as!(
            Transaction,
            r#"SELECT id,  organization_id, date, description, reference, created_at
                FROM transactions WHERE organization_id = $1 ORDER BY date DESC, created_at DESC LIMIT $2"#,
            *org_id,
            limit
        )
        .fetch_all(pool)
        .await
}

pub(crate) async fn insert(
    pool: &mut PgConnection,
    org_id: OrgId,
    date: NaiveDate,
    description: &Option<String>,
    reference: &Option<String>,
) -> Result<TransactionId, sqlx::Error> {
    let t = sqlx::query_as!(
        Transaction,
        r#"INSERT INTO transactions (organization_id, date, description, reference) VALUES ($1, $2, $3, $4)
             RETURNING id, organization_id, date, description, reference, created_at"#,
        *org_id,
        date,
        description.as_deref(),
        reference.as_deref(),
    )
    .fetch_one(pool)
    .await?;
    Ok(t.id)
}

pub(crate) async fn delete(
    pool: &mut PgConnection,
    org_id: OrgId,
    id: TransactionId,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "DELETE FROM transactions WHERE id = $1 and organization_id = $2",
        *id,
        *org_id,
    )
    .execute(pool)
    .await?;
    Ok(())
}
