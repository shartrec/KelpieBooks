/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::db::roles;
use crate::db::user as db_user;
use crate::routes::security::AuthenticatedUser;
use crate::util::types::PathUuid;
use crate::util::ApiError;
use crate::DbKelpie;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, routes, Route};
use rocket_db_pools::Connection;
use shared_core::models::role::Role;
use shared_core::requests::role::{CreateRoleRequest, UpdateRoleRequest};
use sqlx::Acquire;
use crate::util::locale_context::LocaleContext;

pub(crate) fn routes() -> Vec<Route> {
    routes![get_all_roles, create_role, update_role, delete_role]
}

#[get("/api/roles")]
pub(crate) async fn get_all_roles(
    mut pool: Connection<DbKelpie>,
    auth_user: AuthenticatedUser,
) -> Result<Json<Vec<Role>>, ApiError> {
    let roles = roles::find_all_for_org(&mut *pool, auth_user.organization_id).await?;
    Ok(Json(roles))
}

#[post("/api/roles", data = "<req>")]
pub(crate) async fn create_role(
    mut pool: Connection<DbKelpie>,
    auth_user: AuthenticatedUser,
    req: Json<CreateRoleRequest>,
) -> Result<Json<Role>, ApiError> {

    let mut tx = pool.begin().await?;

    let role_id = roles::create(&mut tx, auth_user.organization_id, &req.name).await?;
    roles::add_privileges(&mut tx, role_id, req.privileges.clone()).await?;

    tx.commit().await?;

    let role = roles::find_by_id(&mut pool, auth_user.organization_id, role_id)
        .await?
        .unwrap();
    Ok(Json(role))
}

#[put("/api/roles/<id>", data = "<req>")]
pub(crate) async fn update_role(
    id: PathUuid,
    mut pool: Connection<DbKelpie>,
    auth_user: AuthenticatedUser,
    req: Json<UpdateRoleRequest>,
) -> Result<Json<Role>, ApiError> {

    let i18n = LocaleContext::new(&auth_user.locale);

    let mut tx = pool.begin().await?;
    let _ = roles::update(&mut tx, *id, &req.name).await?;

    roles::clear_privileges(&mut tx, *id).await?;
    roles::add_privileges(&mut tx, *id, req.privileges.clone()).await?;

    // Check we haven't accidentally deleted our admin
    let _ = db_user::check_security_admin_remains(&mut tx, auth_user.organization_id, &i18n).await?;

    tx.commit().await?;

    let role = roles::find_by_id(&mut pool, auth_user.organization_id, *id)
        .await?
        .unwrap();
    Ok(Json(role))
}

#[delete("/api/roles/<id>")]
pub(crate) async fn delete_role(
    id: PathUuid,
    mut pool: Connection<DbKelpie>,
    auth_user: AuthenticatedUser,
) -> Result<&'static str, ApiError> {

    let i18n = LocaleContext::new(&auth_user.locale);

    let mut tx = pool.begin().await?;

    roles::delete(&mut *tx, auth_user.organization_id, *id).await?;
    // Check we haven't accidentally deleted our admin
    let _ = db_user::check_security_admin_remains(&mut tx, auth_user.organization_id, &i18n).await?;

    tx.commit().await?;

    Ok("OK")
}
