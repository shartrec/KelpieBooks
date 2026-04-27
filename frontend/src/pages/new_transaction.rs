/*
 * Copyright (c) 2026. Trevor Campbell and others.
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
use yew_router::prelude::*;
use uuid::Uuid;
use crate::components::layout::Layout;
use shared_core::requests::transaction::{CreateTransactionRequest, JournalEntryLine};
use crate::components::journal_entry_row::JournalEntryRow;
use shared_core::dtos::account_with_balance::AccountWithBalance;
use gloo_net::http::Request;
use serde::Deserialize;

#[derive(Deserialize, PartialEq, Clone)]
pub struct NewTransactionQuery {
    #[serde(rename = "from_account")]
    pub from_account: Option<Uuid>,
}

#[derive(Properties, PartialEq)]
pub struct NewTransactionPageProps {
    #[prop_or_default]
    #[prop_or_else(NewTransactionQuery::default)]
    pub query: NewTransactionQuery,
}

impl Default for NewTransactionQuery {
    fn default() -> Self {
        Self { from_account: None }
    }
}

#[function_component(NewTransactionPage)]
pub fn new_transaction_page(props: &NewTransactionPageProps) -> Html {
    let request = use_state(CreateTransactionRequest::default);
    let postable_accounts = use_state(Vec::new);
    let navigator = use_navigator().unwrap();

    {
        let postable_accounts = postable_accounts.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(response) = Request::get("/api/accounts").send().await {
                    if let Ok(accounts) = response.json::<Vec<AccountWithBalance>>().await {
                        let postable = accounts.into_iter()
                            .filter(|a| !a.is_group)
                            .map(|a| (a.id, a.name))
                            .collect();
                        postable_accounts.set(postable);
                    }
                }
            });
        });
    }

    use_effect_with(props.query.from_account, {
        let request = request.clone();
        move |from_account| {
            let mut entries = vec![
                JournalEntryLine::default(),
                JournalEntryLine::default(),
            ];
            if let Some(id) = from_account {
                entries[0].account_id = *id;
            }
            let mut new_req = (*request).clone();
            new_req.entries = entries;
            request.set(new_req);
            || ()
        }
    });

    let on_entry_change = {
        let request = request.clone();
        Callback::from(move |(index, updated_entry): (usize, JournalEntryLine)| {
            let mut new_req = (*request).clone();
            if let Some(entry) = new_req.entries.get_mut(index) {
                *entry = updated_entry;

                let total_debits: i64 = new_req.entries.iter().map(|e| e.debit).sum();
                let total_credits: i64 = new_req.entries.iter().map(|e| e.credit).sum();
                if index == new_req.entries.len() - 1 && total_debits != total_credits {
                    new_req.entries.push(JournalEntryLine::default());
                }
            }
            request.set(new_req);
        })
    };

    let on_delete_line = {
        let request = request.clone();
        Callback::from(move |index: usize| {
            if request.entries.len() > 2 {
                let mut new_req = (*request).clone();
                new_req.entries.remove(index);
                request.set(new_req);
            }
        })
    };

    let total_debits: i64 = request.entries.iter().map(|e| e.debit).sum();
    let total_credits: i64 = request.entries.iter().map(|e| e.credit).sum();
    let is_balanced = total_debits > 0 && total_debits == total_credits;

    let on_submit = {
        let request = request.clone();
        let navigator = navigator.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if is_balanced {
                let mut req = (*request).clone();
                req.entries.retain(|entry| !entry.account_id.is_nil() && (entry.debit != 0 || entry.credit != 0));
                let navigator = navigator.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let resp = Request::post("/api/transactions").json(&req).unwrap().send().await;
                    if resp.is_ok() {
                        navigator.back();
                    } else {
                        // TODO: Handle error
                    }
                });
            }
        })
    };

    html! {
        <Layout>
            <h1>{ "New Journal Transaction" }</h1>
            <form onsubmit={on_submit} class="transaction-form">
                // TODO: Add Date and Description fields

                <div class="journal-entries">
                    { for request.entries.iter().enumerate().map(|(i, entry)| {
                        let on_change = { let on_entry_change = on_entry_change.clone(); Callback::from(move |updated_entry| { on_entry_change.emit((i, updated_entry)); }) };
                        let on_delete = { let on_delete_line = on_delete_line.clone(); Callback::from(move |_| { on_delete_line.emit(i); }) };
                        html!{
                            <JournalEntryRow
                                key={i}
                                entry={entry.clone()}
                                on_change={on_change}
                                on_delete={on_delete}
                                accounts={(*postable_accounts).clone()}
                            />
                        }
                    })}
                </div>

                <div class="totals">
                    <div>{ format!("Debits: {:.2}", total_debits as f64 / 100.0) }</div>
                    <div>{ format!("Credits: {:.2}", total_credits as f64 / 100.0) }</div>
                    <div class={if is_balanced { "balanced" } else { "unbalanced" }}>
                        { if is_balanced { "Balanced" } else { "Unbalanced" } }
                    </div>
                </div>

                <div class="form-actions">
                    <button type="submit" disabled={!is_balanced}>{ "Save Transaction" }</button>
                </div>
            </form>
        </Layout>
    }
}
