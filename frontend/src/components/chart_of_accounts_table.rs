/*
 * Copyright (c) 2026-2026. Trevor Campbell and others.
 *
 * This file is part of KelpieBooks.
 *
 * KelpieBooks is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License,or
 * (at your option) any later version.
 *
 * KelpieBooks is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with KelpieBooks; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 *
 * Contributors:
 *      Trevor Campbell
 *
 */

use yew::prelude::*;
use gloo_net::http::Request;
use shared_core::dtos::account_with_balance::AccountWithBalance;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use crate::components::account_row::{AccountNode, AccountRow};
use crate::components::add_account_modal::AddAccountModal;
use crate::components::edit_account_modal::EditAccountModal;
use crate::components::delete_confirmation_modal::DeleteConfirmationModal;
use shared_core::requests::account::{CreateAccountRequest, UpdateAccountRequest};

#[function_component(ChartOfAccountsTable)]
pub fn chart_of_accounts_table() -> Html {
    let accounts = use_state(|| Vec::<AccountWithBalance>::new());
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);
    let collapsed_nodes = use_state(HashSet::new);

    // State for modals
    let show_add_modal = use_state(|| false);
    let account_to_edit = use_state(|| None::<AccountWithBalance>);
    let account_to_delete = use_state(|| None::<AccountWithBalance>);

    let fetch_accounts = {
        let accounts = accounts.clone();
        let error = error.clone();
        let loading = loading.clone();
        Callback::from(move |_| {
            let accounts = accounts.clone();
            let error = error.clone();
            let loading = loading.clone();
            loading.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_accounts = Request::get("/api/accounts").send().await;
                loading.set(false);
                match fetched_accounts {
                    Ok(response) if response.ok() => {
                        match response.json::<Vec<AccountWithBalance>>().await {
                            Ok(mut data) => {
                                data.sort_by(|a, b| a.code.cmp(&b.code));
                                accounts.set(data);
                            },
                            Err(e) => error.set(Some(format!("Failed to parse accounts: {}", e))),
                        }
                    }
                    Ok(response) => error.set(Some(format!("Failed to fetch accounts: {}", response.status()))),
                    Err(e) => error.set(Some(format!("Network error: {}", e))),
                }
            });
        })
    };

    let on_modal_close = {
        let show_add_modal = show_add_modal.clone();
        let account_to_edit = account_to_edit.clone();
        let account_to_delete = account_to_delete.clone();
        Callback::from(move |_| {
            show_add_modal.set(false);
            account_to_edit.set(None);
            account_to_delete.set(None);
        })
    };

    let on_add_submit = {
        let on_modal_close = on_modal_close.clone();
        let error = error.clone();
        let fetch_accounts = fetch_accounts.clone();
        Callback::from(move |req: CreateAccountRequest| {
            let on_modal_close = on_modal_close.clone();
            let error = error.clone();
            let fetch_accounts = fetch_accounts.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Request::post("/api/accounts").json(&req).unwrap().send().await;
                match resp {
                    Ok(r) if r.ok() => { on_modal_close.emit(()); fetch_accounts.emit(()); }
                    Ok(r) => error.set(Some(format!("Failed to add account: {}", r.status()))),
                    Err(e) => error.set(Some(format!("Network error: {}", e))),
                }
            });
        })
    };

    let on_edit_submit = {
        let on_modal_close = on_modal_close.clone();
        let error = error.clone();
        let fetch_accounts = fetch_accounts.clone();
        let account_id = account_to_edit.as_ref().map(|a| a.id);
        Callback::from(move |req: UpdateAccountRequest| {
            if let Some(id) = account_id {
                let on_modal_close = on_modal_close.clone();
                let error = error.clone();
                let fetch_accounts = fetch_accounts.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let resp = Request::put(&format!("/api/accounts/{}", id)).json(&req).unwrap().send().await;
                    match resp {
                        Ok(r) if r.ok() => { on_modal_close.emit(()); fetch_accounts.emit(()); }
                        Ok(r) => error.set(Some(format!("Failed to update account: {}", r.status()))),
                        Err(e) => error.set(Some(format!("Network error: {}", e))),
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
        Callback::from(move |_| {
            if let Some(id) = account_id {
                let on_modal_close = on_modal_close.clone();
                let error = error.clone();
                let fetch_accounts = fetch_accounts.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let resp = Request::delete(&format!("/api/accounts/{}", id)).send().await;
                    match resp {
                        Ok(r) if r.ok() => { on_modal_close.emit(()); fetch_accounts.emit(()); }
                        Ok(r) => error.set(Some(format!("Failed to delete account: {}", r.status()))),
                        Err(e) => error.set(Some(format!("Network error: {}", e))),
                    }
                });
            }
        })
    };

    use_effect_with((), move |()| {
        fetch_accounts.emit(());
        || ()
    });

    let on_add_click = { let show_add_modal = show_add_modal.clone(); Callback::from(move |_| show_add_modal.set(true)) };
    let on_edit_click = { let account_to_edit = account_to_edit.clone(); Callback::from(move |acc| account_to_edit.set(Some(acc))) };
    let on_delete_click = { let account_to_delete = account_to_delete.clone(); Callback::from(move |acc| account_to_delete.set(Some(acc))) };

    if *loading { return html! { <p>{ "Loading..." }</p> }; }
    if let Some(err) = &*error { return html! { <div class="error">{ err }</div> }; }

    let mut nodes: HashMap<Uuid, AccountNode> = (*accounts).iter().map(|acc| (acc.id, AccountNode { account: acc.clone(), children: vec![] })).collect();
    let mut parent_to_children: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
    for acc in (*accounts).iter() { if let Some(parent_id) = acc.parent_id { parent_to_children.entry(parent_id).or_default().push(acc.id); } }
    for (parent_id, children_ids) in parent_to_children {
        let mut children: Vec<AccountNode> = children_ids.iter().filter_map(|id| nodes.remove(id)).collect();
        children.sort_by(|a, b| a.account.code.cmp(&b.account.code));
        if let Some(parent_node) = nodes.get_mut(&parent_id) { parent_node.children = children; }
    }
    let mut root_nodes: Vec<AccountNode> = nodes.into_values().collect();
    root_nodes.sort_by(|a, b| a.account.code.cmp(&b.account.code));

    let parent_accounts: Vec<(Uuid, String)> = (*accounts).iter().filter(|a| a.is_group).map(|a| (a.id, a.name.clone())).collect();

    html! {
        <>
            <div class="table-actions">
                <button onclick={on_add_click}>{ "Add Account" }</button>
            </div>

            if *show_add_modal {
                <AddAccountModal on_close={on_modal_close.clone()} on_submit={on_add_submit} parent_accounts={parent_accounts} />
            }
            if let Some(account) = &*account_to_edit {
                <EditAccountModal account={account.clone()} on_close={on_modal_close.clone()} on_submit={on_edit_submit} />
            }
            if let Some(account) = &*account_to_delete {
                <DeleteConfirmationModal account={account.clone()} on_close={on_modal_close.clone()} on_confirm={on_delete_confirm} />
            }

            <table class="table coa-table">
                <thead>
                    <tr>
                        <th class="code-col">{ "Code" }</th>
                        <th class="name-col">{ "Name" }</th>
                        <th class="category-col">{ "Category" }</th>
                        <th class="balance-col">{ "Balance" }</th>
                        <th class="actions-col">{ "Actions" }</th>
                    </tr>
                </thead>
                <tbody>
                    { for root_nodes.into_iter().map(|node| html! {
                        <AccountRow
                            node={node}
                            depth={0}
                            collapsed_nodes={collapsed_nodes.clone()}
                            on_edit={on_edit_click.clone()}
                            on_delete={on_delete_click.clone()}
                        />
                    })}
                </tbody>
            </table>
        </>
    }
}
