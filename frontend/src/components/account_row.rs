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
use shared_core::dtos::account_with_balance::AccountWithBalance;
use std::collections::HashSet;
use uuid::Uuid;
use yew_router::prelude::*;
use crate::Route;

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

    let on_edit_click = { let on_edit = props.on_edit.clone(); let account = props.node.account.clone(); Callback::from(move |_| { on_edit.emit(account.clone()); }) };
    let on_delete_click = { let on_delete = props.on_delete.clone(); let account = props.node.account.clone(); Callback::from(move |_| { on_delete.emit(account.clone()); }) };

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
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="9 18 15 12 9 6"></polyline></svg>
                            } else {
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="6 9 12 15 18 9"></polyline></svg>
                            }
                        </button>
                    }
                    { account_name_display }
                </td>
                <td>{ props.node.account.category.to_string() }</td>
                <td style="text-align: right;">{ format!("{:.2}", (props.node.account.balance as f64) / 100.0) }</td>
                <td class="actions-cell">
                    <button class="icon-button" onclick={on_edit_click}>
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path><path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"></path></svg>
                    </button>
                    <button class="icon-button" onclick={on_delete_click}>
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path></svg>
                    </button>
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
