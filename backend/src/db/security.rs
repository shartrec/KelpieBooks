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

use crate::routes::Role;
use crate::routes::Role::Guest;
use bcrypt::{hash, verify, DEFAULT_COST};
use rocket_db_pools::sqlx;
use rocket_db_pools::sqlx::PgConnection;
use sqlx::Row;

pub async fn check_login(pool: &mut PgConnection, username: &str, password: &str) ->
Result<Option<Role>, sqlx::Error>
{
    let result = sqlx::query(
        "SELECT password_hash FROM users WHERE name = $1")
        .bind(username)
        .fetch_optional(pool)
        .await?;

    if let Some(row) = result {
        let stored_hash: String = row.get("password_hash");
        if verify(password, &stored_hash).unwrap_or(false) {
            let role: &str = row.get("role");
            return Ok(Role::from(role));
        }
    }

    Ok(Some(Guest))
}

pub async fn create_initial_admin(pool: &mut PgConnection) -> Result<(), sqlx::Error> {
    let username = "admin";
    let password = "securepassword"; // Replace with a strong password
    let role = "admin@example.com";

    // Check if the admin user already exists
    let existing_user = sqlx::query(
        "SELECT user_id FROM users WHERE name = $1")
        .bind(username)
        .fetch_optional(&mut *pool)
        .await?;

    if existing_user.is_none() {
        // Hash the password
        let hashed_password = hash(password, DEFAULT_COST).expect("Failed to hash password");

        // Insert the admin user
        let _result = sqlx::query("INSERT INTO users (name, password_hash, email) VALUES ($1, $2, $3)")
            .bind(username)
            .bind(hashed_password)
            .bind(role)
            .execute(&mut *pool)
            .await?;
    }

    Ok(())
}