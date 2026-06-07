/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::ledger::db::account as account_db;
use crate::ledger::reports::account_ledger_export::{generate_ledger_csv, generate_ledger_typst};
use crate::ledger::services::account_service;
use crate::security::{ManageAccounts, RequirePrivilege, UseAccounts};
use crate::util::reports::{compile_typst_to_pdf, DownloadFile};
use crate::util::types::PathUuid;
use crate::util::ApiError;
use crate::DbKelpie;
use chrono::NaiveDate;
use rocket::http::ContentType;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, routes, Route};
use rocket_db_pools::Connection;
use shared_core::dtos::account_with_balance::AccountWithBalance;
use shared_core::dtos::journal_entry_with_balance::JournalEntryWithBalance;
use shared_core::models::account::Account;
use shared_core::models::account_category::AccountCategory;
use shared_core::requests::account::{CreateAccountRequest, UpdateAccountRequest};
use std::str::FromStr;

pub(crate) fn routes() -> Vec<Route> {
    routes![
        get_accounts,
        get_accounts_by_category,
        get_accounts_with_balances,
        get_payment_methods,
        get_account,
        get_account_entries,
        create_account,
        update_account,
        delete_account,
        export_account_ledger
    ]
}

#[get("/api/accounts")]
async fn get_accounts(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UseAccounts>,
) -> Result<Json<Vec<Account>>, ApiError> {
    let user = guard.0;
    let accounts = account_service::get_accounts(&mut pool, user.organization_id).await?;
    Ok(Json(accounts))
}

#[get("/api/accounts_by_category/<category>")]
async fn get_accounts_by_category(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UseAccounts>,
    category: &str,
) -> Result<Json<Vec<Account>>, ApiError> {
    let user = guard.0;
    if let Ok(category) = AccountCategory::from_str(category) {
        let accounts =
            account_service::get_accounts_by_category(&mut pool, user.organization_id, category)
                .await?;
        Ok(Json(accounts))
    } else {
        Err(ApiError::Internal(format!("Category {} not found", category)))
    }
}

#[get("/api/accounts_with_balances")]
async fn get_accounts_with_balances(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UseAccounts>,
) -> Result<Json<Vec<AccountWithBalance>>, ApiError> {
    let user = guard.0;
    let accounts =
        account_service::get_accounts_with_balances(&mut pool, user.organization_id).await?;
    Ok(Json(accounts))
}

#[get("/api/accounts/payment-methods")]
async fn get_payment_methods(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UseAccounts>,
) -> Result<Json<Vec<Account>>, ApiError> {
    let user = guard.0;
    let accounts =
        account_service::get_payment_methods(&mut pool, user.organization_id).await?;
    Ok(Json(accounts))
}

