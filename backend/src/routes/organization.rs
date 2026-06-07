/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::db;
use crate::routes::security::AuthenticatedUser;
use crate::security::{ManageOrganization, RequirePrivilege};
use crate::util::types::PathUuid;
use crate::util::ApiError;
use crate::DbKelpie;
use rocket::serde::json::Json;
use rocket::{get, put, Route};
use rocket_db_pools::Connection;
use shared_core::dtos::organization::{AuditModeRequest, LockDateRequest};
use shared_core::models::organization::Organization;

pub(crate) fn routes() -> Vec<Route> {
    rocket::routes![get_organization, set_lock_date, set_audit_mode]
}

#[get("/api/organization")]
pub(crate) async fn get_organization(
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
    guard: RequirePrivilege<ManageOrganization>,
    id: PathUuid,
    req: Json<LockDateRequest>,
) -> rocket::http::Status {
    let user = guard.0;
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
    guard: RequirePrivilege<ManageOrganization>,
    id: PathUuid,
    req: Json<AuditModeRequest>,
) -> rocket::http::Status {
    let user = guard.0;
    if *id != user.organization_id {
        return rocket::http::Status::Forbidden;
    }

    match db::organization::set_audit_mode(&mut db, *id, req.strict_audit_mode).await {
        Ok(_) => rocket::http::Status::Ok,
        Err(_) => rocket::http::Status::InternalServerError,
    }
}
