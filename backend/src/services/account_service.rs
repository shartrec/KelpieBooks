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
use rocket_db_pools::sqlx::PgConnection;
use shared_core::dtos::account_with_balance::AccountWithBalance;
use shared_core::dtos::journal_entry_with_balance::JournalEntryWithBalance;
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;
use chrono::{Local, NaiveDate};
use sqlx::Acquire;
use shared_core::models::{Account, SystemTag};
use shared_core::requests::configuration::UpdateConfigurationRequest;

pub async fn get_accounts(
    pool: &mut PgConnection,
    organization_id: Uuid,
) -> Result<Vec<Account>, ApiError> {
    let accounts = db::account::get_all_by_org(pool, organization_id).await?;

    Ok(accounts)
}

pub async fn get_account_with_balance(
    pool: &mut PgConnection,
    account_id: Uuid,
) -> Result<AccountWithBalance, ApiError> {
    let account = db::account::get(pool, account_id).await?
        .ok_or_else(|| ApiError::NotFound("Account not found".to_string()))?;

    let balance = db::journal_entry::get_balance_up_to_date(pool, account_id, Local::now().date_naive()).await?;

    Ok(AccountWithBalance {
        balance,
        id: account.id,
        organization_id: account.organization_id,
        parent_id: account.parent_id,
        code: account.code,
        name: account.name,
        category: account.category,
        is_group: account.is_group,
        system_tag: account.system_tag,
        created_at: account.created_at,
    })
}

pub async fn get_accounts_with_balances(
    pool: &mut PgConnection,
    organization_id: Uuid,
) -> Result<Vec<AccountWithBalance>, ApiError> {
    let accounts = db::account::get_all_by_org(pool, organization_id).await?;
    let entries = db::journal_entry::get_all_by_org(pool, organization_id).await?;

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

    // 4. Map to the final DTO.
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
            system_tag: acc.system_tag,
            created_at: acc.created_at,
        })
        .collect();

    Ok(result)
}

pub async fn get_journal_entries_with_running_balance(
    pool: &mut PgConnection,
    account_id: Uuid,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<JournalEntryWithBalance>, ApiError> {
    let opening_balance = db::journal_entry::get_balance_before_date(pool, account_id, start_date).await?;
    let entries = db::journal_entry::get_all_by_account_in_date_range(pool, account_id, start_date, end_date).await?;

    let mut running_balance = opening_balance;
    let mut result = Vec::new();

    // Add an opening balance entry
    result.push(JournalEntryWithBalance {
        id: Uuid::new_v4(), // Bogus ID
        transaction_id: Uuid::new_v4(),
        account_id,
        date: start_date,
        description: Some("Opening Balance".to_string()),
        debit: if opening_balance > 0 { opening_balance } else { 0 },
        credit: if opening_balance < 0 { -opening_balance } else { 0 },
        running_balance: opening_balance,
    });

    for entry in entries {
        running_balance += entry.debit - entry.credit;
        result.push(JournalEntryWithBalance {
            id: entry.id,
            transaction_id: entry.transaction_id,
            account_id: entry.account_id,
            date: entry.date,
            description: entry.description,
            debit: entry.debit,
            credit: entry.credit,
            running_balance,
        });
    }

    Ok(result)
}

pub async fn get_system_accounts(
        pool: &mut PgConnection,
        organization_id: Uuid,
    ) -> Result<HashMap<SystemTag, Uuid>, ApiError> {
    Ok(db::account::get_system_accounts(pool, organization_id).await?)
}

pub async fn update_system_accounts(
        pool: &mut PgConnection,
        organization_id: Uuid,
        system_accounts: HashMap<SystemTag, Uuid>,
    ) -> Result<HashMap<SystemTag, Uuid>, ApiError> {

    let mut tx = pool.begin().await?;
    db::account::update_system_accounts(&mut tx, organization_id, system_accounts).await?;
    let resp = db::account::get_system_accounts(&mut tx, organization_id).await?;
    tx.commit().await?;

    Ok(resp)
}

pub async fn update_configuration(
    pool: &mut PgConnection,
    organization_id: Uuid,
    req: UpdateConfigurationRequest,
) -> Result<(), ApiError> {
    let mut tx = pool.begin().await?;

    db::account::update_system_accounts(&mut tx, organization_id, req.system_accounts).await?;
    db::organization::set_audit_mode(&mut tx, organization_id, req.strict_audit_mode).await?;

    tx.commit().await?;

    Ok(())
}
