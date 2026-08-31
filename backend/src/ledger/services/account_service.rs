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

use chrono::{
    Local,
    NaiveDate,
};
use rocket_db_pools::sqlx::PgConnection;
use rust_decimal::{
    dec,
    Decimal,
};
use shared_core::{
    ledger::{
        dtos::{
            account_with_balance::AccountWithBalance,
            journal_entry_with_balance::JournalEntryWithBalance,
        },
        models::{
            account::Account,
            account_category::AccountCategory,
            system_tag::SystemTag,
        },
        requests::{
            configuration::UpdateConfigurationRequest,
            transaction::CreateTransactionRequest,
        },
    },
    AccountId,
    JournalEntryId,
    OrgId,
    TransactionId,
};
use sqlx::Acquire;
use uuid::Uuid;

use crate::{
    core::db,
    ledger::db::{
        account,
        account::{
            get,
            get_all_by_category,
            get_all_by_org,
        },
        journal_entry,
        transaction,
    },
    util::ApiError,
};

pub(crate) async fn get_accounts(
    pool: &mut PgConnection,
    organization_id: OrgId,
) -> Result<Vec<Account>, ApiError> {
    let accounts = get_all_by_org(pool, organization_id).await?;

    Ok(accounts)
}
pub(crate) async fn get_accounts_by_category(
    pool: &mut PgConnection,
    organization_id: OrgId,
    category: AccountCategory,
) -> Result<Vec<Account>, ApiError> {
    let accounts = get_all_by_category(pool, organization_id, &[category]).await?;
    Ok(accounts)
}

pub(crate) async fn get_account_with_balance(
    pool: &mut PgConnection,
    org_id: OrgId,
    account_id: AccountId,
) -> Result<AccountWithBalance, ApiError> {
    let account = get(pool, org_id, account_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Account not found".to_string()))?;

    let balance =
        journal_entry::get_balance_up_to_date(pool, org_id, account_id, Local::now().date_naive())
            .await?;

    Ok(AccountWithBalance {
        balance,
        id: account.id,
        organization_id: account.organization_id,
        parent_id: account.parent_id,
        code: account.code,
        name: account.name,
        category: account.category,
        is_group: account.is_group,
        is_bank_account: account.is_bank_account,
        system_tag: account.system_tag,
        created_at: account.created_at,
    })
}

pub(crate) async fn get_accounts_with_balances(
    pool: &mut PgConnection,
    organization_id: OrgId,
) -> Result<Vec<AccountWithBalance>, ApiError> {
    let accounts = get_all_by_org(pool, organization_id).await?;
    let entries = journal_entry::get_all_by_org(pool, organization_id).await?;

    let mut balances: HashMap<AccountId, Decimal> = HashMap::new();

    // 1. Calculate the direct balance for each account from its journal entries.
    for entry in &entries {
        *balances.entry(entry.account_id).or_insert(dec!(0.00)) += entry.debit - entry.credit;
    }

    // 2. Build a map of parent to children and child counts for topological sort.
    let mut parent_map: HashMap<AccountId, AccountId> = HashMap::new();
    let mut child_count: HashMap<AccountId, usize> = HashMap::new();

    for account in &accounts {
        child_count.entry(account.id).or_insert(0);
        if let Some(parent_id) = account.parent_id {
            parent_map.insert(account.id, parent_id);
            *child_count.entry(parent_id).or_insert(0) += 1;
        }
    }

    // 3. Use Dependency-Driven Roll-up (topological sort from leaves to roots).
    let mut queue: VecDeque<AccountId> = child_count
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

    // 4. Map to the final DTO.
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

pub(crate) async fn get_payment_methods(
    pool: &mut PgConnection,
    organization_id: OrgId,
) -> Result<Vec<Account>, ApiError> {
    let accounts = get_all_by_org(pool, organization_id).await?;
    let payment_methods = accounts
        .into_iter()
        .filter(|acc| acc.is_bank_account)
        .collect();
    Ok(payment_methods)
}

pub(crate) async fn get_journal_entries_with_running_balance(
    pool: &mut PgConnection,
    org_id: OrgId,
    account_id: AccountId,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<JournalEntryWithBalance>, ApiError> {
    let opening_balance =
        journal_entry::get_balance_before_date(pool, org_id, account_id, start_date).await?;
    let entries = journal_entry::get_all_by_account_in_date_range(
        pool, org_id, account_id, start_date, end_date,
    )
    .await?;

    let mut running_balance = opening_balance;
    let mut result = Vec::new();

    // Add an opening balance entry
    result.push(JournalEntryWithBalance {
        id: JournalEntryId(Uuid::new_v4()), // Bogus ID
        transaction_id: TransactionId::default(),
        account_id,
        date: start_date,
        description: Some("Opening Balance".to_string()),
        debit: if opening_balance > dec!(0.00) {
            opening_balance
        } else {
            dec!(0.00)
        },
        credit: if opening_balance < dec!(0.00) {
            -opening_balance
        } else {
            dec!(0.00)
        },
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

pub(crate) async fn get_system_accounts(
    pool: &mut PgConnection,
    organization_id: OrgId,
) -> Result<HashMap<SystemTag, AccountId>, ApiError> {
    Ok(account::get_system_accounts(pool, organization_id).await?)
}

pub(crate) async fn update_system_accounts(
    pool: &mut PgConnection,
    organization_id: OrgId,
    system_accounts: &HashMap<SystemTag, AccountId>,
) -> Result<HashMap<SystemTag, AccountId>, ApiError> {
    let mut tx = pool.begin().await?;
    account::update_system_accounts(&mut tx, organization_id, system_accounts).await?;
    let resp = get_system_accounts(&mut tx, organization_id).await?;
    tx.commit().await?;

    Ok(resp)
}

pub(crate) async fn update_configuration(
    pool: &mut PgConnection,
    organization_id: OrgId,
    req: &UpdateConfigurationRequest,
) -> Result<(), ApiError> {
    let mut tx = pool.begin().await?;

    update_system_accounts(&mut tx, organization_id, &req.system_accounts).await?;
    db::organization::set_audit_mode(&mut tx, organization_id, req.strict_audit_mode).await?;

    tx.commit().await?;

    Ok(())
}

pub(crate) async fn create_transaction(
    pool: &mut PgConnection,
    organization_id: OrgId,
    req: &CreateTransactionRequest,
) -> Result<TransactionId, ApiError> {
    let total_debits: Decimal = req.entries.iter().map(|e| e.debit).sum();
    let total_credits: Decimal = req.entries.iter().map(|e| e.credit).sum();

    if total_debits == dec!(0.00) || total_credits == dec!(0.00) || total_debits != total_credits {
        return Err(ApiError::BadRequest(
            "Transaction must be balanced and not zero.".to_string(),
        ));
    }

    // Check the organization locked date
    let organization = db::organization::get(pool, organization_id).await?;

    if let Some(date) = organization.unwrap().locked_until {
        if req.date <= date {
            return Err(ApiError::Forbidden(
                "Period is locked for editing".to_string(),
            ));
        }
    }

    let main_description = req.entries.get(0).and_then(|e| e.description.clone());

    let mut tx = pool.begin().await?;

    let transaction_id = transaction::insert(
        &mut tx,
        organization_id,
        req.date,
        &main_description,
        &req.reference,
    )
    .await?;

    for entry in &req.entries {
        journal_entry::insert(
            &mut tx,
            transaction_id,
            entry.account_id,
            entry.debit,
            entry.credit,
            entry.description.as_deref(),
        )
        .await?;
    }

    tx.commit().await?;

    Ok(transaction_id)
}
