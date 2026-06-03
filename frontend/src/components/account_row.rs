/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::contexts::auth_context::use_user_context;
use crate::contexts::locale_context::use_locale;
use crate::router::Route;
use shared_core::dtos::account_with_balance::AccountWithBalance;
use shared_core::models::auth::SystemPrivilege;
use std::collections::HashSet;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct AccountNode {
    pub account: AccountWithBalance,
    pub children: Vec<AccountNode>,
}

#[derive(Properties, PartialEq)]
pub struct AccountRowProps {
    pub node: AccountNode,
    pub depth: usize,
    pub collapsed_nodes: UseStateHandle<HashSet<Uuid>>,
    pub on_edit: Callback<AccountWithBalance>,
    pub on_delete: Callback<AccountWithBalance>,
}

#[function_component(AccountRow)]
pub fn account_row(props: &AccountRowProps) -> Html {
    let user_ctx = use_user_context();
    let is_parent = !props.node.children.is_empty();
    let is_collapsed = props.collapsed_nodes.contains(&props.node.account.id);

    let i18n = use_locale();

    let on_toggle_collapse = {
        let collapsed_nodes = props.collapsed_nodes.clone();
        let node_id = props.node.account.id;
        Callback::from(move |_| {
            let mut new_set = (*collapsed_nodes).clone();
            if new_set.contains(&node_id) {
                new_set.remove(&node_id);
            } else {
                new_set.insert(node_id);
            }
            collapsed_nodes.set(new_set);
        })
    };

    let on_edit_click = {
        let on_edit = props.on_edit.clone();
        let account = props.node.account.clone();
        Callback::from(move |_| {
            on_edit.emit(account.clone());
        })
    };
    let on_delete_click = {
        let on_delete = props.on_delete.clone();
        let account = props.node.account.clone();
        Callback::from(move |_| {
            on_delete.emit(account.clone());
        })
    };

    let name_style = format!("padding-left: {}rem;", props.depth as f64 * 1.5);

    let account_name_display = if props.node.account.is_group {
        html! { { &props.node.account.name } }
    } else if user_ctx.has_privilege(&SystemPrivilege::use_transactions) {
        html! {
            <Link<Route> to={Route::AccountLedger { id: props.node.account.id }}>
                { &props.node.account.name }
            </Link<Route>>
        }
    } else {
        html! { { &props.node.account.name } }
    };

    html! {
        <>
            <tr class={if is_parent { "parent-account" } else { "" }}>
                <td>{ &props.node.account.code }</td>
                <td style={name_style}>
                    if is_parent {
                        <button onclick={on_toggle_collapse} class="collapse-toggle">
                            if is_collapsed {
                                <img src="/images/chevron-right.svg" alt={i18n.t("common-expand")} />
                            } else {
                                <img src="/images/chevron-down.svg" alt={i18n.t("common-collapse")} />
                            }
                        </button>
                    }
                    { account_name_display }
                </td>
                <td>{ props.node.account.category.to_string() }</td>
                <td style="text-align: right;">{ i18n.format_currency(props.node.account.balance) }</td>
                <td class="table__col-actions">
                    <div class="actions-wrapper">
                        { if user_ctx.has_privilege(&SystemPrivilege::manage_accounts) {
                            html! {
                                <>
                                    <button class="icon-button btn-action" onclick={on_edit_click}>
                                        <img src="/images/edit.svg" alt={i18n.t("common-edit")} />
                                    </button>
                                    if !is_parent && props.node.account.balance == 0 {
                                        <button class="icon-button btn-action" onclick={on_delete_click}>
                                            <img src="/images/delete.svg" alt={i18n.t("common-delete")} />
                                        </button>
                                    }
                                </>
                            }
                        } else {
                            html! {}
                        }}
                    </div>
                </td>
            </tr>
            if is_parent && !is_collapsed {
                { for props.node.children.iter().map(|child_node| {
                    html! {
                        <AccountRow
                            key={child_node.account.id.to_string()}
                            node={child_node.clone()}
                            depth={props.depth + 1}
                            collapsed_nodes={props.collapsed_nodes.clone()}
                            on_edit={props.on_edit.clone()}
                            on_delete={props.on_delete.clone()}
                        />
                    }
                })}
            }
        </>
    }
}
