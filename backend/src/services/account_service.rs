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

use rocket_db_pools::sqlx::PgConnection;
use shared_core::dtos::account_with_balance::AccountWithBalance;
use shared_core::models::{Account, JournalEntry};
use std::collections::HashMap;
use uuid::Uuid;
use crate::db;
use crate::util::ApiError;

pub async fn get_accounts_with_balances(
    pool: &mut PgConnection,
    organization_id: Uuid,
) -> Result<Vec<AccountWithBalance>, ApiError> {
    let accounts = db::account::get_all_by_org(pool, organization_id).await?;
    let entries = db::journal_entry::get_all_by_org(pool, organization_id).await?;

    let mut balance_map: HashMap<Uuid, i64> = HashMap::new();

    // 1. Calculate the direct balance for each account
    for entry in entries {
        *balance_map.entry(entry.account_id).or_insert(0) += entry.debit - entry.credit;
    }

    // 2. Create a map for quick account lookup
    let account_map: HashMap<Uuid, Account> = accounts
        .into_iter()
        .map(|acc| (acc.id, acc))
        .collect();

    // 3. Perform the roll-up calculation
    let mut rolled_up_balances = balance_map.clone();
    for (account_id, account) in &account_map {
        let mut current_parent_id = account.parent_id;
        while let Some(parent_id) = current_parent_id {
            if let Some(parent_balance) = rolled_up_balances.get_mut(&parent_id) {
                *parent_balance += balance_map.get(account_id).unwrap_or(&0);
            }
            // Move up the hierarchy
            current_parent_id = account_map.get(&parent_id).and_then(|p| p.parent_id);
        }
    }

    // 4. Combine everything into the final DTO
    let result = account_map
        .into_values()
        .map(|acc| AccountWithBalance {
            balance: *rolled_up_balances.get(&acc.id).unwrap_or(&0),
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
