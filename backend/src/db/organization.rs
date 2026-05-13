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
use uuid::Uuid;
use shared_core::models::Organization;

pub(crate) async fn get(
    pool: &mut PgConnection,
    id: Uuid,
) -> Result<Option<Organization>, sqlx::Error> {
    sqlx::query("SELECT * FROM organizations WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
    .await
        .map(|row| row.map(|r| from_row_to_org(&r)))
}

pub(crate) async fn set_lock_date(
    pool: &mut PgConnection,
    id: Uuid,
    date: Option<NaiveDate>,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE organizations SET locked_until = $1 WHERE id = $2")
        .bind(date)
        .bind(id)
        .execute(pool)
    .await?;
    Ok(())
}
pub(crate) async fn set_audit_mode(
    pool: &mut PgConnection,
    id: Uuid,
    mode: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE organizations SET strict_audit_mode = $1 WHERE id = $2")
        .bind(mode)
        .bind(id)
        .execute(pool)
    .await?;
    Ok(())
}

fn from_row_to_org(row: &sqlx::postgres::PgRow) -> Organization {
    Organization {
        id: row.get("id"),
        name: row.get("name"),
        strict_audit_mode: row.get("strict_audit_mode"),
        created_at: row.get("created_at"),
        locked_until: row.get("locked_until"),
    }
}

pub async fn create(tx: &mut PgConnection, name: &str) -> Result<Organization, sqlx::Error> {
    let row = sqlx::query("INSERT INTO organizations (name) VALUES ($1) RETURNING *")
        .bind(name)
        .fetch_one(tx)
        .await?;
    Ok(from_row_to_org(&row))
}
