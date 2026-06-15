/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

// backend/src/services/onboarding.rs

use shared_core::core::requests::onboard::OnboardingRequest;
use sqlx::{
    Acquire,
    PgConnection,
    Postgres,
    Transaction,
};
use shared_core::core::models::{
    auth::SystemPrivilege,
    user::User,
};
use crate::core::db::{
    organization as db_org,
    roles as db_role,
    user as db_user,
};
use crate::core::routes::security::hash_pwd;
use crate::util::ApiError;

pub(crate) async fn bootstrap_tenant_organization(
    pool: &mut PgConnection,
    req: &OnboardingRequest,
) -> Result<User, ApiError> {
    // 1. Begin the atomic transaction isolation guard
    let mut tx: Transaction<'_, Postgres> = pool.begin().await?;

    // 2. Insert the target corporate workspace cluster
    let org = db_org::create(&mut tx, &req.organization_name).await?;

    // 3. Setup the initial omnipotent master identity record role ('org_admin')
    let role_id = db_role::create(&mut tx, org.id, "Administrator").await?;

    // 4. Collect ALL structural enum keys to populate the master matrix array
    let master_privileges: Vec<SystemPrivilege> = SystemPrivilege::iterator().collect();

    db_role::add_privileges(&mut tx, role_id, master_privileges).await?;

    // 6. Build the initial profile entity record and anchor it to the org_admin role
    let password_hash = hash_pwd(&req.user_password)?;
    let user = db_user::insert(
        &mut tx,
        org.id,
        &req.user_email,
        &password_hash,
        &req.user_full_name,
        req.user_display_name.as_deref(),
        Some(role_id),
    )
    .await?;

    // 7. Everything verified successfully - flush permanently to disk ledger storage
    tx.commit().await?;

    Ok(user)
}
