/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::ledger::db::account;
use crate::ledger::reports::balance_sheet_export::{generate_balance_sheet_csv, generate_balance_sheet_typst};
use crate::ledger::reports::general_ledger_export::{generate_general_ledger_csv, generate_general_ledger_typst};
use crate::ledger::reports::profit_loss_export::{generate_profit_loss_csv, generate_profit_loss_typst};
use crate::ledger::reports::trial_balance_export::{generate_trial_balance_csv, generate_trial_balance_typst};
use crate::ledger::services::report_service;
use crate::security::{RequirePrivilege, UseTransactions};
use crate::util::locale_context::LocaleContext;
use crate::util::reports::{compile_typst_to_pdf, DownloadFile};
use crate::util::ApiError;
use crate::DbKelpie;
use chrono::NaiveDate;
use rocket::http::ContentType;
use rocket::serde::json::Json;
use rocket::{get, routes, Route};
use rocket_db_pools::Connection;
use shared_core::ledger::dtos::account_with_balance::AccountWithBalance;
use shared_core::ledger::dtos::general_ledger_line::GeneralLedgerLine;
use shared_core::ledger::dtos::balance_sheet::BalanceSheet;
use uuid::Uuid;

pub(crate) fn routes() -> Vec<Route> {
    routes![
        get_profit_loss,
        get_balance_sheet,
        get_trial_balance,
        get_general_ledger,
        export_trial_balance,
        export_profit_loss,
        export_balance_sheet,
        export_general_ledger
    ]
}

#[get("/api/reports/profit-loss?<start>&<end>")]
async fn get_profit_loss(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UseTransactions>,
    start: String,
    end: String,
) -> Result<Json<Vec<AccountWithBalance>>, ApiError> {
    let user = guard.0;
    let start_date = NaiveDate::parse_from_str(&start, "%Y-%m-%d")
        .map_err(|_| ApiError::Invalid("Invalid start date".to_string()))?;
    let end_date = NaiveDate::parse_from_str(&end, "%Y-%m-%d")
        .map_err(|_| ApiError::Invalid("Invalid end date".to_string()))?;

    let report =
        report_service::get_profit_loss(&mut pool, user.organization_id, start_date, end_date)
            .await?;
    Ok(Json(report))
}

#[get("/api/reports/balance-sheet?<date>")]
async fn get_balance_sheet(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UseTransactions>,
    date: String,
) -> Result<Json<BalanceSheet>, ApiError> {
    let user = guard.0;
    let report_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|_| ApiError::Invalid("Invalid date".to_string()))?;

    let report =
        report_service::get_balance_sheet(&mut pool, user.organization_id, report_date).await?;
    Ok(Json(report))
}

#[get("/api/reports/trial-balance?<date>")]
async fn get_trial_balance(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UseTransactions>,
    date: String,
) -> Result<Json<Vec<AccountWithBalance>>, ApiError> {
    let user = guard.0;
    let report_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|_| ApiError::Invalid("Invalid date".to_string()))?;

    let report =
        report_service::get_trial_balance(&mut pool, user.organization_id, report_date).await?;
    Ok(Json(report))
}

#[get("/api/reports/general-ledger?<start>&<end>&<accounts>&<min_amount>")]
async fn get_general_ledger(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UseTransactions>,
    start: String,
    end: String,
    accounts: Option<String>,
    min_amount: Option<i64>,
) -> Result<Json<Vec<GeneralLedgerLine>>, ApiError> {
    let user = guard.0;
    let start_date = NaiveDate::parse_from_str(&start, "%Y-%m-%d")
        .map_err(|_| ApiError::Invalid("Invalid start date".to_string()))?;
    let end_date = NaiveDate::parse_from_str(&end, "%Y-%m-%d")
        .map_err(|_| ApiError::Invalid("Invalid end date".to_string()))?;

    let account_ids: Option<Vec<Uuid>> = accounts.map(|s| {
        s.split(',')
            .filter_map(|id| id.parse::<Uuid>().ok())
            .collect()
    });

    let report = report_service::get_general_ledger(
        &mut pool,
        user.organization_id,
        start_date,
        end_date,
        account_ids,
        min_amount,
    )
    .await?;
    Ok(Json(report))
}

#[get("/api/reports/trial-balance/export/<format>?<date>")]
async fn export_trial_balance(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UseTransactions>,
    format: String,
    date: String,
) -> Result<DownloadFile, ApiError> {
    let user = guard.0;
    let report_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|_| ApiError::Invalid("Invalid date".to_string()))?;

    let accounts =
        report_service::get_trial_balance(&mut pool, user.organization_id, report_date).await?;
    let org = crate::db::organization::get(&mut pool, user.organization_id).await?;

    let (content, content_type, filename) = match format.as_str() {
        "csv" => {
            let csv_data = generate_trial_balance_csv(&user, &accounts);
            (
                csv_data.into_bytes(),
                ContentType::CSV,
                "trial_balance.csv".to_string(),
            )
        }
        "pdf" => {
            let typst_data = generate_trial_balance_typst(&user, &accounts, &report_date, &org);

            match compile_typst_to_pdf(typst_data) {
                Ok(pdf_bytes) => (pdf_bytes, ContentType::PDF, "trial_balance.pdf".to_string()),
                Err(e) => return Err(ApiError::Internal(e)),
            }
        }
        _ => return Err(ApiError::Invalid("Invalid format".to_string())),
    };

    Ok(DownloadFile::new(content, filename, content_type))
}

