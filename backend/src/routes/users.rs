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

/// A newtype wrapper for `Uuid` to implement `FromParam` and satisfy the orphan rule.
#[derive(Clone, Copy)]
pub struct PathUuid(Uuid);

/// Allows `PathUuid` to be used as a `Uuid` via dereferencing.
impl Deref for PathUuid {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Synchronous implementation of `FromParam` for our `PathUuid` newtype.
impl<'r> FromParam<'r> for PathUuid {
    type Error = uuid::Error;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        Uuid::parse_str(param).map(PathUuid)
    }
}

// This is a placeholder. You'll need to implement a way to get the
// organization_id from the authenticated user.
fn get_org_id_from_auth() -> Uuid {
    // In a real application, you would get this from the user's session or token.
    // For now, we'll use a placeholder.
    Uuid::new_v4()
}

pub(crate) fn routes() -> Vec<Route> {
    routes![list, add, update, delete, get]
}

#[get("/api/users")]
pub(crate) async fn list(mut pool: Connection<DbKelpie>) -> Result<Json<Vec<User>>, ApiError> {
    let org_id = get_org_id_from_auth();
    let users = user::get_all(&mut *pool, org_id).await?;
    Ok(Json(users))
}

#[post("/api/users", data = "<user>")]
pub(crate) async fn add(user: Json<User>, mut pool: Connection<DbKelpie>) -> Result<Json<User>, ApiError> {
    let org_id = get_org_id_from_auth();
    // In a real application, you would hash the password before storing it.
    let new = user::insert(&mut *pool, org_id, user.email.clone(), user.password_hash.clone()).await?;
    Ok(Json(new))
}

#[put("/api/users", data = "<user>")]
pub(crate) async fn update(user: Json<User>, mut pool: Connection<DbKelpie>) -> Result<Json<User>, ApiError> {
    let updated_user = user::update(&mut *pool, user.id, user.email.clone(), user.password_hash.clone()).await?;
    Ok(Json(updated_user))
}

#[delete("/api/users/<id>")]
pub(crate) async fn delete(id: PathUuid, mut pool: Connection<DbKelpie>) -> Result<&'static str, ApiError> {
    // We use `*id` to dereference PathUuid to Uuid
    user::delete(&mut *pool, *id).await?;
    Ok("OK")
}

#[get("/api/users/<id>")]
pub(crate) async fn get(id: PathUuid, mut pool: Connection<DbKelpie>) -> Result<Json<User>, ApiError> {
    // We use `*id` to dereference PathUuid to Uuid
    match user::get(&mut *pool, *id).await? {
        Some(user) => Ok(Json(user)),
        None => Err(ApiError::NotFound("User not found".to_string())),
    }
}
