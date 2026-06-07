/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::NaiveDate;
use rocket_db_pools::sqlx::PgConnection;
use shared_core::ledger::models::account_category::AccountCategory;
use uuid::Uuid;

use crate::{
    db::organization,
    ledger::db::{
        account,
        account::get_all_by_category,
        journal_entry,
        transaction,
    },
    util::ApiError,
};

pub(crate) async fn close_financial_year(
    pool: &mut PgConnection,
    organization_id: Uuid,
    year_end: NaiveDate,
) -> Result<(), ApiError> {
    let mut total_credits = 0i64;
    let mut total_debits = 0i64;

    // 1. Create the Closing Journal Transaction
    let closing_tx = transaction::insert(
        pool,
        organization_id,
        year_end,
        &Some("Closing Entry".to_string()),
        &None,
    )
    .await?;

    // 2. Create Journal Entries for all Revenue and Expense accounts
    let income_accounts = get_all_by_category(
        pool,
        organization_id,
        &[AccountCategory::Revenue, AccountCategory::Expense],
    )
    .await?;
    for account in income_accounts {
        let balance =
            journal_entry::get_balance_up_to_date(pool, account.id, organization_id, year_end)
                .await?;
        if balance != 0 {
            let (debit, credit) = if balance > 0 {
                (0, balance)
            } else {
                (-balance, 0)
            };
            // Accumulate total debits and credits for the closing transaction
            total_credits += credit;
            total_debits += debit;
            journal_entry::insert(
                pool,
                closing_tx,
                account.id,
                debit,
                credit,
                Some("Closing Entry"),
            )
            .await?;
        }
    }

    // 3. Post the Net Income to Retained Earnings
    let retained_earnings_account =
        account::get_by_system_tag(pool, organization_id, "RetainedEarnings")
            .await?
            .ok_or_else(|| ApiError::NotFound("Retained Earnings account not found".to_string()))?;

    // calculate reversing entry for the closing transaction
    let (debit, credit) = if total_credits > total_debits {
        (total_credits - total_debits, 0)
    } else {
        (0, total_debits - total_credits)
    };
    journal_entry::insert(
        pool,
        closing_tx,
        retained_earnings_account.id,
        debit,
        credit,
        Some("Closing Net Income"),
    )
    .await?;

    // 5. Update Organization Record to lock the period
    organization::set_lock_date(pool, organization_id, Some(year_end)).await?;

    Ok(())
}
