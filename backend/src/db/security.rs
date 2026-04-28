use rocket_db_pools::sqlx::{self, PgConnection, Row};
use crate::db::user;
use shared_core::models::User;
use crate::util::ApiError;

// A temporary struct to hold the joined user and organization data
pub struct UserWithOrg {
    pub id: uuid::Uuid,
    pub organization_id: uuid::Uuid,
    pub email: String,
    pub full_name: String,
    pub display_name: Option<String>,
    pub password_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub strict_audit_mode: bool,
}

pub async fn check_login(
    pool: &mut PgConnection,
    email: &str,
    password_raw: &str,
) -> Result<Option<UserWithOrg>, ApiError> {
    let user_with_org = sqlx::query_as!(
        UserWithOrg,
        r#"
        SELECT
            u.id,
            u.organization_id,
            u.email,
            u.full_name,
            u.display_name,
            u.password_hash,
            u.created_at,
            o.strict_audit_mode
        FROM users u
        JOIN organizations o ON u.organization_id = o.id
        WHERE u.email = $1
        "#,
        email
    )
    .fetch_optional(pool)
    .await?;

    if let Some(user) = user_with_org {
        if bcrypt::verify(password_raw, &user.password_hash)? {
            return Ok(Some(user));
        }
    }
    Ok(None)
}

pub async fn create_initial_admin(pool: &mut PgConnection) -> Result<(), ApiError> {
    let count: Option<i64> = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
        .fetch_one(&mut *pool)
        .await?;

    if matches!(count, None | Some(0)) {
        let org = crate::db::organization::create(pool, "Default Organization").await?;
        let password_hash = bcrypt::hash("admin", bcrypt::DEFAULT_COST)?;
        user::insert(
            pool,
            org.id,
            "admin@kelpie.local".to_string(),
            password_hash,
            "Admin User".to_string(),
            Some("Admin".to_string()),
        ).await?;
        log::info!("Created initial admin user with password 'admin'");
    }

    Ok(())
}