#[get("/api/accounts/<id>")]
async fn get_account(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
    guard: RequirePrivilege<UseAccounts>,
) -> Result<Json<Account>, ApiError> {
    let user = guard.0;

    let account = account_db::get(&mut pool, *id, user.organization_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Account not found".to_string()))?;
    Ok(Json(account))
}

#[get("/api/accounts/<id>/entries?<start>&<end>")]
async fn get_account_entries(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
    start: String,
    end: String,
    guard: RequirePrivilege<UseAccounts>,
) -> Result<Json<Vec<JournalEntryWithBalance>>, ApiError> {
    let user = guard.0;

    let start_date = NaiveDate::parse_from_str(&start, "%Y-%m-%d")
        .map_err(|_| ApiError::Invalid("Invalid start date".to_string()))?;
    let end_date = NaiveDate::parse_from_str(&end, "%Y-%m-%d")
        .map_err(|_| ApiError::Invalid("Invalid end date".to_string()))?;
    let entries = account_service::get_journal_entries_with_running_balance(
        &mut pool, *id, user.organization_id, start_date, end_date
    )
    .await?;
    Ok(Json(entries))
}

#[post("/api/accounts", data = "<req>")]
async fn create_account(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<ManageAccounts>,
    req: Json<CreateAccountRequest>,
) -> Result<Json<AccountWithBalance>, ApiError> {
    let user = guard.0;
    let new_account = account_db::insert(&mut pool, user.organization_id, &req).await?;
    Ok(Json(AccountWithBalance {
        id: new_account.id,
        organization_id: new_account.organization_id,
        parent_id: new_account.parent_id,
        code: new_account.code,
        name: new_account.name,
        category: new_account.category,
        is_group: new_account.is_group,
        is_bank_account: new_account.is_bank_account,
        system_tag: new_account.system_tag,
        created_at: new_account.created_at,
        balance: 0,
    }))
}

#[put("/api/accounts/<id>", data = "<req>")]
async fn update_account(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
    req: Json<UpdateAccountRequest>,
    guard: RequirePrivilege<ManageAccounts>,
) -> Result<Json<AccountWithBalance>, ApiError> {
    let user = guard.0;

    let updated_account = account_db::update(&mut pool, *id, user.organization_id, &req).await?;
    Ok(Json(AccountWithBalance {
        id: updated_account.id,
        organization_id: updated_account.organization_id,
        parent_id: updated_account.parent_id,
        code: updated_account.code,
        name: updated_account.name,
        category: updated_account.category,
        is_group: updated_account.is_group,
        is_bank_account: updated_account.is_bank_account,
        system_tag: updated_account.system_tag,
        created_at: updated_account.created_at,
        balance: 0,
    }))
}

#[delete("/api/accounts/<id>")]
async fn delete_account(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
    guard: RequirePrivilege<ManageAccounts>,
) -> Result<&'static str, ApiError> {
    let user = guard.0;

    if account_db::has_journal_entries(&mut pool, *id).await? {
        return Err(ApiError::Conflict(
            "Cannot delete an account with journal entries.".to_string(),
        ));
    }

    let rows_affected = account_db::delete(&mut pool, *id, user.organization_id).await?;
    if rows_affected == 0 {
        return Err(ApiError::NotFound("Account not found.".to_string()));
    }

    Ok("Account deleted successfully.")
}

#[get("/api/accounts/<id>/export/<format>?<start>&<end>")]
async fn export_account_ledger(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UseAccounts>,
    id: PathUuid,
    format: String,
    start: String,
    end: String,
) -> Result<DownloadFile, ApiError> {
    let user = guard.0;
    let start_date = NaiveDate::parse_from_str(&start, "%Y-%m-%d")
        .map_err(|_| ApiError::Invalid("Invalid start date".to_string()))?;
    let end_date = NaiveDate::parse_from_str(&end, "%Y-%m-%d")
        .map_err(|_| ApiError::Invalid("Invalid end date".to_string()))?;

    let account = account_db::get(&mut pool, *id, user.organization_id).await?;
    if let Some(account) = account {
        let entries = account_service::get_journal_entries_with_running_balance(
            &mut pool, *id, user.organization_id, start_date, end_date,
        )
        .await?;
        let org = crate::db::organization::get(&mut pool, user.organization_id).await?;

        let (content, content_type, filename) = match format.as_str() {
            "csv" => {
                let csv_data = generate_ledger_csv(&user, &entries);
                (
                    csv_data.into_bytes(),
                    ContentType::CSV,
                    "account_ledger.csv".to_string(),
                )
            }
            "pdf" => {
                let typst_data = generate_ledger_typst(
                    &user,
                    &entries,
                    account.name.as_str(),
                    &start_date,
                    &end_date,
                    &org,
                );
                match compile_typst_to_pdf(typst_data) {
                    Ok(pdf_bytes) => (pdf_bytes, ContentType::PDF, "trial_balance.pdf".to_string()),
                    Err(e) => return Err(ApiError::Internal(e)),
                }
            }
            _ => return Err(ApiError::Invalid("Invalid format".to_string())),
        };

        Ok(DownloadFile::new(content, filename, content_type))
    } else {
        Err(ApiError::NotFound("Account not found".to_string()))
    }
}
