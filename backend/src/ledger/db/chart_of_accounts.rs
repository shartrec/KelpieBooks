/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use rocket_db_pools::sqlx::{self, PgConnection};
use serde::Deserialize;
use shared_core::ledger::models::account_category::AccountCategory;
use shared_core::ledger::models::system_tag::SystemTag;
use sqlx::Row;
use std::collections::HashMap;
use uuid::Uuid;

/// Represents the top-level structure of a TOML template file.
#[derive(Debug, Deserialize)]
pub(crate) struct ChartOfAccountsTemplate {
    pub(crate) accounts: Vec<AccountImport>,
}

/// Represents a single account entry to be imported from a template.
#[derive(Debug, Deserialize)]
pub(crate) struct AccountImport {
    pub(crate) code: String,
    pub(crate) name: String,
    pub(crate) category: AccountCategory,
    pub(crate) parent_code: Option<String>,
    #[serde(default)]
    pub(crate) is_group: bool,
    pub(crate) system_tag: Option<SystemTag>,
}

pub(crate) async fn import_default_accounts(
    tx: &mut PgConnection,
    organization_id: Uuid,
    accounts: Vec<AccountImport>,
) -> Result<(), sqlx::Error> {
    let mut code_to_id_map = HashMap::new();
    let mut remaining_accounts = accounts;

    while !remaining_accounts.is_empty() {
        let (ready_to_import, next_remaining): (Vec<_>, Vec<_>) = remaining_accounts
            .into_iter()
            .partition(|a| match &a.parent_code {
                None => true,
                Some(parent_code) => code_to_id_map.contains_key(parent_code),
            });

        if ready_to_import.is_empty() {
            // This happens if there are missing parents or circular dependencies
            for a in next_remaining {
                log::error!(
                    "Could not import account '{}' (code {}): parent '{}' not found or circular dependency.",
                    a.name,
                    a.code,
                    a.parent_code.unwrap_or_default()
                );
            }
            break;
        }

        for account_template in ready_to_import {
            let parent_id = account_template
                .parent_code
                .as_ref()
                .and_then(|code| code_to_id_map.get(code).cloned());

            let id = insert_account(tx, organization_id, parent_id, &account_template).await?;
            code_to_id_map.insert(account_template.code.clone(), id);
        }

        remaining_accounts = next_remaining;
    }

    Ok(())
}

async fn insert_account(
    tx: &mut PgConnection,
    organization_id: Uuid,
    parent_id: Option<Uuid>,
    template: &AccountImport,
) -> Result<Uuid, sqlx::Error> {
    let row = sqlx::query(
        r#"
        INSERT INTO accounts (organization_id, parent_id, code, name, category, is_group, system_tag)
        VALUES ($1, $2, $3, $4, $5::account_category, $6, $7::system_tag)
        RETURNING id
        "#)
        .bind(organization_id)
        .bind(parent_id)
        .bind(&template.code)
        .bind(&template.name)
        .bind(template.category.to_string())
        .bind(template.is_group)
        .bind(template.system_tag.map(|s| s.to_string()))
        .fetch_one(tx)
        .await?;

    // Retrieve the 'id' from the row and return it.
    Ok(row.get(0))
}
