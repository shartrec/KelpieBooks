/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use shared_core::core::models::{
    auth::SystemPrivilege,
    role::Role,
};
use sqlx::{
    PgConnection,
    Result,
    Row,
};
use uuid::Uuid;

pub(crate) async fn find_all_for_org(conn: &mut PgConnection, org_id: Uuid) -> Result<Vec<Role>> {
    let rows = sqlx::query(
        r#"
        SELECT r.id, r.name, COALESCE(array_agg(rp.privilege_id) FILTER (WHERE rp.privilege_id IS NOT NULL), '{}') as privileges
        FROM roles r
        LEFT JOIN role_privileges rp ON r.id = rp.role_id
        WHERE r.organization_id = $1
        GROUP BY r.id, r.name
        ORDER BY r.name
        "#,
    )
    .bind(org_id)
    .fetch_all(conn)
    .await?;

    let roles = rows
        .into_iter()
        .map(|row| {
            let privileges: Vec<SystemPrivilege> = row.get("privileges");
            Role {
                id: row.get("id"),
                name: row.get("name"),
                privileges: privileges,
            }
        })
        .collect();

    Ok(roles)
}

pub(crate) async fn find_by_id(
    conn: &mut PgConnection,
    org_id: Uuid,
    role_id: Uuid,
) -> Result<Option<Role>> {
    let row = sqlx::query(
        r#"
        SELECT r.id, r.name, COALESCE(array_agg(rp.privilege_id) FILTER (WHERE rp.privilege_id IS NOT NULL), '{}') as privileges
        FROM roles r
        LEFT JOIN role_privileges rp ON r.id = rp.role_id
        WHERE r.organization_id = $1 AND r.id = $2
        GROUP BY r.id
        "#,
    )
    .bind(org_id)
    .bind(role_id)
    .fetch_optional(conn)
    .await?;

    Ok(row.map(|row| {
        let privileges: Vec<SystemPrivilege> = row.get("privileges");
        Role {
            id: row.get("id"),
            name: row.get("name"),
            privileges: privileges,
        }
    }))
}

pub(crate) async fn create(conn: &mut PgConnection, org_id: Uuid, name: &str) -> Result<Uuid> {
    let role_id =
        sqlx::query("INSERT INTO roles (organization_id, name) VALUES ($1, $2) RETURNING id")
            .bind(org_id)
            .bind(name)
            .fetch_one(&mut *conn)
            .await?
            .get("id");
    Ok(role_id)
}
pub(crate) async fn update(conn: &mut PgConnection, role_id: Uuid, name: &str) -> Result<()> {
    let _ = sqlx::query("UPDATE roles SET name = $1 WHERE id = $2")
        .bind(name)
        .bind(role_id)
        .execute(&mut *conn)
        .await?;
    Ok(())
}

pub(crate) async fn delete(conn: &mut PgConnection, org_id: Uuid, role_id: Uuid) -> Result<u64> {
    let result = sqlx::query("DELETE FROM roles WHERE id = $1 AND organization_id = $2")
        .bind(role_id)
        .bind(org_id)
        .execute(conn)
        .await?;

    Ok(result.rows_affected())
}

pub(crate) async fn add_privileges(
    conn: &mut PgConnection,
    role_id: Uuid,
    privileges: Vec<SystemPrivilege>,
) -> Result<()> {
    for privilege in privileges {
        sqlx::query(
            r#"
            INSERT INTO role_privileges (role_id, privilege_id)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING
            "#,
        )
        .bind(role_id)
        .bind(privilege)
        .execute(&mut *conn)
        .await?;
    }
    Ok(())
}

pub(crate) async fn remove_privilege(
    conn: &mut PgConnection,
    role_id: Uuid,
    privileges: Vec<SystemPrivilege>,
) -> Result<()> {
    for privilege in privileges {
        sqlx::query("DELETE FROM role_privileges WHERE role_id =  $1 and privilege_id = $2")
            .bind(role_id)
            .bind(privilege)
            .execute(&mut *conn)
            .await?;
    }
    Ok(())
}

pub(crate) async fn clear_privileges(conn: &mut PgConnection, role_id: Uuid) -> Result<()> {
    sqlx::query("DELETE FROM role_privileges WHERE role_id =  $1")
        .bind(role_id)
        .execute(&mut *conn)
        .await?;
    Ok(())
}
