/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use sqlx::{
    PgConnection,
    Row,
};
use strum::Display;
use uuid::Uuid;

use crate::util::ApiError;

#[derive(Display)]
#[strum(serialize_all = "snake_case")]
pub enum SeqType {
    SalesInvoice,
}

/// Safely increments and formats the next sequential invoice number for an organization without gaps.
pub(crate) async fn get_next_invoice_number(
    conn: &mut PgConnection,
    org_id: Uuid,
    key: &SeqType,
) -> Result<String, ApiError> {
    // 1. Lock ONLY this organization's invoice counter row to prevent race conditions
    let row = sqlx::query(
        r#"
        SELECT prefix, next_value
        FROM organization_sequences
        WHERE org_id = $1 AND document_type = $2
        FOR UPDATE
        "#,
    )
    .bind(org_id)
    .bind(key.to_string())
    .fetch_optional(&mut *conn)
    .await?;

    // 2. Default fallback if the tenant hasn't configured a custom range yet
    let (prefix, current_val): (String, i32) = match row {
        Some(r) => {
            let p: String = r.get("prefix");
            let n: i32 = r.get("next_value");
            (p, n)
        }
        None => {
            let row = sqlx::query(
                r#"INSERT INTO organization_sequences
                    (org_id, document_type, next_value) VALUES ($1, $2, 1000) RETURNING prefix, next_value"#)
                .bind(org_id)
                .bind(key.to_string())
                .fetch_one(&mut *conn).await?;
            let p: String = row.get("prefix");
            let n: i32 = row.get("next_value");
            (p, n)
        }
    };

    // 3. Increment the counter state for the next generation request
    let rows = sqlx::query(
        r#"
        UPDATE organization_sequences
        SET next_value = next_value + 1
        WHERE org_id = $1 AND document_type = $2
        "#,
    )
    .bind(org_id)
    .bind(key.to_string())
    .execute(&mut *conn)
    .await?;

    if rows.rows_affected() != 1 {
        return Err(ApiError::Internal(format!(
            "Next {} number generation failed",
            key.to_string()
        )));
    }
    // 4. Return a perfectly padded sequential tracking string (e.g., "INV-1000")
    Ok(format!("{}{}", prefix, current_val))
}
