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

use chrono::NaiveDate;
use rocket_db_pools::sqlx::{self, PgConnection, Row};
use shared_core::models::Transaction;
use uuid::Uuid;

fn from_row_to_transaction(row: &sqlx::postgres::PgRow) -> Transaction {
    Transaction {
        id: row.get("id"),
        organization_id: row.get("organization_id"),
        date: row.get("date"),
        description: row.get("description"),
        reference: row.get("reference"),
        created_at: row.get("created_at"),
    }
}

pub(crate) async fn get(
    pool: &mut PgConnection,
    id: Uuid,
) -> Result<Option<Transaction>, sqlx::Error> {
    sqlx::query("SELECT * FROM transactions WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map(|row| row.map(|r| from_row_to_transaction(&r)))
}

pub(crate) async fn insert(
    pool: &mut PgConnection,
    organization_id: Uuid,
    date: NaiveDate,
    description: Option<String>,
    reference: Option<String>,
) -> Result<Uuid, sqlx::Error> {
    let row = sqlx::query(
        "INSERT INTO transactions (organization_id, date, description, reference) VALUES ($1, $2, $3, $4) RETURNING id"
    )
    .bind(organization_id)
    .bind(date)
    .bind(description)
    .bind(reference)
    .fetch_one(pool)
    .await?;
    Ok(row.get("id"))
}
