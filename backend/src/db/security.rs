use crate::db::user;
use crate::util::ApiError;
use rocket_db_pools::sqlx::{self, PgConnection};

// A temporary struct to hold the joined user and organization data
pub struct UserWithOrg {
    pub id: uuid::Uuid,
    pub organization_id: uuid::Uuid,
    pub email: String,
    pub full_name: String,
    pub display_name: Option<String>,
    pub password_hash: String,
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
