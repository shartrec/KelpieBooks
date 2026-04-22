/*
 * Copyright (c) 2025-2025. Trevor Campbell and others.
 *
 * This file is part of KelpieRustWeb.
 *
 * KelpieRustWeb is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License,or
 * (at your option) any later version.
 *
 * KelpieRustWeb is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with KelpieRustWeb; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 *
 * Contributors:
 *      Trevor Campbell
 *
 */
use shared_core::models::User;
use log::error;
use rocket_db_pools::sqlx::{self, PgConnection};
use uuid::Uuid;

pub(crate) async fn insert(
    pool: &mut PgConnection,
    organization_id: Uuid,
    email: String,
    password_hash: String,
) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        "INSERT INTO users (organization_id, email, password_hash) VALUES ($1, $2, $3) RETURNING *",
        organization_id,
        email,
        password_hash
    )
    .fetch_one(pool)
    .await
}

pub(crate) async fn update(
    pool: &mut PgConnection,
    id: Uuid,
    email: String,
    password_hash: String,
) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        "UPDATE users SET email=$1, password_hash=$2 WHERE id = $3 RETURNING *",
        email,
        password_hash,
        id
    )
    .fetch_one(pool)
    .await
}

pub(crate) async fn delete(pool: &mut PgConnection, id: Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await;
    match result {
        Ok(result) => Ok(result.rows_affected()),
        Err(e) => {
            error!("Error deleting user: {}", e);
            Err(e)
        }
    }
}

pub(crate) async fn get(pool: &mut PgConnection, id: Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_optional(pool)
        .await
}

pub(crate) async fn get_by_email(pool: &mut PgConnection, email: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
        .fetch_optional(pool)
        .await
}

pub(crate) async fn get_all(pool: &mut PgConnection, organization_id: Uuid) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE organization_id = $1 ORDER BY email",
        organization_id
    )
    .fetch_all(pool)
    .await
}
