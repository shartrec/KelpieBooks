/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use rocket_db_pools::Connection;
use shared_core::{
    sales::models::item::UnitOfMeasure,
    OrgId,
    UomId,
};

use crate::{
    sales::db::{
        item as item_db,
        uom as uom_db,
    },
    util::ApiError,
    DbKelpie,
};

pub async fn get_uoms(
    pool: &mut Connection<DbKelpie>,
    org_id: OrgId,
) -> Result<Vec<UnitOfMeasure>, sqlx::Error> {
    uom_db::all(pool, org_id).await
}

pub async fn get_uom(
    pool: &mut Connection<DbKelpie>,
    org_id: OrgId,
    id: UomId,
) -> Result<Option<UnitOfMeasure>, sqlx::Error> {
    uom_db::get(pool, org_id, id).await
}

pub async fn create_uom(
    pool: &mut Connection<DbKelpie>,
    org_id: OrgId,
    uom: &UnitOfMeasure,
) -> Result<UnitOfMeasure, sqlx::Error> {
    uom_db::create(pool, org_id, uom).await
}

pub async fn update_uom(
    pool: &mut Connection<DbKelpie>,
    org_id: OrgId,
    id: UomId,
    uom: &UnitOfMeasure,
) -> Result<UnitOfMeasure, sqlx::Error> {
    uom_db::update(pool, org_id, id, uom).await
}

pub async fn delete_uom(
    pool: &mut Connection<DbKelpie>,
    org_id: OrgId,
    id: UomId,
) -> Result<u64, ApiError> {
    if item_db::is_uom_in_use(pool, id).await? {
        return Err(ApiError::Conflict(
            "Unit of Measure is in use and cannot be deleted.".to_string(),
        ));
    }

    let result = uom_db::delete(pool, org_id, id).await?;
    Ok(result.rows_affected())
}
