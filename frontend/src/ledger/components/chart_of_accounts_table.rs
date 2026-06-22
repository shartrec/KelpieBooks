/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use std::collections::{
    HashMap,
    HashSet,
};

use fluent::fluent_args;
use log::info;
use shared_core::ledger::{
    dtos::account_with_balance::AccountWithBalance,
    requests::account::{
        CreateAccountRequest,
        UpdateAccountRequest,
    },
};
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use shared_core::core::models::auth::SystemPrivilege;
use crate::{
    api::Api,
    contexts::{
        auth_context::use_user_context,
        locale_context::use_locale,
    },
    ledger::components::{
        account_row::{
            AccountNode,
            AccountRow,
        },
        add_account_modal::AddAccountModal,
        edit_account_modal::EditAccountModal,
    },
};
use crate::core::components::delete_confirmation_modal::DeleteConfirmationModal;

#[function_component(ChartOfAccountsTable)]
pub fn chart_of_accounts_table() -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();
    let accounts = use_state(|| Vec::<AccountWithBalance>::new());
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);
    let collapsed_nodes = use_state(HashSet::new);

    let show_add_modal = use_state(|| false);
    let account_to_edit = use_state(|| None::<AccountWithBalance>);
    let account_to_delete = use_state(|| None::<AccountWithBalance>);

    let fetch_accounts = {
        let accounts = accounts.clone();
        let error = error.clone();
        let loading = loading.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        Callback::from(move |_: ()| {
            let accounts = accounts.clone();
            let error = error.clone();
            let loading = loading.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            loading.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_accounts =
                    Api::get("/api/accounts_with_balances", user_ctx, navigator).await;
                loading.set(false);
                match fetched_accounts {
                    Ok(response) if response.ok() => {
                        match response.json::<Vec<AccountWithBalance>>().await {
                            Ok(mut data) => {
                                info!("Successfully fetched {} accounts.", data.len());
                                data.sort_by(|a, b| a.code.cmp(&b.code));
                                accounts.set(data);
                            }
                            Err(e) => error.set(Some(i18n.t_args(
                                "coa-error-parse-accounts",
                                &fluent_args!["error" => e.to_string()],
                            ))),
                        }
                    }
                    Ok(response) => error.set(Some(i18n.t_args(
                        "coa-error-fetch-accounts",
                        &fluent_args!["status" => response.status()],
                    ))),
                    Err(e) => error.set(Some(
                        i18n.t_args("coa-error-network", &fluent_args!["error" => e.to_string()]),
                    )),
                }
            });
        })
    };

    let on_modal_close = {
        let show_add_modal = show_add_modal.clone();
        let account_to_edit = account_to_edit.clone();
        let account_to_delete = account_to_delete.clone();
        Callback::from(move |_: ()| {
            show_add_modal.set(false);
            account_to_edit.set(None);
            account_to_delete.set(None);
        })
    };
    let on_add_submit = {
        let on_modal_close = on_modal_close.clone();
        let error = error.clone();
        let fetch_accounts = fetch_accounts.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        Callback::from(move |req: CreateAccountRequest| {
            let on_modal_close = on_modal_close.clone();
            let error = error.clone();
            let fetch_accounts = fetch_accounts.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Api::post("/api/accounts", &req, user_ctx, navigator).await;
                match resp {
                    Ok(r) if r.ok() => {
                        on_modal_close.emit(());
                        fetch_accounts.emit(());
                    }
                    Ok(r) => error.set(Some(i18n.t_args(
                        "coa-error-add-account",
                        &fluent_args!["status" => r.status()],
                    ))),
                    Err(e) => error.set(Some(
                        i18n.t_args("coa-error-network", &fluent_args!["error" => e.to_string()]),
                    )),
                }
            });
        })
    };
    let on_edit_submit = {
        let on_modal_close = on_modal_close.clone();
        let error = error.clone();
        let fetch_accounts = fetch_accounts.clone();
        let account_id = account_to_edit.as_ref().map(|a| a.id);
        let i18n = i18n.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        Callback::from(move |req: UpdateAccountRequest| {
            if let Some(id) = account_id {
                let on_modal_close = on_modal_close.clone();
                let error = error.clone();
                let fetch_accounts = fetch_accounts.clone();
                let user_ctx = user_ctx.clone();
                let i18n = i18n.clone();
                let navigator = navigator.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let resp =
                        Api::put(&format!("/api/accounts/{}", id), &req, user_ctx, navigator).await;
                    match resp {
                        Ok(r) if r.ok() => {
                            on_modal_close.emit(());
                            fetch_accounts.emit(());
                        }
                        Ok(r) => error.set(Some(i18n.t_args(
                            "coa-error-update-account",
                            &fluent_args!["status" => r.status()],
                        ))),
                        Err(e) => {
                            error.set(Some(i18n.t_args(
                                "coa-error-network",
                                &fluent_args!["error" => e.to_string()],
                            )))
                        }
                    }
                });
            }
        })
    };
    let on_delete_confirm = {
        let on_modal_close = on_modal_close.clone();
        let error = error.clone();
        let fetch_accounts = fetch_accounts.clone();
        let account_id = account_to_delete.as_ref().map(|a| a.id);
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        Callback::from(move |_: ()| {
            if let Some(id) = account_id {
                let on_modal_close = on_modal_close.clone();
                let error = error.clone();
                let fetch_accounts = fetch_accounts.clone();
                let user_ctx = user_ctx.clone();
                let i18n = i18n.clone();
                let navigator = navigator.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let resp =
                        Api::delete(&format!("/api/accounts/{}", id), user_ctx, navigator).await;
                    match resp {
                        Ok(r) if r.ok() => {
                            on_modal_close.emit(());
                            fetch_accounts.emit(());
                        }
                        Ok(r) => error.set(Some(i18n.t_args(
                            "coa-error-delete-account",
                            &fluent_args!["status" => r.status()],
                        ))),
                        Err(e) => {
                            error.set(Some(i18n.t_args(
                                "coa-error-network",
                                &fluent_args!["error" => e.to_string()],
                            )))
                        }
                    }
                });
            }
        })
    };

    use_effect_with((), move |()| {
        fetch_accounts.emit(());
        || ()
    });

    let on_add_click = {
        let show_add_modal = show_add_modal.clone();
        Callback::from(move |_| show_add_modal.set(true))
    };
    let on_edit_click = {
        let account_to_edit = account_to_edit.clone();
        Callback::from(move |acc| account_to_edit.set(Some(acc)))
    };
    let on_delete_click = {
        let account_to_delete = account_to_delete.clone();
        Callback::from(move |acc| account_to_delete.set(Some(acc)))
    };

    let root_nodes = use_memo((*accounts).clone(), |accounts: &Vec<AccountWithBalance>| {
        let accounts_map: HashMap<Uuid, AccountWithBalance> =
            accounts.iter().map(|acc| (acc.id, acc.clone())).collect();
        let mut parent_to_children: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
        for acc in accounts.iter() {
            if let Some(parent_id) = acc.parent_id {
                parent_to_children
                    .entry(parent_id)
                    .or_default()
                    .push(acc.id);
            }
        }
        fn build_node(
            acc: &AccountWithBalance,
            map: &HashMap<Uuid, AccountWithBalance>,
            pc_map: &HashMap<Uuid, Vec<Uuid>>,
        ) -> AccountNode {
            let mut children = Vec::new();
            if let Some(child_ids) = pc_map.get(&acc.id) {
                for child_id in child_ids {
                    if let Some(child_acc) = map.get(child_id) {
                        children.push(build_node(child_acc, map, pc_map));
                    }
                }
            }
            children.sort_by(|a, b| a.account.code.cmp(&b.account.code));
            AccountNode {
                account: acc.clone(),
                children,
            }
        }
        let mut roots = Vec::new();
        for account in accounts.iter() {
            if account.parent_id.is_none() {
                roots.push(build_node(account, &accounts_map, &parent_to_children));
            }
        }
        roots.sort_by(|a, b| a.account.code.cmp(&b.account.code));
        roots
    });

    let parent_accounts: Vec<(Uuid, String)> = (*accounts)
        .iter()
        .filter(|a| a.is_group)
        .map(|a| (a.id, a.name.clone()))
        .collect();

    if *loading {
        return html! { <p>{ i18n.t("common-loading") }</p> };
    }
    if let Some(err) = &*error {
        return html! { <div class="error">{ err }</div> };
    }

    fn render_nodes(
        nodes: &[AccountNode],
        depth: usize,
        collapsed: &UseStateHandle<HashSet<Uuid>>,
        on_edit: &Callback<AccountWithBalance>,
        on_delete: &Callback<AccountWithBalance>,
    ) -> Vec<Html> {
        let mut rows = Vec::new();
        for node in nodes {
            rows.push(html! {
                <AccountRow
                    key={node.account.id.to_string()}
                    node={node.clone()}
                    depth={depth}
                    collapsed_nodes={collapsed.clone()}
                    on_edit={on_edit.clone()}
                    on_delete={on_delete.clone()}
                />
            });
        }
        rows
    }

    html! {
        <>
            <div class="table-actions">
                { if user_ctx.has_privilege(&SystemPrivilege::ManageAccounts) {
                    html! {
                        <button onclick={on_add_click}>{ i18n.t("coa-add-account-button") }</button>
                    }
                } else {
                    html! {}
                }}
            </div>

            if *show_add_modal { <AddAccountModal on_close={on_modal_close.clone()} on_submit={on_add_submit} parent_accounts={parent_accounts} /> }
            if let Some(account) = &*account_to_edit { <EditAccountModal account={account.clone()} on_close={on_modal_close.clone()} on_submit={on_edit_submit} /> }
            if let Some(account) = &*account_to_delete {
                <DeleteConfirmationModal
                    title={i18n.t("common-confirm-deletion")}
                    message={i18n.t_args("account-delete-confirm-message", &fluent_args!["name" => account.name.clone()])}
                    warning={ i18n.t("account-delete-confirm-warning")  }
                    on_cancel={on_modal_close.clone()}
                    on_confirm={on_delete_confirm}

            /> }

            <table class="table coa-table">
                <thead>
                    <tr>
                        <th class="table__text-col">{ i18n.t("common-code") }</th>
                        <th class="table__text-col">{ i18n.t("common-name") }</th>
                        <th class="table__text-col">{ i18n.t("common-category") }</th>
                        <th class="table__value-col">{ i18n.t("common-balance") }</th>
                        <th class="table__col-actions">{ i18n.t("common-actions") }</th>
                    </tr>
                </thead>
                <tbody>
                    { render_nodes(&root_nodes, 0, &collapsed_nodes, &on_edit_click, &on_delete_click) }
                </tbody>
            </table>
        </>
    }
}
