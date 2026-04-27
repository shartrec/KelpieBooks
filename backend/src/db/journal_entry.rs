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

use rocket_db_pools::sqlx::{self, PgConnection, Row};
use shared_core::models::JournalEntry;
use uuid::Uuid;

fn from_row_to_journal_entry(row: &sqlx::postgres::PgRow) -> JournalEntry {
    JournalEntry {
        id: row.get("id"),
        transaction_id: row.get("transaction_id"),
        account_id: row.get("account_id"),
        debit: row.get("debit"),
        credit: row.get("credit"),
        description: row.get("description"),
    }
}

pub(crate) async fn get_all_by_org(
    pool: &mut PgConnection,
    organization_id: Uuid,
) -> Result<Vec<JournalEntry>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT je.*
        FROM journal_entries je
        JOIN transactions t ON je.transaction_id = t.id
        WHERE t.organization_id = $1
        "#
    )
    .bind(organization_id)
    .fetch_all(pool)
    .await
    .map(|rows| rows.iter().map(from_row_to_journal_entry).collect())
}
