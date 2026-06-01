/*
 * Copyright (c) 2025-2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use chrono::{DateTime, Utc};
use rocket_db_pools::sqlx::{self, PgConnection, Row};
use shared_core::models::role::Role;
use shared_core::models::user::User;
use shared_core::models::user_with_org::UserWithOrg;
use uuid::Uuid;

fn from_row_to_user_with_org(row: &sqlx::postgres::PgRow) -> UserWithOrg {
    let role = Role {
        id: row.get("id"),
        name: row.get("name"),
        privileges: vec![],
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
        role_id: row.get("role_id"),
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
    sqlx::query("SELECT u.*, o.name as organisation_name, o.strict_audit_mode, r.name as role_name FROM users u
        JOIN organizations o ON u.organization_id = o.id
        JOIN roles r ON u.role_id = r.id WHERE u.id = $1"
    )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map(|row| row.map(|r| from_row_to_user_with_org(&r)))
}

pub(crate) async fn get_by_email(
    pool: &mut PgConnection,
    email: &str,
) -> Result<Option<UserWithOrg>, sqlx::Error> {
    sqlx::query(r#"SELECT u.id, u.organization_id, u.email, u.password_hash, u.created_at as user_created_at,
       u.full_name, u.display_name, u.role_id , o.name as organisation_name, o.strict_audit_mode,
       r.id, r.organization_id, r.name, r.created_at as role_created_at
        FROM users u
            JOIN organizations o ON u.organization_id = o.id
            JOIN roles r ON u.role_id = r.id
            WHERE u.email = $1
        "#)
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
