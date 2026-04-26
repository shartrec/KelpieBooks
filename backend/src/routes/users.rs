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

use crate::db::user;
use crate::util::ApiError;
use crate::DbKelpie;
use rocket::request::FromParam;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, routes, Route};
use shared_core::models::User;
use std::ops::Deref;
use uuid::Uuid;
use rocket_db_pools::Connection;
use crate::routes::security::AuthenticatedUser;
use serde::Deserialize;

/// A newtype wrapper for `Uuid` to implement `FromParam` and satisfy the orphan rule.
#[derive(Clone, Copy)]
pub struct PathUuid(Uuid);

impl Deref for PathUuid {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'r> FromParam<'r> for PathUuid {
    type Error = uuid::Error;
    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        Uuid::parse_str(param).map(PathUuid)
    }
}

#[derive(Deserialize)]
pub struct UserUpdateData {
    full_name: String,
    display_name: Option<String>,
}

pub(crate) fn routes() -> Vec<Route> {
    routes![update_me, get_all_users, get_user, delete_user]
}

#[put("/api/users/me", data = "<update_data>")]
pub(crate) async fn update_me(
    mut pool: Connection<DbKelpie>,
    auth_user: AuthenticatedUser,
    update_data: Json<UserUpdateData>,
) -> Result<Json<User>, ApiError> {
    // We need the original user object to get the password hash
    let original_user = user::get(&mut *pool, auth_user.user_id).await?
        .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;

    let updated_user = user::update(
        &mut *pool,
        auth_user.user_id,
        original_user.email, // email is not changing here
        original_user.password_hash, // password is not changing here
        update_data.full_name.clone(),
        update_data.display_name.clone(),
    ).await?;

    Ok(Json(updated_user))
}


#[get("/api/users")]
pub(crate) async fn get_all_users(
    mut pool: Connection<DbKelpie>,
    auth_user: AuthenticatedUser,
) -> Result<Json<Vec<User>>, ApiError> {
    // In a real app, you'd get the organization_id from the authenticated user
    let users = user::get_all(&mut *pool, auth_user.user_id).await?;
    Ok(Json(users))
}

#[get("/api/users/<id>")]
pub(crate) async fn get_user(id: PathUuid, mut pool: Connection<DbKelpie>) -> Result<Json<User>, ApiError> {
    match user::get(&mut *pool, *id).await? {
        Some(user) => Ok(Json(user)),
        None => Err(ApiError::NotFound("User not found".to_string())),
    }
}

#[delete("/api/users/<id>")]
pub(crate) async fn delete_user(id: PathUuid, mut pool: Connection<DbKelpie>) -> Result<&'static str, ApiError> {
    user::delete(&mut *pool, *id).await?;
    Ok("OK")
}
