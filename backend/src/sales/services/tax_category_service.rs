/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use rocket_db_pools::Connection;
use shared_core::sales::models::tax::TaxCategory;
use uuid::Uuid;

use crate::{
    sales::db::tax_category as tax_db,
    DbKelpie,
};

pub async fn get_tax_categories(
    pool: &mut Connection<DbKelpie>,
    org_id: Uuid,
) -> Result<Vec<TaxCategory>, sqlx::Error> {
    tax_db::all(pool, org_id).await
}

pub async fn get_tax_category(
    pool: &mut Connection<DbKelpie>,
    org_id: Uuid,
    id: Uuid,
) -> Result<Option<TaxCategory>, sqlx::Error> {
    tax_db::get(pool, org_id, id).await
}

pub async fn create_tax_category(
    pool: &mut Connection<DbKelpie>,
    org_id: Uuid,
    tax_category: &TaxCategory,
) -> Result<TaxCategory, sqlx::Error> {
    tax_db::create(pool, org_id, tax_category).await
}

pub async fn update_tax_category(
    pool: &mut Connection<DbKelpie>,
    org_id: Uuid,
    id: Uuid,
    tax_category: &TaxCategory,
) -> Result<TaxCategory, sqlx::Error> {
    tax_db::update(pool, org_id, id, tax_category).await
}

pub async fn delete_tax_category(
    pool: &mut Connection<DbKelpie>,
    org_id: Uuid,
    id: Uuid,
) -> Result<u64, sqlx::Error> {
    tax_db::delete(pool, org_id, id).await.map(|n| n)
}
