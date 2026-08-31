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
};
use shared_core::{
    ledger::{
        models::{
            account::Account,
            account_category::AccountCategory,
            system_tag::SystemTag,
        },
        requests::account::{
            CreateAccountRequest,
            UpdateAccountRequest,
        },
    },
    AccountId,
    OrgId,
};

pub(crate) async fn get(
    pool: &mut PgConnection,
    org_id: OrgId,
    id: AccountId,
) -> Result<Option<Account>, sqlx::Error> {
    sqlx::query_as!(
        Account,
        r#"
        SELECT
            id,
            organization_id,
            parent_id as "parent_id: AccountId",
            code,
            name,
            category as "category: AccountCategory",
            is_group,
            is_bank_account,
            system_tag as "system_tag: SystemTag",
            created_at
        FROM accounts
        WHERE id = $1
        AND organization_id = $2
        "#,
        *id,
        *org_id,
    )
    .fetch_optional(pool)
    .await
}

pub(crate) async fn get_all_by_org(
    pool: &mut PgConnection,
    organization_id: OrgId,
) -> Result<Vec<Account>, sqlx::Error> {
    sqlx::query_as!(
        Account,
        r#"
        SELECT
            id,
            organization_id,
            parent_id as "parent_id: AccountId",
            code,
            name,
            category as "category: AccountCategory",
            is_group,
            is_bank_account,
            system_tag as "system_tag: SystemTag",
            created_at
        FROM accounts
        WHERE organization_id = $1
        ORDER BY name
        "#,
        *organization_id,
    )
    .fetch_all(pool)
    .await
}

pub(crate) async fn insert(
    pool: &mut PgConnection,
    org_id: OrgId,
    req: &CreateAccountRequest,
) -> Result<Account, sqlx::Error> {
    let account = sqlx::query_as!(
        Account,
        r#"
        INSERT INTO accounts (organization_id, name, code, category, parent_id, is_group, is_bank_account, system_tag)
        VALUES ($1, $2, $3, $4::account_category, $5, $6, $7, $8::system_tag)
        RETURNING id, organization_id, parent_id as "parent_id: AccountId", code, name, category as "category: AccountCategory",
            is_group, is_bank_account, system_tag as "system_tag: SystemTag", created_at
        "#,
    *org_id,
    &req.name,
    &req.code,
    req.category as AccountCategory,
    req.parent_id.map(|id| *id),
    req.is_group,
    req.is_bank_account,
    req.system_tag as Option<SystemTag>,
    )
    .   fetch_one(pool)
    .await?;
    Ok(account)
}

pub(crate) async fn update(
    pool: &mut PgConnection,
    org_id: OrgId,
    id: AccountId,
    req: &UpdateAccountRequest,
) -> Result<Account, sqlx::Error> {
    let account = sqlx::query_as!(
        Account,
        r#"
        UPDATE accounts
        SET name = $1, code = $2, category = $3::account_category, is_group = $4, is_bank_account = $5, system_tag = $6::system_tag
        WHERE id = $7
        AND organization_id = $8
        RETURNING id, organization_id, parent_id as "parent_id: AccountId", code, name, category as "category: AccountCategory",
            is_group, is_bank_account, system_tag as "system_tag: SystemTag", created_at
        "#,
        &req.name,
        &req.code,
        req.category as AccountCategory,
        req.is_group,
        req.is_bank_account,
        req.system_tag.clone() as Option<SystemTag>,
        *id,
        *org_id,
    )
    .fetch_one(pool)
    .await?;
    Ok(account)
}

pub(crate) async fn has_journal_entries(
    pool: &mut PgConnection,
    id: AccountId,
) -> Result<bool, sqlx::Error> {
    let count: Option<i64> = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM journal_entries WHERE account_id = $1",
        *id
    )
    .fetch_one(pool)
    .await?;
    Ok(count.unwrap_or(0) > 0)
}

pub(crate) async fn delete(
    pool: &mut PgConnection,
    org_id: OrgId,
    id: AccountId,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        "DELETE FROM accounts WHERE id = $1 AND organization_id = $2",
        *id,
        *org_id,
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}

pub(crate) async fn get_all_by_category(
    pool: &mut PgConnection,
    organization_id: OrgId,
    categories: &[AccountCategory],
) -> Result<Vec<Account>, sqlx::Error> {
    let mut query = sqlx::QueryBuilder::<sqlx::Postgres>::new(
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
        WHERE organization_id =
        "#,
    );

    query.push_bind(organization_id);

    query.push(" AND category IN (");

    let mut separated = query.separated(", ");

    for category in categories {
        separated.push_bind(category);
    }

    separated.push_unseparated(")");

    query.build_query_as::<Account>().fetch_all(pool).await
}

pub(crate) async fn get_by_system_tag(
    pool: &mut PgConnection,
    org_id: OrgId,
    tag: &SystemTag,
) -> Result<Option<Account>, sqlx::Error> {
    sqlx::query_as!(
        Account,
        r#"
        SELECT
            id,
            organization_id,
            parent_id as "parent_id: AccountId",
            code,
            name,
            category as "category: AccountCategory",
            is_group,
            is_bank_account,
            system_tag as "system_tag: SystemTag",
            created_at
        FROM accounts
        WHERE organization_id = $1 AND system_tag = $2::system_tag
        "#,
        *org_id,
        tag as &SystemTag
    )
    .fetch_optional(pool)
    .await
}

pub(crate) async fn get_system_accounts(
    pool: &mut PgConnection,
    org_id: OrgId,
) -> Result<HashMap<SystemTag, AccountId>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT system_tag as "system_tag: SystemTag", id as "id: AccountId"
        FROM accounts
        WHERE organization_id = $1 AND system_tag IS NOT NULL
        "#,
        *org_id,
    )
    .fetch_all(pool)
    .await?;

    let mut map = HashMap::new();
    for row in rows.iter() {
        let tag: Option<SystemTag> = row.system_tag;
        let id: AccountId = row.id;
        if let Some(tag) = tag {
            map.insert(tag, id);
        }
    }
    Ok(map)
}

pub(crate) async fn update_system_accounts(
    pool: &mut PgConnection,
    org_id: OrgId,
    system_accounts: &HashMap<SystemTag, AccountId>,
) -> Result<(), sqlx::Error> {
    // Clear all existing system tags for the organization
    sqlx::query!(
        r#"
        UPDATE accounts
        SET system_tag = NULL
        WHERE organization_id = $1 AND system_tag IS NOT NULL
        "#,
        *org_id,
    )
    .execute(&mut *pool)
    .await?;

    // Set the new system tags
    for (tag, account_id) in system_accounts {
        sqlx::query!(
            r#"
            UPDATE accounts
            SET system_tag = $1
            WHERE id = $2 AND organization_id = $3
            "#,
            tag as &SystemTag,
            **account_id,
            *org_id,
        )
        .execute(&mut *pool)
        .await?;
    }

    Ok(())
}
