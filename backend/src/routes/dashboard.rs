/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::db::journal_entry::get_all_by_transaction;
use crate::db::transaction::get_recent_transactions as get_recent_transactions_from_db;
use crate::db::vendor_invoice::get_top_payables as get_top_payables_from_db;
use crate::security::{RequirePrivilege, UseTransactions};
use crate::services::account_service::{get_account_with_balance, get_system_accounts};
use crate::services::report_service::{
    get_expense_breakdown as get_expense_breakdown_from_service, get_profit_loss,
};
use crate::DbKelpie;
use chrono::{Datelike, Duration, Local, NaiveDate};
use rocket::serde::json::Json;
use rocket::{get, routes, Route};
use rocket_db_pools::Connection;
use shared_core::dtos::dashboard::FinancialHealth;
use shared_core::dtos::expense_breakdown::ExpenseBreakdown;
use shared_core::dtos::recent_transaction::RecentTransaction;
use shared_core::dtos::top_payable::TopPayable;
use shared_core::models::{account_category::AccountCategory, system_tag::SystemTag};
use uuid::Uuid;

#[get("/financial-health")]
async fn get_financial_health(
    mut db: Connection<DbKelpie>,
    guard: RequirePrivilege<UseTransactions>,
) -> Result<Json<FinancialHealth>, rocket::http::Status> {
    let user = guard.0;
    let org_id = user.organization_id;

    let today = Local::now().date_naive();

    let year_start = NaiveDate::from_ymd_opt(today.year(), 1, 1).unwrap();

    let system_accounts = get_system_accounts(&mut db, org_id)
        .await
        .unwrap_or_default();

    let ar_account_id = system_accounts.get(&SystemTag::AccountsReceivable).cloned();
    let ap_account_id = system_accounts.get(&SystemTag::AccountsPayable).cloned();
    let bank_account_id = system_accounts.get(&SystemTag::CashAtBank).cloned();

    let accounts_receivable_balance = if let Some(id) = ar_account_id {
        get_account_with_balance(&mut db, id, user.organization_id)
            .await
            .map(|a| a.balance)
            .unwrap_or(-999)
    } else {
        0
    };
    let accounts_payable_balance = if let Some(id) = ap_account_id {
        get_account_with_balance(&mut db, id, user.organization_id)
            .await
            .map(|a| a.balance)
            .unwrap_or(-999)
    } else {
        0
    };

    let bank_balance = if let Some(id) = bank_account_id {
        get_account_with_balance(&mut db, id, user.organization_id)
            .await
            .map(|a| a.balance)
            .unwrap_or(-999)
    } else {
        0
    };

    let profit_loss_accounts = get_profit_loss(&mut db, org_id, year_start, today)
        .await
        .unwrap_or_default();

    let revenue_total = profit_loss_accounts
        .iter()
        .filter(|a| a.category == AccountCategory::Revenue)
        .map(|a| a.balance)
        .sum::<i64>();

    let expense_total = profit_loss_accounts
        .iter()
        .filter(|a| a.category == AccountCategory::Expense)
        .map(|a| a.balance)
        .sum::<i64>();

    let net_profit_ytd = (-revenue_total) - expense_total;

    Ok(Json(FinancialHealth {
        net_profit_ytd: net_profit_ytd,
        bank_balance: bank_balance,
        accounts_receivable: accounts_receivable_balance,
        accounts_payable: accounts_payable_balance,
    }))
}

#[get("/recent-transactions")]
async fn get_recent_transactions(
    mut db: Connection<DbKelpie>,
    guard: RequirePrivilege<UseTransactions>,
) -> Result<Json<Vec<RecentTransaction>>, rocket::http::Status> {
    let user = guard.0;
    let org_id = user.organization_id;

    let transactions = get_recent_transactions_from_db(&mut db, org_id, 5)
        .await
        .unwrap_or_default();
    let mut recent_transactions = Vec::new();

    for tx in transactions {
        let journal_entries = get_all_by_transaction(&mut db, tx.id)
            .await
            .unwrap_or_default();
        let amount = journal_entries.iter().map(|je| je.debit).sum::<i64>();
        let account_id = journal_entries.first().map(|je| je.account_id);

        recent_transactions.push(RecentTransaction {
            id: tx.id,
            account_id: account_id.unwrap_or(Uuid::nil()),
            date: tx.date,
            description: tx.description.unwrap_or_default(),
            amount: amount,
        });
    }

    Ok(Json(recent_transactions))
}

#[get("/expense-breakdown")]
async fn get_expense_breakdown(
    mut db: Connection<DbKelpie>,
    guard: RequirePrivilege<UseTransactions>,
) -> Result<Json<Vec<ExpenseBreakdown>>, rocket::http::Status> {
    let user = guard.0;
    let org_id = user.organization_id;
    let today = Local::now().date_naive();
    let month_start = NaiveDate::from_ymd_opt(today.year(), today.month(), 1).unwrap();

    let breakdown = get_expense_breakdown_from_service(&mut db, org_id, month_start, today)
        .await
        .unwrap_or_default()
        .iter()
        .map(|acc| ExpenseBreakdown {
            category: acc.name.clone(),
            amount: acc.balance,
        })
        .collect();

    Ok(Json(breakdown))
}

#[get("/top-payables")]
async fn get_top_payables(
    mut db: Connection<DbKelpie>,
    guard: RequirePrivilege<UseTransactions>,
) -> Result<Json<Vec<TopPayable>>, rocket::http::Status> {
    let user = guard.0;
    let org_id = user.organization_id;

    let date_before = Local::now().date_naive() + Duration::days(7);
    let payables = get_top_payables_from_db(&mut db, org_id, &date_before)
        .await
        .unwrap_or_default();

    Ok(Json(payables))
}

pub fn routes() -> Vec<Route> {
    routes![
        get_financial_health,
        get_recent_transactions,
        get_expense_breakdown,
        get_top_payables
    ]
}
