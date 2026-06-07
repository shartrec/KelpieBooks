/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::{DateTime, NaiveDate, Utc};
use rocket_db_pools::sqlx::{self, PgConnection, Row};
use shared_core::ledger::dtos::journal_entry_detail::JournalEntryDetail;
use shared_core::ledger::models::journal_entry::JournalEntry;
use uuid::Uuid;

pub(crate) struct JournalEntryWithDate {
    pub(crate) id: Uuid,
    pub(crate) transaction_id: Uuid,
    pub(crate) account_id: Uuid,
    pub(crate) debit: i64,
    pub(crate) credit: i64,
    pub(crate) description: Option<String>,
    pub(crate) date: NaiveDate,
    pub(crate) created_at: DateTime<Utc>,
}

fn from_row_to_journal_entry(row: &sqlx::postgres::PgRow) -> JournalEntry {
    JournalEntry {
        id: row.get("id"),
        transaction_id: row.get("transaction_id"),
        account_id: row.get("account_id"),
        debit: row.get("debit"),
        credit: row.get("credit"),
        description: row.get("description"),
        created_at: row.get("created_at"),
    }
}

fn from_row_to_journal_entry_with_date(row: &sqlx::postgres::PgRow) -> JournalEntryWithDate {
    JournalEntryWithDate {
        id: row.get("id"),
        transaction_id: row.get("transaction_id"),
        account_id: row.get("account_id"),
        debit: row.get("debit"),
        credit: row.get("credit"),
        description: row.get("description"),
        date: row.get("date"),
        created_at: row.get("created_at"),
    }
}

fn from_row_to_journal_entry_detail(row: &sqlx::postgres::PgRow) -> JournalEntryDetail {
    JournalEntryDetail {
        id: row.get("id"),
        transaction_id: row.get("transaction_id"),
        account_id: row.get("account_id"),
        account_name: row.get("account_name"),
        debit: row.get("debit"),
        credit: row.get("credit"),
        description: row.get("description"),
        created_at: row.get("created_at"),
    }
}

pub(crate) async fn get_all_by_org(
    pool: &mut PgConnection,
    organization_id: Uuid,
) -> Result<Vec<JournalEntry>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT je.*
        FROM journal_entries je
        JOIN transactions t ON je.transaction_id = t.id
        WHERE t.organization_id = $1
        "#,
    )
    .bind(organization_id)
    .fetch_all(pool)
    .await
    .map(|rows| rows.iter().map(from_row_to_journal_entry).collect())
}

pub(crate) async fn get_all_by_account_with_date(
    pool: &mut PgConnection,
    account_id: Uuid,
) -> Result<Vec<JournalEntryWithDate>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT
            je.id,
            je.transaction_id,
            je.account_id,
            je.debit,
            je.credit,
            je.description,
            je.created_at,
            t.date
        FROM journal_entries je
        JOIN transactions t ON je.transaction_id = t.id
        WHERE je.account_id = $1
        ORDER BY t.date, je.created_at
        "#,
    )
    .bind(account_id)
    .fetch_all(pool)
    .await
    .map(|rows| {
        rows.iter()
            .map(from_row_to_journal_entry_with_date)
            .collect()
    })
}

pub(crate) async fn get_all_by_transaction(
    pool: &mut PgConnection,
    transaction_id: Uuid,
) -> Result<Vec<JournalEntryDetail>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT
            je.id,
            je.transaction_id,
            je.account_id,
            a.name as account_name,
            je.debit,
            je.credit,
            je.description,
            je.created_at
        FROM journal_entries je
        JOIN accounts a ON je.account_id = a.id
        WHERE je.transaction_id = $1
        ORDER BY je.debit DESC, je.credit
        "#,
    )
    .bind(transaction_id)
    .fetch_all(pool)
    .await
    .map(|rows| rows.iter().map(from_row_to_journal_entry_detail).collect())
}

pub(crate) async fn insert(
    pool: &mut PgConnection,
    transaction_id: Uuid,
    account_id: Uuid,
    debit: i64,
    credit: i64,
    description: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO journal_entries (transaction_id, account_id, debit, credit, description) VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(transaction_id)
    .bind(account_id)
    .bind(debit)
    .bind(credit)
    .bind(description)
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn get_balance_before_date(
    pool: &mut PgConnection,
    account_id: Uuid,
    org_id: Uuid,
    date: NaiveDate,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        SELECT COALESCE(SUM(debit - credit), 0)::BIGINT as balance
        FROM journal_entries je
        JOIN transactions t ON je.transaction_id = t.id
        JOIN accounts a ON je.account_id = a.id
        WHERE je.account_id = $1 AND a.organization_id = $2 AND t.date < $3
        "#,
    )
    .bind(account_id)
    .bind(org_id)
    .bind(date)
    .fetch_one(pool)
    .await?;

    Ok(result.get("balance"))
}

pub(crate) async fn get_all_by_account_in_date_range(
    pool: &mut PgConnection,
    account_id: Uuid,
    org_id: Uuid,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<JournalEntryWithDate>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT
            je.id,
            je.transaction_id,
            je.account_id,
            je.debit,
            je.credit,
            je.description,
            je.created_at,
            t.date
        FROM journal_entries je
        JOIN transactions t ON je.transaction_id = t.id
        JOIN accounts a ON je.account_id = a.id
        WHERE je.account_id = $1  AND a.organization_id = $2 AND t.date >= $3 AND t.date <= $4
        ORDER BY t.date, je.created_at
        "#,
    )
    .bind(account_id)
    .bind(org_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_all(pool)
    .await
    .map(|rows| {
        rows.iter()
            .map(from_row_to_journal_entry_with_date)
            .collect()
    })
}

pub(crate) async fn get_balance_up_to_date(
    pool: &mut PgConnection,
    account_id: Uuid,
    org_id: Uuid,
    date: NaiveDate,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        SELECT COALESCE(SUM(debit - credit), 0)::BIGINT as balance
        FROM journal_entries je
        JOIN transactions t ON je.transaction_id = t.id
        JOIN accounts a ON je.account_id = a.id
        WHERE je.account_id = $1  AND a.organization_id = $2 AND t.date <= $3
        "#,
    )
    .bind(account_id)
    .bind(org_id)
    .bind(date)
    .fetch_one(pool)
    .await?;

    Ok(result.get("balance"))
}
