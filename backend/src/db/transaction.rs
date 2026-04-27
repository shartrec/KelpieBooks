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

use rocket_db_pools::sqlx::{self, PgConnection, Row};
use uuid::Uuid;
use chrono::NaiveDate;

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
