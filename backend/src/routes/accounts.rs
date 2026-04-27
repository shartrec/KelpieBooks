use rocket::serde::json::Json;
use rocket::{get, post, put, delete, routes, Route};
use rocket_db_pools::Connection;
use shared_core::dtos::account_with_balance::AccountWithBalance;
use crate::services::account_service;
use crate::util::ApiError;
use crate::DbKelpie;
use crate::routes::security::AuthenticatedUser;
use shared_core::requests::account::{CreateAccountRequest, UpdateAccountRequest};
use crate::db::account as account_db;
use crate::util::types::PathUuid;

pub(crate) fn routes() -> Vec<Route> {
    routes![get_accounts, create_account, update_account, delete_account]
}

#[get("/api/accounts")]
async fn get_accounts(
    mut pool: Connection<DbKelpie>,
    user: AuthenticatedUser,
) -> Result<Json<Vec<AccountWithBalance>>, ApiError> {
    let accounts = account_service::get_accounts_with_balances(&mut pool, user.organization_id).await?;
    Ok(Json(accounts))
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
        return Err(ApiError::Conflict("Cannot delete an account with journal entries.".to_string()));
    }

    let rows_affected = account_db::delete(&mut pool, *id).await?;
    if rows_affected == 0 {
        return Err(ApiError::NotFound("Account not found.".to_string()));
    }

    Ok("Account deleted successfully.")
}
