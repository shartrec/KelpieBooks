/*
 * Copyright (c) 2026. Trevor Campbell and others.
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

use rocket::serde::json::Json;
use rocket::{post, routes, Route};
use rocket_db_pools::Connection;
use sqlx::Acquire;
use shared_core::requests::transaction::CreateTransactionRequest;
use crate::db;
use crate::util::ApiError;
use crate::DbKelpie;
use crate::routes::security::AuthenticatedUser;

pub(crate) fn routes() -> Vec<Route> {
    routes![create_transaction]
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
        return Err(ApiError::Invalid("Transaction must be balanced and not zero.".to_string()));
    }

    let mut tx = pool.begin().await?;

    let transaction_id = db::transaction::insert(
        &mut tx,
        user.organization_id,
        req.date,
        req.description.clone(),
        req.reference.clone(),
    ).await?;

    for entry in &req.entries {
        db::journal_entry::insert(
            &mut tx,
            transaction_id,
            entry.account_id,
            entry.debit,
            entry.credit,
            entry.description.clone(),
        ).await?;
    }

    tx.commit().await?;

    Ok("Transaction created successfully.")
}
