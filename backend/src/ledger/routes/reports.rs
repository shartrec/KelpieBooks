/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::NaiveDate;
use fluent::fluent_args;
use rocket::{
    get,
    http::ContentType,
    routes,
    serde::json::Json,
    Route,
    State,
};
use rocket_db_pools::Connection;
use rust_decimal::Decimal;
use shared_core::{
    ledger::dtos::{
        account_with_balance::AccountWithBalance,
        balance_sheet::BalanceSheet,
        general_ledger_line::GeneralLedgerLine,
    },
    AccountId,
};
use uuid::Uuid;

use crate::{
    ledger::{
        db::account,
        reports::{
            balance_sheet_export::{
                generate_balance_sheet_csv,
                generate_balance_sheet_typst,
            },
            general_ledger_export::{
                generate_general_ledger_csv,
                generate_general_ledger_typst,
            },
            profit_loss_export::{
                generate_profit_loss_csv,
                generate_profit_loss_typst,
            },
            trial_balance_export::{
                generate_trial_balance_csv,
                generate_trial_balance_typst,
            },
        },
        services::report_service,
    },
    security::{
        RequirePrivilege,
        UseTransactions,
    },
    util::{
        locale_context::LocaleContext,
        reports::{
            compile_typst_to_pdf,
            DownloadFile,
        },
        ApiError,
    },
    DbKelpie,
    TemplateConfig,
};

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
    start: &str,
    end: &str,
) -> Result<Json<Vec<AccountWithBalance>>, ApiError> {
    let user = guard.0;
    let start_date = NaiveDate::parse_from_str(&start, "%Y-%m-%d")
        .map_err(|_| ApiError::BadRequest("Invalid start date".to_string()))?;
    let end_date = NaiveDate::parse_from_str(&end, "%Y-%m-%d")
        .map_err(|_| ApiError::BadRequest("Invalid end date".to_string()))?;

    let report =
        report_service::get_profit_loss(&mut pool, user.organization_id, start_date, end_date)
            .await?;
    Ok(Json(report))
}

#[get("/api/reports/balance-sheet?<date>")]
async fn get_balance_sheet(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UseTransactions>,
    date: &str,
) -> Result<Json<BalanceSheet>, ApiError> {
    let user = guard.0;
    let report_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|_| ApiError::BadRequest("Invalid date".to_string()))?;

    let report =
        report_service::get_balance_sheet(&mut pool, user.organization_id, report_date).await?;
    Ok(Json(report))
}

#[get("/api/reports/trial-balance?<date>")]
async fn get_trial_balance(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UseTransactions>,
    date: &str,
) -> Result<Json<Vec<AccountWithBalance>>, ApiError> {
    let user = guard.0;
    let report_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|_| ApiError::BadRequest("Invalid date".to_string()))?;

    let report =
        report_service::get_trial_balance(&mut pool, user.organization_id, report_date).await?;
    Ok(Json(report))
}

