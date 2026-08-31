/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::{
    DateTime,
    NaiveDate,
    Utc,
};
use rocket_db_pools::sqlx::{
    self,
    PgConnection,
};
use rust_decimal::Decimal;
use shared_core::{
    ledger::{
        dtos::journal_entry_detail::JournalEntryDetail,
        models::journal_entry::JournalEntry,
    },
    AccountId,
    JournalEntryId,
    OrgId,
    TransactionId,
};

pub(crate) struct JournalEntryWithDate {
    pub(crate) id: JournalEntryId,
    pub(crate) transaction_id: TransactionId,
    pub(crate) account_id: AccountId,
    pub(crate) debit: Decimal,
    pub(crate) credit: Decimal,
    pub(crate) description: Option<String>,
    pub(crate) date: NaiveDate,
    pub(crate) created_at: DateTime<Utc>,
}

pub(crate) async fn get_all_by_org(
    pool: &mut PgConnection,
    org_id: OrgId,
) -> Result<Vec<JournalEntry>, sqlx::Error> {
    sqlx::query_as!(
        JournalEntry,
        r#"
        SELECT je.*
        FROM journal_entries je
        JOIN transactions t ON je.transaction_id = t.id
        WHERE t.organization_id = $1
        "#,
        *org_id
    )
    .fetch_all(pool)
    .await
}

pub(crate) async fn get_all_by_transaction(
    pool: &mut PgConnection,
    transaction_id: TransactionId,
) -> Result<Vec<JournalEntryDetail>, sqlx::Error> {
    sqlx::query_as!(
        JournalEntryDetail,
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
        *transaction_id
    )
    .fetch_all(pool)
    .await
}

pub(crate) async fn insert(
    pool: &mut PgConnection,
    transaction_id: TransactionId,
    account_id: AccountId,
    debit: Decimal,
    credit: Decimal,
    description: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO journal_entries (transaction_id, account_id, debit, credit, description) VALUES ($1, $2, $3, $4, $5)",
        *transaction_id,
        *account_id,
        debit,
        credit,
        description
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn get_balance_before_date(
    pool: &mut PgConnection,
    org_id: OrgId,
    account_id: AccountId,
    date: NaiveDate,
) -> Result<Decimal, sqlx::Error> {
    let result = sqlx::query_scalar!(
        r#"
        SELECT COALESCE(SUM(debit - credit), 0)::NUMERIC(15,4) as balance
        FROM journal_entries je
        JOIN transactions t ON je.transaction_id = t.id
        JOIN accounts a ON je.account_id = a.id
        WHERE je.account_id = $1 AND a.organization_id = $2 AND t.date < $3
        "#,
        *account_id,
        *org_id,
        date
    )
    .fetch_one(pool)
    .await?;

    Ok(result.unwrap_or(Decimal::ZERO))
}

pub(crate) async fn get_all_by_account_in_date_range(
    pool: &mut PgConnection,
    org_id: OrgId,
    account_id: AccountId,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<JournalEntryWithDate>, sqlx::Error> {
    sqlx::query_as!(
        JournalEntryWithDate,
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
        *account_id,
        *org_id,
        start_date,
        end_date
    )
    .fetch_all(pool)
    .await
}

pub(crate) async fn get_balance_up_to_date(
    pool: &mut PgConnection,
    org_id: OrgId,
    account_id: AccountId,
    date: NaiveDate,
) -> Result<Decimal, sqlx::Error> {
    let result = sqlx::query_scalar!(
        r#"
        SELECT COALESCE(SUM(debit - credit), 0)::NUMERIC(15,4) as balance
        FROM journal_entries je
        JOIN transactions t ON je.transaction_id = t.id
        JOIN accounts a ON je.account_id = a.id
        WHERE je.account_id = $1  AND a.organization_id = $2 AND t.date <= $3
        "#,
        *account_id,
        *org_id,
        date
    )
    .fetch_one(pool)
    .await?;

    Ok(result.unwrap_or(Decimal::ZERO))
}
