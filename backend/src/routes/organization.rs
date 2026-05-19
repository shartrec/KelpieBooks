/*
 * Copyright (c) 2026. Trevor Campbell and others.
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

use crate::db;
use crate::routes::security::AuthenticatedUser;
use crate::util::types::PathUuid;
use crate::util::ApiError;
use crate::DbKelpie;
use rocket::serde::json::Json;
use rocket::{get, put, Route};
use rocket_db_pools::Connection;
use shared_core::dtos::organization::{AuditModeRequest, LockDateRequest};
use shared_core::models::organization::Organization;

pub fn routes() -> Vec<Route> {
    rocket::routes![get_organization, set_lock_date, set_audit_mode]
}

#[get("/api/organization")]
pub async fn get_organization(
    mut pool: Connection<DbKelpie>,
    user: AuthenticatedUser,
) -> Result<Json<Organization>, ApiError> {
    let org = db::organization::get(&mut *pool, user.organization_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Organization not found".to_string()))?;
    Ok(Json(org))
}

#[put("/api/organizations/<id>/lock", data = "<req>")]
async fn set_lock_date(
    mut db: Connection<DbKelpie>,
    user: AuthenticatedUser,
    id: PathUuid,
    req: Json<LockDateRequest>,
) -> rocket::http::Status {
    if *id != user.organization_id {
        return rocket::http::Status::Forbidden;
    }

    match db::organization::set_lock_date(&mut db, *id, req.locked_until).await {
        Ok(_) => rocket::http::Status::Ok,
        Err(_) => rocket::http::Status::InternalServerError,
    }
}
#[put("/api/organizations/<id>/audit_mode", data = "<req>")]
async fn set_audit_mode(
    mut db: Connection<DbKelpie>,
    user: AuthenticatedUser,
    id: PathUuid,
    req: Json<AuditModeRequest>,
) -> rocket::http::Status {
    if *id != user.organization_id {
        return rocket::http::Status::Forbidden;
    }

    match db::organization::set_audit_mode(&mut db, *id, req.strict_audit_mode).await {
        Ok(_) => rocket::http::Status::Ok,
        Err(_) => rocket::http::Status::InternalServerError,
    }
}
