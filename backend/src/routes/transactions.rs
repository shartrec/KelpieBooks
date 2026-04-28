use crate::db;
use crate::routes::security::AuthenticatedUser;
use crate::util::types::PathUuid;
use crate::util::ApiError;
use crate::DbKelpie;
use rocket::serde::json::Json;
use rocket::{get, post, routes, Route};
use rocket_db_pools::Connection;
use shared_core::dtos::transaction_detail::TransactionDetail;
use shared_core::requests::transaction::CreateTransactionRequest;
use sqlx::Acquire;

pub(crate) fn routes() -> Vec<Route> {
    routes![create_transaction, get_transaction, reverse_transaction]
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

#[post("/api/transactions/<id>/reverse")]
async fn reverse_transaction(
    mut pool: Connection<DbKelpie>,
    user: AuthenticatedUser,
    id: PathUuid,
) -> Result<&'static str, ApiError> {
    let original_transaction = db::transaction::get(&mut pool, *id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Transaction not found".to_string()))?;

    let original_entries = db::journal_entry::get_all_by_transaction(&mut pool, *id).await?;

    let reversal_description = format!(
        "Reversal of transaction {}",
        &original_transaction.id.to_string()[..8]
    );

    let mut tx = pool.begin().await?;

    let new_transaction_id = db::transaction::insert(
        &mut tx,
        user.organization_id,
        original_transaction.date,
        Some(reversal_description),
        original_transaction.reference,
    )
    .await?;

    for entry in &original_entries {
        db::journal_entry::insert(
            &mut tx,
            new_transaction_id,
            entry.account_id,
            entry.credit, // Swap debit and credit
            entry.debit,
            entry.description.clone(),
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
    let total_debits: i64 = req.entries.iter().map(|e| e.debit).sum();
    let total_credits: i64 = req.entries.iter().map(|e| e.credit).sum();

    if total_debits == 0 || total_credits == 0 || total_debits != total_credits {
        return Err(ApiError::Invalid(
            "Transaction must be balanced and not zero.".to_string(),
        ));
    }

    let main_description = req.entries.get(0).and_then(|e| e.description.clone());

    let mut tx = pool.begin().await?;

    let transaction_id = db::transaction::insert(
        &mut tx,
        user.organization_id,
        req.date,
        main_description,
        req.reference.clone(),
    )
    .await?;

    for entry in &req.entries {
        db::journal_entry::insert(
            &mut tx,
            transaction_id,
            entry.account_id,
            entry.debit,
            entry.credit,
            entry.description.clone(),
        )
        .await?;
    }

    tx.commit().await?;

    Ok("Transaction created successfully.")
}
