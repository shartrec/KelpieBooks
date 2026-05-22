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
use rocket_db_pools::sqlx::{self, PgConnection, Row};
use shared_core::models::user::User;
use shared_core::models::user_with_org::UserWithOrg;
use uuid::Uuid;

fn from_row_to_user_with_org(row: &sqlx::postgres::PgRow) -> UserWithOrg {
    UserWithOrg {
        id: row.get("id"),
        organization_id: row.get("organization_id"),
        email: row.get("email"),
        full_name: row.get("full_name"),
        display_name: row.get("display_name"),
        password_hash: row.get("password_hash"),
        created_at: row.get("created_at"),
        organisation_name: row.get("organisation_name"),
        strict_audit_mode: row.get("strict_audit_mode"),
    }
}

pub(crate) async fn insert(
    pool: &mut PgConnection,
    organization_id: Uuid,
    email: &str,
    password_hash: &str,
    full_name: &str,
    display_name: Option<&str>,
) -> Result<User, sqlx::Error> {
    let row = sqlx::query(
        "INSERT INTO users (organization_id, email, password_hash, full_name, display_name) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(organization_id)
    .bind(email)
    .bind(password_hash)
    .bind(full_name)
    .bind(display_name)
    .fetch_one(pool)
    .await?;
    Ok(User {
        id: row.get("id"),
        organization_id: row.get("organization_id"),
        email: row.get("email"),
        full_name: row.get("full_name"),
        display_name: row.get("display_name"),
        password_hash: row.get("password_hash"),
        created_at: row.get("created_at"),
    })
}

pub(crate) async fn update(
    pool: &mut PgConnection,
    id: Uuid,
    email: &str,
    password_hash: &str,
    full_name: &str,
    display_name: Option<&str>,
) -> Result<User, sqlx::Error> {
    let row = sqlx::query(
        "UPDATE users SET email=$1, password_hash=$2, full_name=$3, display_name=$4 WHERE id = $5 RETURNING *"
    )
    .bind(email)
    .bind(password_hash)
    .bind(full_name)
    .bind(display_name)
    .bind(id)
    .fetch_one(pool)
    .await?;
    Ok(User {
        id: row.get("id"),
        organization_id: row.get("organization_id"),
        email: row.get("email"),
        full_name: row.get("full_name"),
        display_name: row.get("display_name"),
        password_hash: row.get("password_hash"),
        created_at: row.get("created_at"),
    })
}

pub(crate) async fn delete(pool: &mut PgConnection, id: Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}

pub(crate) async fn get(
    pool: &mut PgConnection,
    id: Uuid,
) -> Result<Option<UserWithOrg>, sqlx::Error> {
    sqlx::query("SELECT u.*, o.name as organisation_name, o.strict_audit_mode FROM users u JOIN organizations o ON u.organization_id = o.id WHERE u.id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map(|row| row.map(|r| from_row_to_user_with_org(&r)))
}

pub(crate) async fn get_by_email(
    pool: &mut PgConnection,
    email: &str,
) -> Result<Option<UserWithOrg>, sqlx::Error> {
    sqlx::query("SELECT u.*, o.name as organisation_name, o.strict_audit_mode FROM users u JOIN organizations o ON u.organization_id = o.id WHERE u.email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await
        .map(|row| row.map(|r| from_row_to_user_with_org(&r)))
}

pub(crate) async fn get_all(
    pool: &mut PgConnection,
    organization_id: Uuid,
) -> Result<Vec<UserWithOrg>, sqlx::Error> {
    sqlx::query("SELECT u.*, o.name as organisation_name, o.strict_audit_mode FROM users u JOIN organizations o ON u.organization_id = o.id WHERE u.organization_id = $1 ORDER BY u.email")
        .bind(organization_id)
        .fetch_all(pool)
        .await
        .map(|rows| rows.iter().map(from_row_to_user_with_org).collect())
}
