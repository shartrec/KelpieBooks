/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::{
    DateTime,
    Utc,
};
use shared_core::UserId;
use sqlx::{
    PgConnection,
    Row,
};

pub struct PasswordResetToken {
    pub user_id: UserId,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
}

pub async fn save_reset_token(
    pool: &mut PgConnection,
    user_id: UserId,
    token_hash: &str,
    expires_at: DateTime<Utc>,
) -> Result<i32, sqlx::Error> {
    let row = sqlx::query!(
        r#"INSERT INTO password_reset_tokens (user_id, token_hash, expires_at)
            VALUES ($1, $2, $3)
            RETURNING id"#,
        user_id,
        token_hash,
        expires_at
    )
    .fetch_one(pool)
    .await?;
    Ok(row.id)
}

pub async fn find_active_token(
    pool: &mut PgConnection,
    token_id: &i32,
) -> Result<Option<PasswordResetToken>, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT token_hash, user_id, expires_at FROM password_reset_tokens WHERE id = $1 AND used = false AND expires_at > NOW()",
        token_id
    )
        .fetch_optional(pool)
        .await?;
    Ok(row.map(|r| PasswordResetToken {
        user_id: r.user_id,
        token_hash: r.token_hash,
        expires_at: r.expires_at,
    }))
}

pub async fn mark_token_as_used(pool: &mut PgConnection, token_id: i32) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE password_reset_tokens SET used = true WHERE id = $1 AND used = false",
        token_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn delete_expired_reset_tokens(
    pool: &mut PgConnection,
) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM password_reset_tokens WHERE expires_at < CURRENT_TIMESTAMP")
        .execute(pool)
        .await?;
    Ok(())
}
