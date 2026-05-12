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
use shared_core::models::Account;
use shared_core::models::{AccountCategory, SystemTag};
use shared_core::requests::account::{CreateAccountRequest, UpdateAccountRequest};
use std::collections::HashMap;
use std::str::FromStr;
use uuid::Uuid;

fn from_row_to_account(row: &sqlx::postgres::PgRow) -> Account {
    let category_str: String = row.get("category");
    let system_tag_str: Option<String> = row.get("system_tag");

    let category = AccountCategory::from_str(&category_str)
        .expect("DB schema and AccountCategory enum are out of sync!");

    let system_tag = system_tag_str.and_then(|s| {
        SystemTag::from_str(&s)
            .map_err(|e| {
                log::error!(
                    "DB schema and SystemTag enum are out of sync for value '{}': {}",
                    s,
                    e
                )
            })
            .ok()
    });

    Account {
        id: row.get("id"),
        organization_id: row.get("organization_id"),
        parent_id: row.get("parent_id"),
        code: row.get("code"),
        name: row.get("name"),
        category,
        is_group: row.get("is_group"),
        system_tag,
        created_at: row.get("created_at"),
    }
}

pub(crate) async fn get(pool: &mut PgConnection, id: Uuid) -> Result<Option<Account>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT
            id,
            organization_id,
            parent_id,
            code,
            name,
            category::TEXT as category,
            is_group,
            system_tag::TEXT as system_tag,
            created_at
        FROM accounts
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map(|row| row.map(|r| from_row_to_account(&r)))
}

pub(crate) async fn get_all_by_org(
    pool: &mut PgConnection,
    organization_id: Uuid,
) -> Result<Vec<Account>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT
            id,
            organization_id,
            parent_id,
            code,
            name,
            category::TEXT as category,
            is_group,
            system_tag::TEXT as system_tag,
            created_at
        FROM accounts
        WHERE organization_id = $1
        ORDER BY name
        "#,
    )
    .bind(organization_id)
    .fetch_all(pool)
    .await
    .map(|rows| rows.iter().map(from_row_to_account).collect())
}

pub(crate) async fn insert(
    pool: &mut PgConnection,
    org_id: Uuid,
    req: &CreateAccountRequest,
) -> Result<Account, sqlx::Error> {
    let row = sqlx::query(
        r#"
        INSERT INTO accounts (organization_id, name, code, category, parent_id, is_group, system_tag)
        VALUES ($1, $2, $3, $4::account_category, $5, $6, $7::system_tag)
        RETURNING id, organization_id, parent_id, code, name, category::TEXT as category, is_group, system_tag::TEXT as system_tag, created_at
        "#,
    )
    .bind(org_id)
    .bind(&req.name)
    .bind(&req.code)
    .bind(req.category.to_string())
    .bind(req.parent_id)
    .bind(req.is_group)
    .bind(req.system_tag.map(|s| s.to_string()))
    .fetch_one(pool)
    .await?;
    Ok(from_row_to_account(&row))
}

pub(crate) async fn update(
    pool: &mut PgConnection,
    id: Uuid,
    req: &UpdateAccountRequest,
) -> Result<Account, sqlx::Error> {
    let row = sqlx::query(
        r#"
        UPDATE accounts
        SET name = $1, code = $2, category = $3::account_category, is_group = $4, system_tag = $5::system_tag
        WHERE id = $6
        RETURNING id, organization_id, parent_id, code, name, category::TEXT as category, is_group, system_tag::TEXT as system_tag, created_at
        "#,
    )
    .bind(&req.name)
    .bind(&req.code)
    .bind(req.category.to_string())
    .bind(req.is_group)
    .bind(req.system_tag.map(|s| s.to_string()))
    .bind(id)
    .fetch_one(pool)
    .await?;
    Ok(from_row_to_account(&row))
}

pub(crate) async fn has_journal_entries(
    pool: &mut PgConnection,
    id: Uuid,
) -> Result<bool, sqlx::Error> {
    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM journal_entries WHERE account_id = $1")
            .bind(id)
            .fetch_one(pool)
            .await?;
    Ok(count > 0)
}

pub(crate) async fn delete(pool: &mut PgConnection, id: Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM accounts WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}

pub(crate) async fn get_all_by_category(
    pool: &mut PgConnection,
    organization_id: Uuid,
    categories: Vec<AccountCategory>,
) -> Result<Vec<Account>, sqlx::Error> {
    let category_strs: Vec<String> = categories.into_iter().map(|c| c.to_string()).collect();
    sqlx::query(
        r#"
        SELECT
            id,
            organization_id,
            parent_id,
            code,
            name,
            category::TEXT as category,
            is_group,
            system_tag::TEXT as system_tag,
            created_at
        FROM accounts
        WHERE organization_id = $1 AND category::TEXT = ANY($2)
        "#,
    )
    .bind(organization_id)
    .bind(category_strs)
    .fetch_all(pool)
    .await
    .map(|rows| rows.iter().map(from_row_to_account).collect())
}

pub(crate) async fn get_by_system_tag(
    pool: &mut PgConnection,
    organization_id: Uuid,
    tag: &str,
) -> Result<Option<Account>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT
            id,
            organization_id,
            parent_id,
            code,
            name,
            category::TEXT as category,
            is_group,
            system_tag::TEXT as system_tag,
            created_at
        FROM accounts
        WHERE organization_id = $1 AND system_tag::TEXT = $2
        "#,
    )
    .bind(organization_id)
    .bind(tag)
    .fetch_optional(pool)
    .await
    .map(|row| row.map(|r| from_row_to_account(&r)))
}

pub(crate) async fn get_system_accounts(
    pool: &mut PgConnection,
    organization_id: Uuid,
) -> Result<HashMap<SystemTag, Uuid>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT system_tag::TEXT, id
        FROM accounts
        WHERE organization_id = $1 AND system_tag IS NOT NULL
        "#,
    )
    .bind(organization_id)
    .fetch_all(pool)
    .await?;

    let mut map = HashMap::new();
    for row in rows {
        let tag_str: String = row.get(0);
        let id: Uuid = row.get(1);
        if let Ok(tag) = SystemTag::from_str(&tag_str) {
            map.insert(tag, id);
        }
    }
    Ok(map)
}

pub(crate) async fn update_system_accounts(
    pool: &mut PgConnection,
    organization_id: Uuid,
    system_accounts: HashMap<SystemTag, Uuid>,
) -> Result<(), sqlx::Error> {

    // Clear all existing system tags for the organization
    sqlx::query(
        r#"
        UPDATE accounts
        SET system_tag = NULL
        WHERE organization_id = $1 AND system_tag IS NOT NULL
        "#,
    )
    .bind(organization_id)
    .execute(&mut *pool)
    .await?;

    // Set the new system tags
    for (tag, account_id) in system_accounts {
        sqlx::query(
            r#"
            UPDATE accounts
            SET system_tag = $1::system_tag
            WHERE id = $2 AND organization_id = $3
            "#,
        )
        .bind(tag.to_string())
        .bind(account_id)
        .bind(organization_id)
        .execute(&mut *pool)
        .await?;
    }

    Ok(())
}
