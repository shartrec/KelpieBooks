/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use chrono::NaiveDate;
use rocket_db_pools::sqlx;
use shared_core::sales::models::tax::TaxRate;
use sqlx::{
    Acquire,
    PgConnection,
};
use uuid::Uuid;

pub async fn get_tax_rates_for_category(
    conn: &mut PgConnection,
    category_id: Uuid,
    organization_id: Uuid,
) -> Result<Vec<TaxRate>, sqlx::Error> {
    let rows = sqlx::query_as(
        r#"
        SELECT id, organization_id, tax_category_id, name, rate, liability_account_id, valid_from, valid_to
        FROM tax_rates
        WHERE tax_category_id = $1 AND organization_id = $2
        ORDER BY valid_from DESC
        "#)
        .bind(category_id)
        .bind(organization_id)
        .fetch_all(conn)
        .await;
    rows
}

pub async fn get_current_tax_rate_for_category(
    conn: &mut PgConnection,
    category_id: Uuid,
    organization_id: Uuid,
    effective_date: NaiveDate,
) -> Result<Option<TaxRate>, sqlx::Error> {
    let row = sqlx::query_as(
        r#"
        SELECT id, organization_id, tax_category_id, name, rate, liability_account_id, valid_from, valid_to
        FROM tax_rates
        WHERE tax_category_id = $1
          AND organization_id = $2
          AND valid_from <= $3
          AND (valid_to IS NULL OR valid_to >= $3)
        ORDER BY valid_from DESC
        LIMIT 1
        "#)
        .bind(category_id)
        .bind(organization_id)
        .bind(effective_date)
        .fetch_optional(conn)
        .await;
    row
}

pub async fn update_tax_rates_for_category(
    conn: &mut PgConnection,
    category_id: Uuid,
    organization_id: Uuid,
    rates: &[TaxRate],
) -> Result<(), sqlx::Error> {
    let mut tx = conn.begin().await?;

    // First, delete all existing rates for this category
    sqlx::query("DELETE FROM tax_rates WHERE tax_category_id = $1 AND organization_id = $2")
        .bind(category_id)
        .bind(organization_id)
        .execute(&mut *tx)
        .await?;

    // Then, insert the new rates
    for rate in rates {
        sqlx::query(
            r#"
            INSERT INTO tax_rates (id, organization_id, tax_category_id, name, rate, liability_account_id, valid_from, valid_to)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#)
            .bind(rate.id)
            .bind(organization_id)
            .bind(category_id)
            .bind(rate.name.clone())
            .bind(rate.rate)
            .bind(rate.liability_account_id)
            .bind(rate.valid_from)
            .bind(rate.valid_to)
           .execute(&mut *tx)
            .await?;
    }

    tx.commit().await
}
