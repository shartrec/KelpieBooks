/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use chrono::NaiveDate;
use rocket_db_pools::Connection;
use shared_core::sales::models::tax::TaxRate;
use shared_core::{OrgId, TaxCategoryId};
use crate::{
    sales::db::tax_rate,
    util::ApiError,
    DbKelpie,
};

pub async fn get_tax_rates_for_category(
    pool: &mut Connection<DbKelpie>,
    org_id: OrgId,
    category_id: TaxCategoryId,
) -> Result<Vec<TaxRate>, ApiError> {
    let rates = tax_rate::get_tax_rates_for_category(pool, org_id, category_id).await?;
    Ok(rates)
}

pub async fn get_current_tax_rate_for_category(
    pool: &mut Connection<DbKelpie>,
    org_id: OrgId,
    category_id: TaxCategoryId,
    effective_date: NaiveDate,
) -> Result<Option<TaxRate>, ApiError> {
    let rate = tax_rate::get_current_tax_rate_for_category(
        pool,
        org_id,
        category_id,
        effective_date,
    )
    .await?;
    Ok(rate)
}

pub async fn update_tax_rates_for_category(
    pool: &mut Connection<DbKelpie>,
    org_id: OrgId,
    category_id: TaxCategoryId,
    rates: &[TaxRate],
) -> Result<(), ApiError> {
    tax_rate::update_tax_rates_for_category(pool, org_id, category_id, rates).await?;
    Ok(())
}
