/*
 * Copyright (c) 2026-2026. Trevor Campbell and others.
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

use rocket_db_pools::sqlx::{self, PgConnection};
use shared_core::models::Organization;

pub async fn create(
    tx: &mut PgConnection,
    name: &str,
) -> Result<Organization, sqlx::Error> {
    sqlx::query_as!(
        Organization,
        "INSERT INTO organizations (name) VALUES ($1) RETURNING *",
        name
    )
    .fetch_one(tx)
    .await
}
