/*
 * Copyright (c) 2025-2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use crate::util::locale_context::LocaleContext;
use crate::util::ApiError;
use rocket_db_pools::sqlx::{self, PgConnection, Row};
use shared_core::models::auth::SystemPrivilege;
use shared_core::models::role::Role;
use shared_core::models::user::User;
use shared_core::models::user_with_org::UserWithOrg;
use uuid::Uuid;

fn from_row_to_user_with_org(row: &sqlx::postgres::PgRow) -> UserWithOrg {
    let role = if row.get::<Option<Uuid>, _>("role_id").is_some() {
        Some(Role {
            id: row.get("role_id"),
            name: row.get("role_name"),
            privileges: vec![],
        })
    } else {
        None
    };

    UserWithOrg {
        id: row.get("id"),
        organization_id: row.get("organization_id"),
        email: row.get("email"),
        full_name: row.get("full_name"),
        display_name: row.get("display_name"),
        password_hash: row.get("password_hash"),
        created_at: row.get("user_created_at"),
        organisation_name: row.get("organisation_name"),
        strict_audit_mode: row.get("strict_audit_mode"),
        role
    }
}

pub(crate) async fn insert(
    pool: &mut PgConnection,
    organization_id: Uuid,
    email: &str,
    password_hash: &str,
    full_name: &str,
    display_name: Option<&str>,
    role_id: Option<Uuid>,
) -> Result<User, sqlx::Error> {
    let row = sqlx::query(
        "INSERT INTO users (organization_id, email, password_hash, full_name, display_name, role_id)
            VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(organization_id)
    .bind(email)
    .bind(password_hash)
    .bind(full_name)
    .bind(display_name)
    .bind(role_id)
    .fetch_one(pool)
    .await?;
    Ok(User {
        id: row.get("id"),
        organization_id: row.get("organization_id"),
        email: row.get("email"),
        full_name: row.get("full_name"),
        display_name: row.get("display_name"),
        password_hash: row.get("password_hash"),
        role_id: row.get("role_id"),
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
    role_id: Option<Uuid>,
) -> Result<User, sqlx::Error> {
    let row = sqlx::query(
        "UPDATE users SET email=$1, password_hash=$2, full_name=$3, display_name=$4, role_id=$5 WHERE id = $6 RETURNING *"
    )
    .bind(email)
    .bind(password_hash)
    .bind(full_name)
    .bind(display_name)
    .bind(role_id)
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
        role_id: row.get("role_id"),
        created_at: row.get("created_at"),
    })
}

pub(crate) async fn update_password(
    pool: &mut PgConnection,
    id: Uuid,
    password_hash: &str,
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query(
        "UPDATE users SET password_hash=$1 WHERE id = $2"
    )
    .bind(password_hash)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn delete(pool: &mut PgConnection, id: Uuid) -> Result<u64, ApiError> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}

pub(crate) async fn check_security_admin_remains(pool: &mut PgConnection, org_id: Uuid, i18n: &LocaleContext<'_>) -> Result<(), ApiError> {
    let admin_count = sqlx::query(r#"SELECT Count(u.id) FROM users u
            JOIN roles r ON u.role_id = r.id
            JOIN role_privileges rp ON r.id = rp.role_id
            WHERE rp.privilege_id = $1
            AND u.organization_id = $2
        "#)
        .bind(SystemPrivilege::security_admin)
        .bind(org_id)
        .fetch_one(pool)
        .await
        .map(|row| row.get::<i64, &str>("count"));
    match admin_count {
        Ok(count) if count == 0 => {
            Err(ApiError::Forbidden(i18n.t("security-error-no-admin")))
        }
        Ok(_) => Ok(()),
        Err(e) => Err(ApiError::Db(e))
    }

}

const SQL: &'static str = r#"SELECT u.id, u.organization_id, u.email, u.password_hash, u.created_at as user_created_at,
       u.full_name, u.display_name, u.role_id , o.name as organisation_name, o.strict_audit_mode,
       r.id as role_id, r.organization_id as role_org, r.name as role_name, r.created_at as role_created_at
        FROM users u
            JOIN organizations o ON u.organization_id = o.id
            LEFT JOIN roles r ON u.role_id = r.id
       "#;

pub(crate) async fn get(
    pool: &mut PgConnection,
    id: Uuid,
) -> Result<Option<UserWithOrg>, sqlx::Error> {
    sqlx::query(format!("{} {} ", SQL, "WHERE u.id = $1").as_str())
        .bind(id)
        .fetch_optional(pool)
        .await
        .map(|row| row.map(|r| from_row_to_user_with_org(&r)))
}

pub(crate) async fn get_by_email(
    pool: &mut PgConnection,
    email: &str,
) -> Result<Option<UserWithOrg>, sqlx::Error> {
    sqlx::query(format!("{} {} ", SQL, "WHERE u.email = $1").as_str())
        .bind(email)
        .fetch_optional(pool)
        .await
        .map(|row| row.map(|r| from_row_to_user_with_org(&r)))
}

pub(crate) async fn get_all(
    pool: &mut PgConnection,
    organization_id: Uuid,
) -> Result<Vec<UserWithOrg>, sqlx::Error> {
    sqlx::query(format!("{} {} ", SQL, "WHERE u.organization_id = $1").as_str())
        .bind(organization_id)
        .fetch_all(pool)
        .await
        .map(|rows| rows.iter().map(from_row_to_user_with_org).collect())
}
