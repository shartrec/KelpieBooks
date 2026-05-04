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

use crate::Route;
use shared_core::dtos::account_with_balance::AccountWithBalance;
use std::collections::HashSet;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;
use shared_core::util::format_currency;

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
    let is_parent = !props.node.children.is_empty();
    let is_collapsed = props.collapsed_nodes.contains(&props.node.account.id);

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
    } else {
        html! {
            <Link<Route> to={Route::AccountLedger { id: props.node.account.id }}>
                { &props.node.account.name }
            </Link<Route>>
        }
    };

    html! {
        <>
            <tr class={if is_parent { "parent-account" } else { "" }}>
                <td>{ &props.node.account.code }</td>
                <td style={name_style}>
                    if is_parent {
                        <button onclick={on_toggle_collapse} class="collapse-toggle">
                            if is_collapsed {
                                <img src="/images/chevron-right.svg" alt="Expand" />
                            } else {
                                <img src="/images/chevron-down.svg" alt="Collapse" />
                            }
                        </button>
                    }
                    { account_name_display }
                </td>
                <td>{ props.node.account.category.to_string() }</td>
                <td style="text-align: right;">{ format_currency(&props.node.account.balance) }</td>
                <td class="actions-cell">
                    <button class="icon-button" onclick={on_edit_click}>
                        <img src="/images/edit.svg" alt="Edit" />
                    </button>
                    if !is_parent && props.node.account.balance == 0 {
                        <button class="icon-button" onclick={on_delete_click}>
                            <img src="/images/delete.svg" alt="Delete" />
                        </button>
                    }
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
