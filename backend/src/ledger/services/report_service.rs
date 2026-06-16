/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use std::collections::{
    HashMap,
    VecDeque,
};

use chrono::NaiveDate;
use rocket_db_pools::sqlx::{
    PgConnection,
    Row,
};
use rust_decimal::{dec, Decimal};
use shared_core::ledger::{
    dtos::{
        account_with_balance::AccountWithBalance,
        balance_sheet::BalanceSheet,
        general_ledger_line::GeneralLedgerLine,
    },
    models::account_category::AccountCategory,
};
use uuid::Uuid;

use crate::{
    ledger::db::account::get_all_by_org,
    util::ApiError,
};

pub(crate) async fn get_profit_loss(
    pool: &mut PgConnection,
    organization_id: Uuid,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<AccountWithBalance>, ApiError> {
    let accounts = get_all_by_org(pool, organization_id).await?;

    let entries = sqlx::query(
        r#"
        SELECT je.account_id, je.debit, je.credit
        FROM journal_entries je
        JOIN transactions t ON je.transaction_id = t.id
        WHERE t.organization_id = $1 AND t.date >= $2 AND t.date <= $3
        "#,
    )
    .bind(organization_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;

    let mut balances: HashMap<Uuid, Decimal> = HashMap::new();

    for entry in &entries {
        *balances.entry(entry.get("account_id")).or_insert(dec!(0.00)) +=
            entry.get::<Decimal, _>("debit") - entry.get::<Decimal, _>("credit");
    }

    let mut parent_map: HashMap<Uuid, Uuid> = HashMap::new();
    let mut child_count: HashMap<Uuid, usize> = HashMap::new();

    for account in &accounts {
        child_count.entry(account.id).or_insert(0);
        if let Some(parent_id) = account.parent_id {
            parent_map.insert(account.id, parent_id);
            *child_count.entry(parent_id).or_insert(0) += 1;
        }
    }

    let mut queue: VecDeque<Uuid> = child_count
        .iter()
        .filter(|(_, &count)| count == 0)
        .map(|(&id, _)| id)
        .collect();

    while let Some(account_id) = queue.pop_front() {
        if let Some(&parent_id) = parent_map.get(&account_id) {
            let balance = *balances.get(&account_id).unwrap_or(&dec!(0.00));
            *balances.entry(parent_id).or_insert(dec!(0.00)) += balance;

            if let Some(count) = child_count.get_mut(&parent_id) {
                *count -= 1;
                if *count == 0 {
                    queue.push_back(parent_id);
                }
            }
        }
    }

    let result = accounts
        .into_iter()
        .filter(|acc| {
            acc.category == AccountCategory::Revenue || acc.category == AccountCategory::Expense
        })
        .map(|acc| AccountWithBalance {
            balance: *balances.get(&acc.id).unwrap_or(&dec!(0.00)),
            id: acc.id,
            organization_id: acc.organization_id,
            parent_id: acc.parent_id,
            code: acc.code,
            name: acc.name,
            category: acc.category,
            is_group: acc.is_group,
            is_bank_account: acc.is_bank_account,
            system_tag: acc.system_tag,
            created_at: acc.created_at,
        })
        .collect();

    Ok(result)
}

pub(crate) async fn get_expense_breakdown(
    pool: &mut PgConnection,
    organization_id: Uuid,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<AccountWithBalance>, ApiError> {
    let accounts = get_all_by_org(pool, organization_id).await?;

    let entries = sqlx::query(
        r#"
        SELECT je.account_id, je.debit, je.credit
        FROM journal_entries je
        JOIN transactions t ON je.transaction_id = t.id
        WHERE t.organization_id = $1 AND t.date >= $2 AND t.date <= $3
        "#,
    )
    .bind(organization_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;

    let mut balances: HashMap<Uuid, Decimal> = HashMap::new();

    for entry in &entries {
        *balances.entry(entry.get("account_id")).or_insert(dec!(0.00)) +=
            entry.get::<Decimal, _>("debit") - entry.get::<Decimal, _>("credit");
    }

    let mut parent_map: HashMap<Uuid, Uuid> = HashMap::new();
    let mut child_count: HashMap<Uuid, usize> = HashMap::new();

    for account in &accounts {
        child_count.entry(account.id).or_insert(0);
        if let Some(parent_id) = account.parent_id {
            parent_map.insert(account.id, parent_id);
            *child_count.entry(parent_id).or_insert(0) += 1;
        }
    }

    let mut queue: VecDeque<Uuid> = child_count
        .iter()
        .filter(|(_, &count)| count == 0)
        .map(|(&id, _)| id)
        .collect();

    while let Some(account_id) = queue.pop_front() {
        if let Some(&parent_id) = parent_map.get(&account_id) {
            let balance = *balances.get(&account_id).unwrap_or(&dec!(0.00));
            *balances.entry(parent_id).or_insert(dec!(0.00)) += balance;

            if let Some(count) = child_count.get_mut(&parent_id) {
                *count -= 1;
                if *count == 0 {
                    queue.push_back(parent_id);
                }
            }
        }
    }

    let result = accounts
        .into_iter()
        .filter(|acc| acc.category == AccountCategory::Expense && acc.parent_id.is_none())
        .map(|acc| AccountWithBalance {
            balance: *balances.get(&acc.id).unwrap_or(&dec!(0.00)),
            id: acc.id,
            organization_id: acc.organization_id,
            parent_id: acc.parent_id,
            code: acc.code,
            name: acc.name,
            category: acc.category,
            is_group: acc.is_group,
            is_bank_account: acc.is_bank_account,
            system_tag: acc.system_tag,
            created_at: acc.created_at,
        })
        .collect();

    Ok(result)
}

pub(crate) async fn get_balance_sheet(
    pool: &mut PgConnection,
    organization_id: Uuid,
    date: NaiveDate,
) -> Result<BalanceSheet, ApiError> {
    let accounts = get_all_by_org(pool, organization_id).await?;

    let entries = sqlx::query(
        r#"
        SELECT je.account_id, je.debit, je.credit
        FROM journal_entries je
        JOIN transactions t ON je.transaction_id = t.id
        WHERE t.organization_id = $1 AND t.date <= $2
        "#,
    )
    .bind(organization_id)
    .bind(date)
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;

    let mut balances: HashMap<Uuid, Decimal> = HashMap::new();

    for entry in &entries {
        let debit: Decimal = entry.get("debit");
        let credit: Decimal = entry.get("credit");
        *balances.entry(entry.get("account_id")).or_insert(dec!(0.00)) += debit - credit;
    }

    let mut parent_map: HashMap<Uuid, Uuid> = HashMap::new();
    let mut child_count: HashMap<Uuid, usize> = HashMap::new();

    for account in &accounts {
        child_count.entry(account.id).or_insert(0);
        if let Some(parent_id) = account.parent_id {
            parent_map.insert(account.id, parent_id);
            *child_count.entry(parent_id).or_insert(0) += 1;
        }
    }

    let mut queue: VecDeque<Uuid> = child_count
        .iter()
        .filter(|(_, &count)| count == 0)
        .map(|(&id, _)| id)
        .collect();

    while let Some(account_id) = queue.pop_front() {
        if let Some(&parent_id) = parent_map.get(&account_id) {
            let balance = *balances.get(&account_id).unwrap_or(&dec!(0.00));
            *balances.entry(parent_id).or_insert(dec!(0.00)) += balance;

            if let Some(count) = child_count.get_mut(&parent_id) {
                *count -= 1;
                if *count == 0 {
                    queue.push_back(parent_id);
                }
            }
        }
    }

    let mut assets = Vec::new();
    let mut liabilities = Vec::new();
    let mut equity = Vec::new();
    let mut total_assets = dec!(0.00);
    let mut total_liabilities = dec!(0.00);
    let mut total_equity = dec!(0.00);
    let mut net_income = dec!(0.00);

    for acc in &accounts {
        let balance = *balances.get(&acc.id).unwrap_or(&dec!(0.00));

        if acc.parent_id.is_none() {
            match acc.category {
                AccountCategory::Asset => total_assets += balance,
                AccountCategory::Liability => total_liabilities += balance,
                AccountCategory::Equity => total_equity += balance,
                AccountCategory::Revenue => net_income -= balance,
                AccountCategory::Expense => net_income += balance,
            }
        }
    }

    for acc in accounts {
        let balance = *balances.get(&acc.id).unwrap_or(&dec!(0.00));
        let account_with_balance = AccountWithBalance {
            balance,
            id: acc.id,
            organization_id: acc.organization_id,
            parent_id: acc.parent_id,
            code: acc.code,
            name: acc.name,
            category: acc.category,
            is_group: acc.is_group,
            is_bank_account: acc.is_bank_account,
            system_tag: acc.system_tag,
            created_at: acc.created_at,
        };

        match acc.category {
            AccountCategory::Asset => assets.push(account_with_balance),
            AccountCategory::Liability => liabilities.push(account_with_balance),
            AccountCategory::Equity => equity.push(account_with_balance),
            _ => (),
        }
    }

    total_equity += net_income;

    Ok(BalanceSheet {
        assets,
        liabilities,
        equity,
        total_assets,
        total_liabilities,
        total_equity,
        net_income,
    })
}

pub(crate) async fn get_trial_balance(
    pool: &mut PgConnection,
    organization_id: Uuid,
    date: NaiveDate,
) -> Result<Vec<AccountWithBalance>, ApiError> {
    let accounts = get_all_by_org(pool, organization_id).await?;

    let entries = sqlx::query(
        r#"
        SELECT je.account_id, je.debit, je.credit
        FROM journal_entries je
        JOIN transactions t ON je.transaction_id = t.id
        WHERE t.organization_id = $1 AND t.date <= $2
        "#,
    )
    .bind(organization_id)
    .bind(date)
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;

    let mut balances: HashMap<Uuid, Decimal> = HashMap::new();

    for entry in &entries {
        *balances.entry(entry.get("account_id")).or_insert(dec!(0.00)) +=
            entry.get::<Decimal, _>("debit") - entry.get::<Decimal, _>("credit");
    }

    let result = accounts
        .into_iter()
        .map(|acc| AccountWithBalance {
            balance: *balances.get(&acc.id).unwrap_or(&dec!(0.00)),
            id: acc.id,
            organization_id: acc.organization_id,
            parent_id: acc.parent_id,
            code: acc.code,
            name: acc.name,
            category: acc.category,
            is_group: acc.is_group,
            is_bank_account: acc.is_bank_account,
            system_tag: acc.system_tag,
            created_at: acc.created_at,
        })
        .collect();

    Ok(result)
}

pub(crate) async fn get_general_ledger(
    pool: &mut PgConnection,
    organization_id: Uuid,
    start_date: NaiveDate,
    end_date: NaiveDate,
    account_ids: Option<Vec<Uuid>>,
    min_amount: Option<Decimal>,
) -> Result<Vec<GeneralLedgerLine>, ApiError> {
    let account_ids = account_ids.unwrap_or_default();
    let min_amount = min_amount.unwrap_or(dec!(0.00));

    let rows = sqlx::query(
        r#"
        SELECT
            t.id as transaction_id,
            je.id as journal_entry_id,
            t.date,
            a.id as account_id,
            a.name as account_name,
            a.code,
            je.description,
            je.debit,
            je.credit
        FROM journal_entries je
        JOIN transactions t ON je.transaction_id = t.id
        JOIN accounts a ON je.account_id = a.id
        WHERE t.organization_id = $1
          AND t.date >= $2
          AND t.date <= $3
          AND (a.category = $6 OR a.category = $7)
          AND (CARDINALITY($4::uuid[]) = 0 OR a.id = ANY($4))
          AND (je.debit >= $5 OR je.credit >= $5)
        ORDER BY a.code ASC, t.date ASC
        "#,
    )
    .bind(organization_id)
    .bind(start_date)
    .bind(end_date)
    .bind(&account_ids)
    .bind(min_amount)
    .bind(AccountCategory::Revenue)
    .bind(AccountCategory::Expense)
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;

    let mut result = Vec::new();
    let mut balances: HashMap<Uuid, Decimal> = HashMap::new();

    for row in rows {
        let balance = balances.entry(row.get("account_id")).or_insert(dec!(0.00));
        *balance += row.get::<Decimal, _>("debit") - row.get::<Decimal, _>("credit");

        result.push(GeneralLedgerLine {
            transaction_id: row.get("transaction_id"),
            journal_entry_id: row.get("journal_entry_id"),
            date: row.get("date"),
            account_id: row.get("account_id"),
            account_name: row.get("account_name"),
            code: row.get("code"),
            description: row.get("description"),
            debit: row.get("debit"),
            credit: row.get("credit"),
            balance: *balance,
        });
    }

    Ok(result)
}