#[get("/api/reports/profit-loss/export/<format>?<start>&<end>")]
async fn export_profit_loss(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UseTransactions>,
    format: String,
    start: String,
    end: String,
) -> Result<DownloadFile, ApiError> {
    let user = guard.0;
    let start_date = NaiveDate::parse_from_str(&start, "%Y-%m-%d")
        .map_err(|_| ApiError::Invalid("Invalid start date".to_string()))?;
    let end_date = NaiveDate::parse_from_str(&end, "%Y-%m-%d")
        .map_err(|_| ApiError::Invalid("Invalid end date".to_string()))?;

    let accounts =
        report_service::get_profit_loss(&mut pool, user.organization_id, start_date, end_date)
            .await?;
    let org = crate::db::organization::get(&mut pool, user.organization_id).await?;

    let (content, content_type, filename) = match format.as_str() {
        "csv" => {
            let csv_data = generate_profit_loss_csv(&user, &accounts);
            (
                csv_data.into_bytes(),
                ContentType::CSV,
                "profit_loss.csv".to_string(),
            )
        }
        "pdf" => {
            let typst_data = generate_profit_loss_typst(&user, &accounts, &start_date, &end_date, &org);
            match compile_typst_to_pdf(typst_data) {
                Ok(pdf_bytes) => (pdf_bytes, ContentType::PDF, "trial_balance.pdf".to_string()),
                Err(e) => return Err(ApiError::Internal(e)),
            }
        }
        _ => return Err(ApiError::Invalid("Invalid format".to_string())),
    };

    Ok(DownloadFile::new(content, filename, content_type))
}

#[get("/api/reports/balance-sheet/export/<format>?<date>")]
async fn export_balance_sheet(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UseTransactions>,
    format: String,
    date: String,
) -> Result<DownloadFile, ApiError> {
    let user = guard.0;
    let report_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|_| ApiError::Invalid("Invalid date".to_string()))?;

    let balance_sheet =
        report_service::get_balance_sheet(&mut pool, user.organization_id, report_date).await?;
    let org = crate::db::organization::get(&mut pool, user.organization_id).await?;

    let (content, content_type, filename) = match format.as_str() {
        "csv" => {
            let csv_data = generate_balance_sheet_csv(&user, &balance_sheet);
            (
                csv_data.into_bytes(),
                ContentType::CSV,
                "balance_sheet.csv".to_string(),
            )
        }
        "pdf" => {
            let typst_data = generate_balance_sheet_typst(&user, &balance_sheet, &report_date, &org);
            match compile_typst_to_pdf(typst_data) {
                Ok(pdf_bytes) => (pdf_bytes, ContentType::PDF, "trial_balance.pdf".to_string()),
                Err(e) => return Err(ApiError::Internal(e)),
            }
        }
        _ => return Err(ApiError::Invalid("Invalid format".to_string())),
    };

    Ok(DownloadFile::new(content, filename, content_type))
}

#[get("/api/reports/general-ledger/export/<format>?<start>&<end>&<accounts>&<min_amount>")]
async fn export_general_ledger(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UseTransactions>,
    format: String,
    start: String,
    end: String,
    accounts: Option<String>,
    min_amount: Option<i64>,
) -> Result<DownloadFile, ApiError> {

    let user = guard.0;
    let i18n = LocaleContext::new(&user.locale);

    let start_date = NaiveDate::parse_from_str(&start, "%Y-%m-%d")
        .map_err(|_| ApiError::Invalid("Invalid start date".to_string()))?;
    let end_date = NaiveDate::parse_from_str(&end, "%Y-%m-%d")
        .map_err(|_| ApiError::Invalid("Invalid end date".to_string()))?;

    let account_ids: Option<Vec<Uuid>> = accounts.map(|s| {
        s.split(',')
            .filter_map(|id| id.parse::<Uuid>().ok())
            .collect()
    });

    //Validate the accounts are valid and in user organization
    if let Some(ref ids) = account_ids {
        for id in ids {
            let account = account::get(&mut pool, *id, user.organization_id).await?;
            if let Some(acc) = account {
                if acc.organization_id != user.organization_id {
                    return Err(ApiError::NotFound(i18n.t("coa-error-not-found")));
                }
            }
        }
    }

    let lines = report_service::get_general_ledger(
        &mut pool,
        user.organization_id,
        start_date,
        end_date,
        account_ids,
        min_amount,
    )
    .await?;
    let org = crate::db::organization::get(&mut pool, user.organization_id).await?;

    let (content, content_type, filename) = match format.as_str() {
        "csv" => {
            let csv_data = generate_general_ledger_csv(&user, &lines);
            (
                csv_data.into_bytes(),
                ContentType::CSV,
                "general_ledger.csv".to_string(),
            )
        }
        "pdf" => {
            let typst_data = generate_general_ledger_typst(&user, &lines, &start_date, &end_date, &org);
            match compile_typst_to_pdf(typst_data) {
                Ok(pdf_bytes) => (
                    pdf_bytes,
                    ContentType::PDF,
                    "general_ledger.pdf".to_string(),
                ),
                Err(e) => return Err(ApiError::Internal(e)),
            }
        }
        _ => return Err(ApiError::Invalid("Invalid format".to_string())),
    };

    Ok(DownloadFile::new(content, filename, content_type))
}
