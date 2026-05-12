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

use crate::db;
use crate::routes::security::AuthenticatedUser;
use crate::DbKelpie;
use rocket::{get, put};
use rocket::serde::json::Json;
use rocket::Route;
use rocket_db_pools::Connection;
use shared_core::dtos::lock_date_request::LockDateRequest;
use shared_core::dtos::organization::OrganizationDto;
use uuid::Uuid;
use crate::util::types::PathUuid;

pub fn routes() -> Vec<Route> {
    rocket::routes![get_organization, set_lock_date]
}

#[get("/api/organization")]
async fn get_organization(
    mut db: Connection<DbKelpie>,
    user: AuthenticatedUser,
) -> Result<Json<OrganizationDto>, rocket::http::Status> {
    match db::organization::get(&mut db, user.organization_id).await {
        Ok(Some(org)) => Ok(Json(OrganizationDto {
            id: org.id,
            name: org.name,
            strict_audit_mode: org.strict_audit_mode,
            created_at: org.created_at,
            locked_until: org.locked_until,
        })),
        Ok(None) => Err(rocket::http::Status::NotFound),
        Err(_) => Err(rocket::http::Status::InternalServerError),
    }
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