#[get("/api/reports/general-ledger?<start>&<end>&<accounts>&<min_amount>")]
async fn get_general_ledger(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UseTransactions>,
    start: &str,
    end: &str,
    accounts: Option<&str>,
    min_amount: Option<Decimal>,
) -> Result<Json<Vec<GeneralLedgerLine>>, ApiError> {
    let user = guard.0;
    let start_date = NaiveDate::parse_from_str(&start, "%Y-%m-%d")
        .map_err(|_| ApiError::BadRequest("Invalid start date".to_string()))?;
    let end_date = NaiveDate::parse_from_str(&end, "%Y-%m-%d")
        .map_err(|_| ApiError::BadRequest("Invalid end date".to_string()))?;

    let account_ids: Option<Vec<AccountId>> = accounts.map(|s| {
        s.split(',')
            .filter_map(|id| id.parse::<Uuid>().map(AccountId).ok())
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
    config: &State<TemplateConfig>,
    format: &str,
    date: &str,
) -> Result<DownloadFile, ApiError> {
    let user = guard.0;
    let i18n = LocaleContext::new(&user.locale);
    let template_dir = config.root_directory.to_string_lossy();

    let report_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|_| ApiError::BadRequest("Invalid date".to_string()))?;

    let accounts =
        report_service::get_trial_balance(&mut pool, user.organization_id, report_date).await?;
    let org = crate::core::db::organization::get(&mut pool, user.organization_id).await?;

    let (content, content_type, filename) = match format {
        "csv" => {
            let csv_data = generate_trial_balance_csv(&user, &accounts);
            (
                csv_data.into_bytes(),
                ContentType::CSV,
                "trial_balance.csv".to_string(),
            )
        }
        "pdf" => {
            let typst_data = generate_trial_balance_typst(&user, &accounts);

            let report_date_str = i18n.format_date(report_date);
            let report_qual = i18n.t_args(
                "balance-sheet-export-as-at",
                &fluent_args!["date" => report_date_str],
            );
            match compile_typst_to_pdf(
                typst_data,
                &i18n.t("trial-balance-title"),
                &report_qual,
                &org.map(|o| o.name).unwrap_or("".to_string()),
                &template_dir,
            ) {
                Ok(pdf_bytes) => (pdf_bytes, ContentType::PDF, "trial_balance.pdf".to_string()),
                Err(e) => return Err(ApiError::Internal(e)),
            }
        }
        _ => return Err(ApiError::BadRequest("Invalid format".to_string())),
    };

    Ok(DownloadFile::new(content, filename, content_type))
}

#[get("/api/reports/profit-loss/export/<format>?<start>&<end>")]
async fn export_profit_loss(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UseTransactions>,
    config: &State<TemplateConfig>,
    format: &str,
    start: &str,
    end: &str,
) -> Result<DownloadFile, ApiError> {
    let user = guard.0;
    let i18n = LocaleContext::new(&user.locale);
    let template_dir = config.root_directory.to_string_lossy();

    let start_date = NaiveDate::parse_from_str(&start, "%Y-%m-%d")
        .map_err(|_| ApiError::BadRequest("Invalid start date".to_string()))?;
    let end_date = NaiveDate::parse_from_str(&end, "%Y-%m-%d")
        .map_err(|_| ApiError::BadRequest("Invalid end date".to_string()))?;

    let accounts =
        report_service::get_profit_loss(&mut pool, user.organization_id, start_date, end_date)
            .await?;
    let org = crate::core::db::organization::get(&mut pool, user.organization_id).await?;

    let (content, content_type, filename) = match format {
        "csv" => {
            let csv_data = generate_profit_loss_csv(&user, &accounts);
            (
                csv_data.into_bytes(),
                ContentType::CSV,
                "profit_loss.csv".to_string(),
            )
        }
        "pdf" => {
            let typst_data = generate_profit_loss_typst(&user, &accounts);
            let start_date_str = i18n.format_date(start_date);
            let end_date_str = i18n.format_date(end_date);
            let report_qual = i18n.t_args(
                "general-ledger-export-period",
                &fluent_args!["start_date" => start_date_str, "end_date" => end_date_str],
            );
            match compile_typst_to_pdf(
                typst_data,
                &i18n.t("profit-loss-title"),
                &report_qual,
                &org.map(|o| o.name).unwrap_or("".to_string()),
                &template_dir,
            ) {
                Ok(pdf_bytes) => (pdf_bytes, ContentType::PDF, "trial_balance.pdf".to_string()),
                Err(e) => return Err(ApiError::Internal(e)),
            }
        }
        _ => return Err(ApiError::BadRequest("Invalid format".to_string())),
    };

    Ok(DownloadFile::new(content, filename, content_type))
}

#[get("/api/reports/balance-sheet/export/<format>?<date>")]
async fn export_balance_sheet(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UseTransactions>,
    config: &State<TemplateConfig>,
    format: &str,
    date: &str,
) -> Result<DownloadFile, ApiError> {
    let user = guard.0;
    let i18n = LocaleContext::new(&user.locale);
    let template_dir = config.root_directory.to_string_lossy();

    let report_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|_| ApiError::BadRequest("Invalid date".to_string()))?;

    let balance_sheet =
        report_service::get_balance_sheet(&mut pool, user.organization_id, report_date).await?;
    let org = crate::core::db::organization::get(&mut pool, user.organization_id).await?;

    let (content, content_type, filename) = match format {
        "csv" => {
            let csv_data = generate_balance_sheet_csv(&user, &balance_sheet);
            (
                csv_data.into_bytes(),
                ContentType::CSV,
                "balance_sheet.csv".to_string(),
            )
        }
        "pdf" => {
            let typst_data = generate_balance_sheet_typst(&user, &balance_sheet);
            let date_str = i18n.format_date(report_date);
            let report_qual = i18n.t_args(
                "balance-sheet-export-as-at",
                &fluent_args!["date" => date_str],
            );
            match compile_typst_to_pdf(
                typst_data,
                &i18n.t("balance-sheet-title"),
                &report_qual,
                &org.map(|o| o.name).unwrap_or("".to_string()),
                &template_dir,
            ) {
                Ok(pdf_bytes) => (pdf_bytes, ContentType::PDF, "trial_balance.pdf".to_string()),
                Err(e) => return Err(ApiError::Internal(e)),
            }
        }
        _ => return Err(ApiError::BadRequest("Invalid format".to_string())),
    };

    Ok(DownloadFile::new(content, filename, content_type))
}

