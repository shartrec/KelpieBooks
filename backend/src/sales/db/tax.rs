/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use rocket_db_pools::sqlx::{self, PgConnection};
use shared_core::sales::models::tax::TaxCategory;

pub(crate) async fn get_active_tax_categories(pool: &mut PgConnection) -> Result<Vec<TaxCategory>, sqlx::Error> {
    sqlx::query_as::<_, TaxCategory>(
        r#"SELECT c.id, c.organization_id, c.name, r.rate, c.is_active FROM tax_categories c, tax_rates r
                 WHERE is_active = true
                 AND c.id = r.tax_category_id
                 AND NOW() BETWEEN r.valid_from AND r.valid_to
                 ORDER BY c.name ASC"#
    )
        .fetch_all(pool)
        .await
}
