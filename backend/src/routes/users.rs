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
use rocket::{delete, get, put, routes, Route};
use rocket_db_pools::Connection;
use serde::Deserialize;
use shared_core::dtos::user_detail::UserDetail;

#[derive(Deserialize)]
pub struct UserUpdateData {
    email: String,
    full_name: String,
    display_name: Option<String>,
}

#[derive(Deserialize)]
pub struct PasswordUpdateData {
    old_password: String,
    new_password: String,
}

pub(crate) fn routes() -> Vec<Route> {
    routes![
        update_me,
        update_password,
        get_all_users,
        get_user,
        delete_user
    ]
}

#[put("/api/users/me", data = "<update_data>")]
pub(crate) async fn update_me(
    mut pool: Connection<DbKelpie>,
    auth_user: AuthenticatedUser,
    update_data: Json<UserUpdateData>,
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
    )
    .await?;

    let user_detail = UserDetail {
        id: updated_user.id,
        email: updated_user.email,
        full_name: updated_user.full_name,
        display_name: updated_user.display_name,
        role: auth_user.role.name,
        organization_id: auth_user.organization_id,
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

    user::update(
        &mut *pool,
        auth_user.user_id,
        &original_user.email,
        &new_password_hash,
        &original_user.full_name,
        original_user.display_name.as_deref(),
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
            role: "User".to_string(), // Placeholder
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
                role: "User".to_string(), // Placeholder
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
) -> Result<&'static str, ApiError> {
    user::delete(&mut *pool, *id).await?;
    Ok("OK")
}