#[get("/api/reports/general-ledger/export/<format>?<start>&<end>&<accounts>&<min_amount>")]
async fn export_general_ledger(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UseTransactions>,
    config: &State<TemplateConfig>,
    format: &str,
    start: &str,
    end: &str,
    accounts: Option<&str>,
    min_amount: Option<Decimal>,
) -> Result<DownloadFile, ApiError> {
    let user = guard.0;
    let i18n = LocaleContext::new(&user.locale);
    let template_dir = config.root_directory.to_string_lossy();

    let start_date = NaiveDate::parse_from_str(&start, "%Y-%m-%d")
        .map_err(|_| ApiError::BadRequest("Invalid start date".to_string()))?;
    let end_date = NaiveDate::parse_from_str(&end, "%Y-%m-%d")
        .map_err(|_| ApiError::BadRequest("Invalid end date".to_string()))?;

    let account_ids: Option<Vec<AccountId>> = accounts.map(|s| {
        s.split(',')
            .filter_map(|id| id.parse::<Uuid>().map(AccountId).ok())
            .collect()
    });

    //Validate the accounts are valid and in user organization
    if let Some(ref ids) = account_ids {
        for id in ids {
            let account = account::get(&mut pool, user.organization_id, *id).await?;
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
    let org = crate::core::db::organization::get(&mut pool, user.organization_id).await?;

    let (content, content_type, filename) = match format {
        "csv" => {
            let csv_data = generate_general_ledger_csv(&user, &lines);
            (
                csv_data.into_bytes(),
                ContentType::CSV,
                "general_ledger.csv".to_string(),
            )
        }
        "pdf" => {
            let typst_data = generate_general_ledger_typst(&user, &lines);

            let start_date_str = i18n.format_date(start_date);
            let end_date_str = i18n.format_date(end_date);
            let report_qual = i18n.t_args(
                "general-ledger-export-period",
                &fluent_args!["start_date" => start_date_str, "end_date" => end_date_str],
            );

            match compile_typst_to_pdf(
                typst_data,
                &i18n.t("account-ledger-export-title"),
                &report_qual,
                &org.map(|o| o.name).unwrap_or("".to_string()),
                &template_dir,
            ) {
                Ok(pdf_bytes) => (
                    pdf_bytes,
                    ContentType::PDF,
                    "general_ledger.pdf".to_string(),
                ),
                Err(e) => return Err(ApiError::Internal(e)),
            }
        }
        _ => return Err(ApiError::BadRequest("Invalid format".to_string())),
    };

    Ok(DownloadFile::new(content, filename, content_type))
}
