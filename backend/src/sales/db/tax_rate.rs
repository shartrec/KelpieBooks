/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use chrono::NaiveDate;
use rocket_db_pools::sqlx;
use shared_core::{
    sales::models::tax::TaxRate,
    OrgId,
    TaxCategoryId,
};
use sqlx::{
    Acquire,
    PgConnection,
};

pub async fn get_tax_rates_for_category(
    conn: &mut PgConnection,
    org_id: OrgId,
    category_id: TaxCategoryId,
) -> Result<Vec<TaxRate>, sqlx::Error> {
    let rows = sqlx::query_as!(
        TaxRate,
        r#"
        SELECT id, organization_id as org_id, tax_category_id, name, rate, liability_account_id, valid_from, valid_to
        FROM tax_rates
        WHERE tax_category_id = $1 AND organization_id = $2
        ORDER BY valid_from DESC
        "#,
        *category_id,
        *org_id,
    )
        .fetch_all(conn)
        .await;
    rows
}

pub async fn get_current_tax_rate_for_category(
    conn: &mut PgConnection,
    org_id: OrgId,
    category_id: TaxCategoryId,
    effective_date: NaiveDate,
) -> Result<Option<TaxRate>, sqlx::Error> {
    let row = sqlx::query_as!(
        TaxRate,
        r#"
        SELECT id, organization_id as org_id, tax_category_id, name, rate, liability_account_id, valid_from, valid_to
        FROM tax_rates
        WHERE tax_category_id = $1
          AND organization_id = $2
          AND valid_from <= $3
          AND (valid_to IS NULL OR valid_to >= $3)
        ORDER BY valid_from DESC
        LIMIT 1
        "#,
        *category_id,
        *org_id,
        effective_date,
    )
        .fetch_optional(conn)
        .await;
    row
}

pub async fn update_tax_rates_for_category(
    conn: &mut PgConnection,
    org_id: OrgId,
    category_id: TaxCategoryId,
    rates: &[TaxRate],
) -> Result<(), sqlx::Error> {
    let mut tx = conn.begin().await?;

    // First, delete all existing rates for this category
    sqlx::query!(
        "DELETE FROM tax_rates WHERE tax_category_id = $1 AND organization_id = $2",
        *category_id,
        *org_id,
    )
    .execute(&mut *tx)
    .await?;

    // Then, insert the new rates
    for rate in rates {
        sqlx::query!(
            r#"
            INSERT INTO tax_rates (id, organization_id, tax_category_id, name, rate, liability_account_id, valid_from, valid_to)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            *rate.id,
            *org_id,
            *category_id,
            rate.name,
            rate.rate,
            *rate.liability_account_id,
            rate.valid_from,
            rate.valid_to,
        )
           .execute(&mut *tx)
            .await?;
    }

    tx.commit().await
}
