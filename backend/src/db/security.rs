use rocket_db_pools::sqlx::PgConnection;
use crate::db::user;
use shared_core::models::User;
use crate::util::ApiError;

pub async fn check_login(
    pool: &mut PgConnection,
    email: &str,
    password_raw: &str,
) -> Result<Option<User>, ApiError> {
    if let Some(user) = user::get_by_email(pool, email).await? {
        if bcrypt::verify(password_raw, &user.password_hash)? {
            return Ok(Some(user));
        }
    }
    Ok(None)
}

pub async fn create_initial_admin(pool: &mut PgConnection) -> Result<(), ApiError> {
    // Check if there are any users
    let count = sqlx::query!("SELECT COUNT(*) FROM users")
        .fetch_one(&mut *pool)
        .await?
        .count
        .unwrap_or(0);

    if count == 0 {
        // Create a default organization
        let org = crate::db::organization::create(pool, "Default Organization").await?;

        // Create the admin user
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
