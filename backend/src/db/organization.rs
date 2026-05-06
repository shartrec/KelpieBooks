use chrono::NaiveDate;
use rocket_db_pools::sqlx::{self, PgConnection, Row};
use uuid::Uuid;
use shared_core::models::Organization;

pub(crate) async fn get(
    pool: &mut PgConnection,
    id: Uuid,
) -> Result<Option<Organization>, sqlx::Error> {
    sqlx::query("SELECT * FROM organizations WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map(|row| row.map(|r| from_row_to_org(&r)))
}

pub(crate) async fn set_locked_until(
    pool: &mut PgConnection,
    id: Uuid,
    date: NaiveDate,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE organizations SET locked_until = $1 WHERE id = $2")
        .bind(date)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
fn from_row_to_org(row: &sqlx::postgres::PgRow) -> Organization {
    Organization {
        id: row.get("id"),
        name: row.get("name"),
        strict_audit_mode: row.get("strict_audit_mode"),
        created_at: row.get("created_at"),
        locked_until: row.get("locked_until"),
    }
}

pub async fn create(tx: &mut PgConnection, name: &str) -> Result<Organization, sqlx::Error> {
    let row = sqlx::query("INSERT INTO organizations (name) VALUES ($1) RETURNING *")
        .bind(name)
        .fetch_one(tx)
        .await?;
    Ok(from_row_to_org(&row))
}
