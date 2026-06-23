/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use std::collections::HashMap;

use rocket_db_pools::sqlx::{
    self,
    PgConnection,
    Row,
};
use shared_core::ledger::{
    models::{
        account::Account,
        account_category::AccountCategory,
        system_tag::SystemTag,
    },
    requests::account::{
        CreateAccountRequest,
        UpdateAccountRequest,
    },
};
use uuid::Uuid;

fn from_row_to_account(row: &sqlx::postgres::PgRow) -> Account {
    Account {
        id: row.get("id"),
        organization_id: row.get("organization_id"),
        parent_id: row.get("parent_id"),
        code: row.get("code"),
        name: row.get("name"),
        category: row.get("category"),
        is_group: row.get("is_group"),
        is_bank_account: row.get("is_bank_account"),
        system_tag: row.get("system_tag"),
        created_at: row.get("created_at"),
    }
}

pub(crate) async fn get(
    pool: &mut PgConnection,
    id: Uuid,
    org_id: Uuid,
) -> Result<Option<Account>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT
            id,
            organization_id,
            parent_id,
            code,
            name,
            category,
            is_group,
            is_bank_account,
            system_tag,
            created_at
        FROM accounts
        WHERE id = $1
        AND organization_id = $2
        "#,
    )
    .bind(id)
    .bind(org_id)
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
            category,
            is_group,
            is_bank_account,
            system_tag,
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
        INSERT INTO accounts (organization_id, name, code, category, parent_id, is_group, is_bank_account, system_tag)
        VALUES ($1, $2, $3, $4::account_category, $5, $6, $7, $8::system_tag)
        RETURNING id, organization_id, parent_id, code, name, category, is_group, is_bank_account, system_tag, created_at
        "#,
    )
    .bind(org_id)
    .bind(&req.name)
    .bind(&req.code)
    .bind(req.category)
    .bind(req.parent_id)
    .bind(req.is_group)
    .bind(req.is_bank_account)
    .bind(req.system_tag)
    .fetch_one(pool)
    .await?;
    Ok(from_row_to_account(&row))
}

pub(crate) async fn update(
    pool: &mut PgConnection,
    id: Uuid,
    org_id: Uuid,
    req: &UpdateAccountRequest,
) -> Result<Account, sqlx::Error> {
    let row = sqlx::query(
        r#"
        UPDATE accounts
        SET name = $1, code = $2, category = $3::account_category, is_group = $4, is_bank_account = $5, system_tag = $6::system_tag
        WHERE id = $7
        AND organization_id = $8
        RETURNING id, organization_id, parent_id, code, name, category, is_group, is_bank_account, system_tag, created_at
        "#,
    )
    .bind(&req.name)
    .bind(&req.code)
    .bind(req.category)
    .bind(req.is_group)
    .bind(req.is_bank_account)
    .bind(req.system_tag)
    .bind(id)
    .bind(org_id)
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

pub(crate) async fn delete(
    pool: &mut PgConnection,
    id: Uuid,
    org_id: Uuid,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM accounts WHERE id = $1 AND organization_id = $2")
        .bind(id)
        .bind(org_id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}

pub(crate) async fn get_all_by_category(
    pool: &mut PgConnection,
    organization_id: Uuid,
    categories: &[AccountCategory],
) -> Result<Vec<Account>, sqlx::Error> {
    let mut query = String::from(
        r#"
        SELECT
            id,
            organization_id,
            parent_id,
            code,
            name,
            category,
            is_group,
            is_bank_account,
            system_tag,
            created_at
        FROM accounts
        WHERE organization_id = $1
        "#,
    );
    let mut i = 2i32;
    let or_clauses: Vec<String> = (0..categories.len())
        .map(|_| {
            let clause = format!("category = ${}", i);
            i += 1;
            clause
        })
        .collect();
    query.push_str(&format!(" AND ({})", or_clauses.join(" OR ")));

    let mut query = sqlx::query(&query).bind(organization_id);
    for category in categories {
        query = query.bind(category);
    }
    query
        .fetch_all(pool)
        .await
        .map(|rows| rows.iter().map(from_row_to_account).collect())
}

pub(crate) async fn get_by_system_tag(
    pool: &mut PgConnection,
    organization_id: Uuid,
    tag: &SystemTag,
) -> Result<Option<Account>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT
            id,
            organization_id,
            parent_id,
            code,
            name,
            category,
            is_group,
            is_bank_account,
            system_tag,
            created_at
        FROM accounts
        WHERE organization_id = $1 AND system_tag = $2
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
        SELECT system_tag, id
        FROM accounts
        WHERE organization_id = $1 AND system_tag IS NOT NULL
        "#,
    )
    .bind(organization_id)
    .fetch_all(pool)
    .await?;

    let mut map = HashMap::new();
    for row in rows.iter() {
        let tag: Option<SystemTag> = row.get("system_tag");
        let id: Uuid = row.get("id");
        if let Some(tag) = tag {
            map.insert(tag, id);
        }
    }
    Ok(map)
}

pub(crate) async fn update_system_accounts(
    pool: &mut PgConnection,
    organization_id: Uuid,
    system_accounts: &HashMap<SystemTag, Uuid>,
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
            SET system_tag = $1
            WHERE id = $2 AND organization_id = $3
            "#,
        )
        .bind(tag)
        .bind(account_id)
        .bind(organization_id)
        .execute(&mut *pool)
        .await?;
    }

    Ok(())
}
