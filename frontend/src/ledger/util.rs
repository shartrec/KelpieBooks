/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use fluent::fluent_args;
use yew_router::navigator::Navigator;
use shared_core::ledger::models::account::Account;
use shared_core::ledger::models::account_category::AccountCategory;
use crate::api::Api;
use crate::contexts::auth_context::UserContextHandle;
use crate::contexts::locale_context::LocaleContext;

pub(crate) async fn get_accounts_by_category(
    category: AccountCategory,
    user_ctx: UserContextHandle,
    navigator: Navigator,
    i18n: &LocaleContext,
    include_group_accounts: bool,
) -> Result<Vec<Account>, String> {
    let url = format!(
        "/api/accounts_by_category/{}",
        category.to_string()
    );
    let fetched_accounts = Api::get(&url, user_ctx, navigator).await;
    match fetched_accounts {
        Ok(response) if response.ok() => match response.json::<Vec<Account>>().await {
            Ok(data) => {
                if include_group_accounts {
                    Ok(data)
                } else {
                    let postable_accounts: Vec<Account> = data.into_iter().filter(|a| !a.is_group).collect();
                    Ok(postable_accounts)
                }
            },
            Err(e) => Err(i18n.t_args(
                "new-vendor-invoice-error-parse-accounts",
                &fluent_args!["error" => e.to_string()],
            )),
        },
        Ok(response) => Err(i18n.t_args(
            "new-vendor-invoice-error-fetch-accounts",
            &fluent_args!["status" => response.status()],
        )),
        Err(e) => Err(i18n.t_args(
            "common-network-error",
            &fluent_args!["error" => e.to_string()],
        )),
    }

}