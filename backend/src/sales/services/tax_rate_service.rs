/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use rocket_db_pools::Connection;
use shared_core::sales::models::tax::TaxRate;
use uuid::Uuid;
use crate::DbKelpie;
use crate::sales::db::tax_rate;
use crate::util::ApiError;

pub async fn get_tax_rates_for_category(
    pool: &mut Connection<DbKelpie>,
    category_id: Uuid,
    organization_id: Uuid,
) -> Result<Vec<TaxRate>, ApiError> {
    let rates = tax_rate::get_tax_rates_for_category(pool, category_id, organization_id).await?;
    Ok(rates)
}

pub async fn update_tax_rates_for_category(
    pool: &mut Connection<DbKelpie>,
    category_id: Uuid,
    organization_id: Uuid,
    rates: &[TaxRate],
) -> Result<(), ApiError> {
    tax_rate::update_tax_rates_for_category(pool, category_id, organization_id, rates).await?;
    Ok(())
}