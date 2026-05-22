use crate::db;
use crate::routes::security::AuthenticatedUser;
use crate::services::account_service;
use crate::util::types::PathUuid;
use crate::util::ApiError;
use crate::DbKelpie;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, routes, Route};
use rocket_db_pools::Connection;
use shared_core::dtos::transaction_detail::TransactionDetail;
use shared_core::requests::transaction::{
    CreateTransactionRequest, ReverseTransactionRequest, UpdateTransactionRequest,
};
use sqlx::Acquire;

pub(crate) fn routes() -> Vec<Route> {
    routes![
        create_transaction,
        get_transaction,
        reverse_transaction,
        delete_transaction,
        update_transaction
    ]
}

#[get("/api/transactions/<id>")]
async fn get_transaction(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
) -> Result<Json<TransactionDetail>, ApiError> {
    let transaction = db::transaction::get(&mut pool, *id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Transaction not found".to_string()))?;

    let entries = db::journal_entry::get_all_by_transaction(&mut pool, *id).await?;

    Ok(Json(TransactionDetail {
        transaction,
        entries,
    }))
}

#[delete("/api/transactions/<id>")]
async fn delete_transaction(
    mut pool: Connection<DbKelpie>,
    user: AuthenticatedUser,
    id: PathUuid,
) -> Result<&'static str, ApiError> {
    let transaction = db::transaction::get(&mut pool, *id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Transaction not found".to_string()))?;

    let organization = db::organization::get(&mut pool, user.organization_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Organization not found".to_string()))?;

    if let Some(locked_until) = organization.locked_until {
        if transaction.date <= locked_until {
            return Err(ApiError::Forbidden(
                "Period is locked for editing".to_string(),
            ));
        }
    }

    if organization.strict_audit_mode {
        return Err(ApiError::Forbidden(
            "Cannot delete transactions in strict audit mode.".to_string(),
        ));
    }

    db::transaction::delete(&mut pool, *id).await?;

    Ok("Transaction deleted successfully.")
}

#[put("/api/transactions/<id>", data = "<req>")]
async fn update_transaction(
    mut pool: Connection<DbKelpie>,
    user: AuthenticatedUser,
    id: PathUuid,
    req: Json<UpdateTransactionRequest>,
) -> Result<&'static str, ApiError> {
    let total_debits: i64 = req.entries.iter().map(|e| e.debit).sum();
    let total_credits: i64 = req.entries.iter().map(|e| e.credit).sum();

    if total_debits == 0 || total_credits == 0 || total_debits != total_credits {
        return Err(ApiError::Invalid(
            "Transaction must be balanced and not zero.".to_string(),
        ));
    }

    let organization = db::organization::get(&mut pool, user.organization_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Organization not found".to_string()))?;

    let original_transaction = db::transaction::get(&mut pool, *id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Transaction not found".to_string()))?;

    if let Some(locked_until) = organization.locked_until {
        if req.date <= locked_until || original_transaction.date <= locked_until {
            return Err(ApiError::Forbidden(
                "Period is locked for editing".to_string(),
            ));
        }
    }

    if organization.strict_audit_mode {
        return Err(ApiError::Forbidden(
            "Cannot edit transactions in strict audit mode.".to_string(),
        ));
    }

    let main_description = &req.entries.get(0).and_then(|e| e.description.clone());

    let mut tx = pool.begin().await?;

    db::transaction::delete(&mut tx, *id).await?;

    let transaction_id = db::transaction::insert(
        &mut tx,
        user.organization_id,
        req.date,
        main_description,
        &req.reference,
    )
    .await?;

    for entry in &req.entries {
        db::journal_entry::insert(
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

    Ok("Transaction updated successfully.")
}

#[post("/api/transactions/<id>/reverse", data = "<req>")]
async fn reverse_transaction(
    mut pool: Connection<DbKelpie>,
    user: AuthenticatedUser,
    id: PathUuid,
    req: Json<ReverseTransactionRequest>,
) -> Result<&'static str, ApiError> {
    let original_transaction = db::transaction::get(&mut pool, *id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Transaction not found".to_string()))?;

    let organization = db::organization::get(&mut pool, user.organization_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Organization not found".to_string()))?;

    let reversal_date = if let Some(locked_until) = organization.locked_until {
        if original_transaction.date <= locked_until {
            locked_until.succ_opt().unwrap_or(locked_until)
        } else {
            original_transaction.date
        }
    } else {
        original_transaction.date
    };

    let original_entries = db::journal_entry::get_all_by_transaction(&mut pool, *id).await?;

    let mut tx = pool.begin().await?;

    let new_transaction_id = db::transaction::insert(
        &mut tx,
        user.organization_id,
        reversal_date,
        &Some(req.description.clone()),
        &original_transaction.reference,
    )
    .await?;

    for entry in &original_entries {
        db::journal_entry::insert(
            &mut tx,
            new_transaction_id,
            entry.account_id,
            entry.credit, // Swap debit and credit
            entry.debit,
            Some(format!(
                "{} - {}",
                req.description,
                entry.description.as_deref().unwrap_or("")
            ).as_str()),
        )
        .await?;
    }

    tx.commit().await?;

    Ok("Transaction reversed successfully.")
}

#[post("/api/transactions", data = "<req>")]
async fn create_transaction(
    mut pool: Connection<DbKelpie>,
    user: AuthenticatedUser,
    req: Json<CreateTransactionRequest>,
) -> Result<&'static str, ApiError> {
    account_service::create_transaction(&mut pool, user.organization_id, &req).await?;
    Ok("Transaction created successfully.")
}
