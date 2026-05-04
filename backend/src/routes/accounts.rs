use crate::db::account as account_db;
use crate::routes::security::AuthenticatedUser;
use crate::services::account_service;
use crate::util::types::PathUuid;
use crate::util::ApiError;
use crate::DbKelpie;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, routes, Route};
use rocket_db_pools::Connection;
use shared_core::dtos::account_with_balance::AccountWithBalance;
use shared_core::dtos::journal_entry_with_balance::JournalEntryWithBalance;
use shared_core::models::Account;
use shared_core::requests::account::{CreateAccountRequest, UpdateAccountRequest};
use crate::export::DownloadFile;
use crate::export::account_ledger_export::{generate_ledger_csv, generate_ledger_typst};
use rocket::http::ContentType;

pub(crate) fn routes() -> Vec<Route> {
    routes![
        get_accounts,
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
    user: AuthenticatedUser,
) -> Result<Json<Vec<AccountWithBalance>>, ApiError> {
    let accounts =
        account_service::get_accounts_with_balances(&mut pool, user.organization_id).await?;
    Ok(Json(accounts))
}

#[get("/api/accounts/<id>")]
async fn get_account(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
) -> Result<Json<Account>, ApiError> {
    let account = account_db::get(&mut pool, *id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Account not found".to_string()))?;
    Ok(Json(account))
}

#[get("/api/accounts/<id>/entries")]
async fn get_account_entries(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
) -> Result<Json<Vec<JournalEntryWithBalance>>, ApiError> {
    let entries = account_service::get_journal_entries_with_running_balance(&mut pool, *id).await?;
    Ok(Json(entries))
}

#[post("/api/accounts", data = "<req>")]
async fn create_account(
    mut pool: Connection<DbKelpie>,
    user: AuthenticatedUser,
    req: Json<CreateAccountRequest>,
) -> Result<Json<AccountWithBalance>, ApiError> {
    let new_account = account_db::insert(&mut pool, user.organization_id, &req).await?;
    Ok(Json(AccountWithBalance {
        id: new_account.id,
        organization_id: new_account.organization_id,
        parent_id: new_account.parent_id,
        code: new_account.code,
        name: new_account.name,
        category: new_account.category,
        is_group: new_account.is_group,
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
) -> Result<Json<AccountWithBalance>, ApiError> {
    let updated_account = account_db::update(&mut pool, *id, &req).await?;
    Ok(Json(AccountWithBalance {
        id: updated_account.id,
        organization_id: updated_account.organization_id,
        parent_id: updated_account.parent_id,
        code: updated_account.code,
        name: updated_account.name,
        category: updated_account.category,
        is_group: updated_account.is_group,
        system_tag: updated_account.system_tag,
        created_at: updated_account.created_at,
        balance: 0,
    }))
}

#[delete("/api/accounts/<id>")]
async fn delete_account(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
) -> Result<&'static str, ApiError> {
    if account_db::has_journal_entries(&mut pool, *id).await? {
        return Err(ApiError::Conflict(
            "Cannot delete an account with journal entries.".to_string(),
        ));
    }

    let rows_affected = account_db::delete(&mut pool, *id).await?;
    if rows_affected == 0 {
        return Err(ApiError::NotFound("Account not found.".to_string()));
    }

    Ok("Account deleted successfully.")
}

#[get("/api/accounts/<id>/export/<format>")]
async fn export_account_ledger(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
    format: String,
) -> Result<DownloadFile, ApiError> {
    let entries = account_service::get_journal_entries_with_running_balance(&mut pool, *id).await?;

    let (content, content_type, filename) = match format.as_str() {
        "csv" => {
            let csv_data = generate_ledger_csv(&entries);
            (csv_data.into_bytes(), ContentType::CSV, "account_ledger.csv".to_string())
        }
        "typst" => {
            let typst_data = generate_ledger_typst(&entries);
            (typst_data.into_bytes(), ContentType::Plain, "account_ledger.typ".to_string())
        }
        _ => return Err(ApiError::Invalid("Invalid format".to_string())),
    };

    Ok(DownloadFile::new(content, filename, content_type))
}
