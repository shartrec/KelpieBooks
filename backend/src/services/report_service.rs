/*
 * Copyright (c) 2026-2026. Trevor Campbell and others.
 *
 * This file is part of KelpieBooks.
 *
 * KelpieBooks is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License,or
 * (at your option) any later version.
 *
 * KelpieBooks is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with KelpieBooks; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 *
 * Contributors:
 *      Trevor Campbell
 *
 */

use crate::db;
use crate::util::ApiError;
use chrono::{Local, NaiveDate};
use rocket_db_pools::sqlx::PgConnection;
use shared_core::dtos::account_with_balance::AccountWithBalance;
use shared_core::dtos::aged_payable_summary::AgedPayableSummary;
use shared_core::dtos::general_ledger_line::GeneralLedgerLine;
use shared_core::models::account_category::AccountCategory;
use shared_core::models::invoice_status::InvoiceStatus;
use shared_core::reports::balance_sheet::BalanceSheet;
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

pub async fn get_profit_loss(
    pool: &mut PgConnection,
    organization_id: Uuid,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<AccountWithBalance>, ApiError> {
    let accounts = db::account::get_all_by_org(pool, organization_id).await?;

    // We need to fetch journal entries within the date range
    let entries = sqlx::query!(
        r#"
        SELECT je.account_id, je.debit, je.credit
        FROM journal_entries je
        JOIN transactions t ON je.transaction_id = t.id
        WHERE t.organization_id = $1 AND t.date >= $2 AND t.date <= $3
        "#,
        organization_id,
        start_date,
        end_date
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;

    let mut balances: HashMap<Uuid, i64> = HashMap::new();

    // 1. Calculate the direct balance for each account from its journal entries.
    for entry in &entries {
        *balances.entry(entry.account_id).or_insert(0) += entry.debit - entry.credit;
    }

    // 2. Build a map of parent to children and child counts for topological sort.
    let mut parent_map: HashMap<Uuid, Uuid> = HashMap::new();
    let mut child_count: HashMap<Uuid, usize> = HashMap::new();

    for account in &accounts {
        child_count.entry(account.id).or_insert(0);
        if let Some(parent_id) = account.parent_id {
            parent_map.insert(account.id, parent_id);
            *child_count.entry(parent_id).or_insert(0) += 1;
        }
    }

    // 3. Use Dependency-Driven Roll-up (topological sort from leaves to roots).
    let mut queue: VecDeque<Uuid> = child_count
        .iter()
        .filter(|(_, &count)| count == 0)
        .map(|(&id, _)| id)
        .collect();

    while let Some(account_id) = queue.pop_front() {
        if let Some(&parent_id) = parent_map.get(&account_id) {
            let balance = *balances.get(&account_id).unwrap_or(&0);
            *balances.entry(parent_id).or_insert(0) += balance;

            if let Some(count) = child_count.get_mut(&parent_id) {
                *count -= 1;
                if *count == 0 {
                    queue.push_back(parent_id);
                }
            }
        }
    }

    // 4. Map to the final DTO, filtering for Revenue and Expense.
    let result = accounts
        .into_iter()
        .filter(|acc| {
            acc.category == AccountCategory::Revenue || acc.category == AccountCategory::Expense
        })
        .map(|acc| AccountWithBalance {
            balance: *balances.get(&acc.id).unwrap_or(&0),
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

pub async fn get_expense_breakdown(
    pool: &mut PgConnection,
    organization_id: Uuid,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<AccountWithBalance>, ApiError> {
    let accounts = db::account::get_all_by_org(pool, organization_id).await?;

    let entries = sqlx::query!(
        r#"
        SELECT je.account_id, je.debit, je.credit
        FROM journal_entries je
        JOIN transactions t ON je.transaction_id = t.id
        WHERE t.organization_id = $1 AND t.date >= $2 AND t.date <= $3
        "#,
        organization_id,
        start_date,
        end_date
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;

    let mut balances: HashMap<Uuid, i64> = HashMap::new();

    for entry in &entries {
        *balances.entry(entry.account_id).or_insert(0) += entry.debit - entry.credit;
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
            let balance = *balances.get(&account_id).unwrap_or(&0);
            *balances.entry(parent_id).or_insert(0) += balance;

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
            balance: *balances.get(&acc.id).unwrap_or(&0),
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

pub async fn get_balance_sheet(
    pool: &mut PgConnection,
    organization_id: Uuid,
    date: NaiveDate,
) -> Result<BalanceSheet, ApiError> {
    let accounts = db::account::get_all_by_org(pool, organization_id).await?;

    let entries = sqlx::query!(
        r#"
        SELECT je.account_id, je.debit, je.credit
        FROM journal_entries je
        JOIN transactions t ON je.transaction_id = t.id
        WHERE t.organization_id = $1 AND t.date <= $2
        "#,
        organization_id,
        date
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;

    let mut balances: HashMap<Uuid, i64> = HashMap::new();

    for entry in &entries {
        let debit = i64::from(entry.debit);
        let credit = i64::from(entry.credit);
        *balances.entry(entry.account_id).or_insert(0) += debit - credit;
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
            let balance = *balances.get(&account_id).unwrap_or(&0);
            *balances.entry(parent_id).or_insert(0) += balance;

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
    let mut total_assets = 0;
    let mut total_liabilities = 0;
    let mut total_equity = 0;
    let mut net_income = 0;

    for acc in &accounts {
        let balance = *balances.get(&acc.id).unwrap_or(&0);

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
        let balance = *balances.get(&acc.id).unwrap_or(&0);
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

pub async fn get_trial_balance(
    pool: &mut PgConnection,
    organization_id: Uuid,
    date: NaiveDate,
) -> Result<Vec<AccountWithBalance>, ApiError> {
    let accounts = db::account::get_all_by_org(pool, organization_id).await?;

    let entries = sqlx::query!(
        r#"
        SELECT je.account_id, je.debit, je.credit
        FROM journal_entries je
        JOIN transactions t ON je.transaction_id = t.id
        WHERE t.organization_id = $1 AND t.date <= $2
        "#,
        organization_id,
        date
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;

    let mut balances: HashMap<Uuid, i64> = HashMap::new();

    for entry in &entries {
        *balances.entry(entry.account_id).or_insert(0) += entry.debit - entry.credit;
    }

    let result = accounts
        .into_iter()
        .map(|acc| AccountWithBalance {
            balance: *balances.get(&acc.id).unwrap_or(&0),
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

pub async fn get_general_ledger(
    pool: &mut PgConnection,
    organization_id: Uuid,
    start_date: NaiveDate,
    end_date: NaiveDate,
    account_ids: Option<Vec<Uuid>>,
    min_amount: Option<i64>,
) -> Result<Vec<GeneralLedgerLine>, ApiError> {
    let account_ids = account_ids.unwrap_or_default();
    let min_amount = min_amount.unwrap_or(0);

    let rows = sqlx::query!(
        r#"
        SELECT
            t.id as transaction_id,
            je.id as journal_entry_id,
            t.date,
            a.id as account_id,
            a.name as account_name,
            je.description,
            je.debit,
            je.credit
        FROM journal_entries je
        JOIN transactions t ON je.transaction_id = t.id
        JOIN accounts a ON je.account_id = a.id
        WHERE t.organization_id = $1
          AND t.date >= $2
          AND t.date <= $3
          AND a.category IN ('Revenue', 'Expense')
          AND (CARDINALITY($4::uuid[]) = 0 OR a.id = ANY($4))
          AND (je.debit >= $5 OR je.credit >= $5)
        ORDER BY a.code ASC, t.date ASC
        "#,
        organization_id,
        start_date,
        end_date,
        &account_ids,
        min_amount
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;

    let mut result = Vec::new();
    let mut balances: HashMap<Uuid, i64> = HashMap::new();

    for row in rows {
        let balance = balances.entry(row.account_id).or_insert(0);
        *balance += row.debit - row.credit;

        result.push(GeneralLedgerLine {
            transaction_id: row.transaction_id,
            journal_entry_id: row.journal_entry_id,
            date: row.date,
            account_id: row.account_id,
            account_name: row.account_name,
            description: row.description,
            debit: row.debit,
            credit: row.credit,
            balance: *balance,
        });
    }

    Ok(result)
}

pub async fn get_aged_payables(
    pool: &mut PgConnection,
    organization_id: Uuid,
    date: NaiveDate,
) -> Result<Vec<AgedPayableSummary>, ApiError> {
    let invoices = db::vendor_invoice::get_by_org(
        pool,
        organization_id,
        None,
        None,
        None,
        None,
        Some(format!("{},{}", InvoiceStatus::Open.as_str(), InvoiceStatus::PartiallyPaid.as_str())),
    )
    .await?;

    let mut summary_map: HashMap<Uuid, AgedPayableSummary> = HashMap::new();

    for invoice in invoices {
        let summary = summary_map
            .entry(invoice.partner_id)
            .or_insert_with(|| AgedPayableSummary {
                partner_id: invoice.partner_id,
                partner_name: invoice.partner_name.clone(),
                current: 0,
                days_30: 0,
                days_60: 0,
                days_90: 0,
                days_90_plus: 0,
                total: 0,
                invoices: Vec::new(),
            });

        let days_overdue = (date - invoice.due_date).num_days();

        if days_overdue <= 0 {
            summary.current += invoice.amount_remaining;
        } else if days_overdue <= 30 {
            summary.days_30 += invoice.amount_remaining;
        } else if days_overdue <= 60 {
            summary.days_60 += invoice.amount_remaining;
        } else if days_overdue <= 90 {
            summary.days_90 += invoice.amount_remaining;
        } else {
            summary.days_90_plus += invoice.amount_remaining;
        }
        summary.total += invoice.amount_remaining;
        summary.invoices.push(invoice);
    }

    Ok(summary_map.into_values().collect())
}
