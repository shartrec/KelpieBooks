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
use std::collections::HashMap;
use uuid::Uuid;

pub async fn get_accounts_with_balances(
    pool: &mut PgConnection,
    organization_id: Uuid,
) -> Result<Vec<AccountWithBalance>, ApiError> {
    let mut accounts = db::account::get_all_by_org(pool, organization_id).await?;
    let entries = db::journal_entry::get_all_by_org(pool, organization_id).await?;

    let mut balances: HashMap<Uuid, i64> = HashMap::new();

    // 1. Calculate the direct balance for each account from its journal entries.
    for entry in &entries {
        *balances.entry(entry.account_id).or_insert(0) += entry.debit - entry.credit;
    }

    // 2. Sort accounts by code in descending order. This ensures children are processed before parents.
    accounts.sort_by(|a, b| b.code.cmp(&a.code));

    // 3. Iterate and roll up balances from child to parent.
    for account in &accounts {
        if let Some(parent_id) = account.parent_id {
            let child_balance = *balances.get(&account.id).unwrap_or(&0);
            *balances.entry(parent_id).or_insert(0) += child_balance;
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
) -> Result<Vec<JournalEntryWithBalance>, ApiError> {
    let entries = db::journal_entry::get_all_by_account_with_date(pool, account_id).await?;
    let mut running_balance = 0;

    let result = entries
        .into_iter()
        .map(|entry| {
            running_balance += entry.debit - entry.credit;
            JournalEntryWithBalance {
                id: entry.id,
                transaction_id: entry.transaction_id,
                account_id: entry.account_id,
                date: entry.date,
                description: entry.description,
                debit: entry.debit,
                credit: entry.credit,
                running_balance,
            }
        })
        .collect();

    Ok(result)
}
