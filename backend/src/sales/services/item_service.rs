/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use rocket_db_pools::Connection;
use shared_core::sales::{
    models::item::{
        Item,
        ItemType,
    },
    requests::item::CreateItemRequest,
};
use uuid::Uuid;

use crate::{
    sales::db::item as item_db,
    util::ApiError,
    DbKelpie,
};

pub async fn get_items(
    pool: &mut Connection<DbKelpie>,
    org_id: Uuid,
    search_term: Option<String>,
    item_type: Option<ItemType>,
    include_inactive: bool,
    limit: u32,
) -> Result<Vec<Item>, sqlx::Error> {
    item_db::all(
        pool,
        org_id,
        search_term,
        item_type,
        include_inactive,
        limit,
    )
    .await
}

pub async fn get_item(
    pool: &mut Connection<DbKelpie>,
    id: Uuid,
    org_id: Uuid,
) -> Result<Option<Item>, sqlx::Error> {
    item_db::get(pool, id, org_id).await
}

pub async fn create_item(
    pool: &mut Connection<DbKelpie>,
    org_id: Uuid,
    item: &CreateItemRequest,
) -> Result<Item, sqlx::Error> {
    item_db::create(pool, org_id, item).await
}

pub async fn update_item(
    pool: &mut Connection<DbKelpie>,
    id: Uuid,
    org_id: Uuid,
    item: &Item,
) -> Result<Item, sqlx::Error> {
    item_db::update(pool, id, org_id, item).await
}

pub async fn delete_item(
    pool: &mut Connection<DbKelpie>,
    id: Uuid,
    org_id: Uuid,
) -> Result<u64, ApiError> {
    let rows_affected = item_db::delete(pool, id, org_id).await?;
    if rows_affected == 0 {
        return Err(ApiError::NotFound("Account not found.".to_string()));
    }
    Ok(rows_affected)
}
