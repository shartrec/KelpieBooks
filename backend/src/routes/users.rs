/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::db::user;
use crate::routes::security::{hash_pwd, AuthenticatedUser};
use crate::util::types::PathUuid;
use crate::util::ApiError;
use crate::DbKelpie;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, routes, Route};
use rocket_db_pools::Connection;
use serde::Deserialize;
use shared_core::dtos::user_detail::UserDetail;
use shared_core::requests::user::{CreateUserRequest, UpdateUserRequest};
use sqlx::Acquire;
use crate::util::locale_context::LocaleContext;

#[derive(Deserialize)]
pub struct PasswordUpdateData {
    old_password: String,
    new_password: String,
}

pub(crate) fn routes() -> Vec<Route> {
    routes![
        add_user,
        update_user,
        update_me,
        update_password,
        get_all_users,
        get_user,
        delete_user
    ]
}

#[post("/api/users", data = "<create_data>")]
pub(crate) async fn add_user(
    mut pool: Connection<DbKelpie>,
    auth_user: AuthenticatedUser,
    create_data: Json<CreateUserRequest>,
) -> Result<Json<UserDetail>, ApiError> {
    let password_hash = hash_pwd(&create_data.password)?;

    let new_user = user::insert(
        &mut *pool,
        auth_user.organization_id,
        &create_data.email,
        &password_hash,
        &create_data.full_name,
        create_data.display_name.as_deref(),
        create_data.role_id,
    )
    .await?;

    let user_with_org = user::get(&mut *pool, new_user.id).await?.unwrap();

    let user_detail = UserDetail {
        id: user_with_org.id,
        email: user_with_org.email,
        full_name: user_with_org.full_name,
        display_name: user_with_org.display_name,
        role: user_with_org.role.map(|r| r.name),
        organization_id: user_with_org.organization_id,
    };

    Ok(Json(user_detail))
}

#[put("/api/users/<id>", data = "<update_data>")]
pub(crate) async fn update_user(
    id: PathUuid,
    mut pool: Connection<DbKelpie>,
    auth_user: AuthenticatedUser,
    update_data: Json<UpdateUserRequest>,
) -> Result<Json<UserDetail>, ApiError> {

    let i18n = LocaleContext::new(&auth_user.locale);

    let original_user = user::get(&mut *pool, *id)
        .await?
        .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;

    let mut tx = pool.begin().await?;

    let updated_user = user::update(
        &mut *tx,
        *id,
        &update_data.email,
        &original_user.password_hash,
        &update_data.full_name,
        update_data.display_name.as_deref(),
        update_data.role_id,
    )
    .await?;

    // Check we haven't accidentally deleted our admin
    let _ = crate::db::user::check_security_admin_remains(&mut tx, auth_user.organization_id, &i18n).await?;

    tx.commit().await?;

    let user_with_org = user::get(&mut *pool, updated_user.id).await?.unwrap();

    let user_detail = UserDetail {
        id: user_with_org.id,
        email: user_with_org.email,
        full_name: user_with_org.full_name,
        display_name: user_with_org.display_name,
        role: user_with_org.role.map(|r| r.name),
        organization_id: user_with_org.organization_id,
    };

    Ok(Json(user_detail))
}

#[put("/api/users/me", data = "<update_data>")]
pub(crate) async fn update_me(
    mut pool: Connection<DbKelpie>,
    auth_user: AuthenticatedUser,
    update_data: Json<UpdateUserRequest>,
) -> Result<Json<UserDetail>, ApiError> {
    let original_user = user::get(&mut *pool, auth_user.user_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;

    let updated_user = user::update(
        &mut *pool,
        auth_user.user_id,
        &update_data.email,
        &original_user.password_hash,
        &update_data.full_name,
        update_data.display_name.as_deref(),
        // A user cannot update his own role.
        original_user.role.map(|r| r.id),
    )
    .await?;

    let user_with_org = user::get(&mut *pool, updated_user.id).await?.unwrap();

    let user_detail = UserDetail {
        id: user_with_org.id,
        email: user_with_org.email,
        full_name: user_with_org.full_name,
        display_name: user_with_org.display_name,
        role: user_with_org.role.map(|r| r.name),
        organization_id: user_with_org.organization_id,
    };

    Ok(Json(user_detail))
}

#[put("/api/users/me/password", data = "<password_data>")]
pub(crate) async fn update_password(
    mut pool: Connection<DbKelpie>,
    auth_user: AuthenticatedUser,
    password_data: Json<PasswordUpdateData>,
) -> Result<&'static str, ApiError> {
    let original_user = user::get(&mut *pool, auth_user.user_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;

    let valid = bcrypt::verify(&password_data.old_password, &original_user.password_hash)?;
    if !valid {
        return Err(ApiError::Invalid("Incorrect old password".to_string()));
    }

    let new_password_hash = hash_pwd(&password_data.new_password)?;

    user::update_password(
        &mut *pool,
        auth_user.user_id,
        &new_password_hash,
    )
    .await?;

    Ok("Password updated successfully")
}

#[get("/api/users")]
pub(crate) async fn get_all_users(
    mut pool: Connection<DbKelpie>,
    auth_user: AuthenticatedUser,
) -> Result<Json<Vec<UserDetail>>, ApiError> {
    let users = user::get_all(&mut *pool, auth_user.organization_id).await?;
    let user_details = users
        .into_iter()
        .map(|user| UserDetail {
            id: user.id,
            email: user.email,
            full_name: user.full_name,
            display_name: user.display_name,
            role: user.role.map(|r| r.name),
            organization_id: user.organization_id,
        })
        .collect();
    Ok(Json(user_details))
}

#[get("/api/users/<id>")]
pub(crate) async fn get_user(
    id: PathUuid,
    mut pool: Connection<DbKelpie>,
) -> Result<Json<UserDetail>, ApiError> {
    match user::get(&mut *pool, *id).await? {
        Some(user) => {
            let user_detail = UserDetail {
                id: user.id,
                email: user.email,
                full_name: user.full_name,
                display_name: user.display_name,
                role: user.role.map(|r| r.name),
                organization_id: user.organization_id,
            };
            Ok(Json(user_detail))
        }
        None => Err(ApiError::NotFound("User not found".to_string())),
    }
}

#[delete("/api/users/<id>")]
pub(crate) async fn delete_user(
    id: PathUuid,
    mut pool: Connection<DbKelpie>,
    auth_user: AuthenticatedUser,
) -> Result<&'static str, ApiError> {

    let i18n = LocaleContext::new(&auth_user.locale);

    let mut tx = pool.begin().await?;

    user::delete(&mut *tx, *id).await?;
    // You can't delete the last administrator.
    user::check_security_admin_remains(&mut *tx, auth_user.organization_id, &i18n).await?;

    let _ = tx.commit().await;
    Ok("OK")
}
