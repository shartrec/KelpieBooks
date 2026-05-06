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

use crate::components::journal_entry_row::JournalEntryRow;
use crate::components::layout::Layout;
use crate::Route;
use chrono::NaiveDate;
use gloo_net::http::Request;
use log::info;
use serde::{Deserialize, Serialize};
use shared_core::dtos::account_with_balance::AccountWithBalance;
use shared_core::models::Account;
use shared_core::requests::transaction::{CreateTransactionRequest, JournalEntryLine};
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct NewTransactionQuery {
    #[serde(default)]
    #[serde(rename = "from_account")]
    pub from_account: Option<Uuid>,
    #[serde(default)]
    #[serde(rename = "duplicate_from")]
    pub duplicate_from: Option<Uuid>,
}

#[function_component(NewTransactionPage)]
pub fn new_transaction_page() -> Html {
    let request = use_state(CreateTransactionRequest::default);
    let focus_index = use_state(|| None::<usize>);
    let postable_accounts = use_state(Vec::new);
    let from_account = use_state(|| None::<Account>);
    let navigator = use_navigator().unwrap();
    let location = use_location().unwrap();

    {
        let request = request.clone();
        let postable_accounts = postable_accounts.clone();
        let from_account = from_account.clone();

        use_effect_with((), move |_| {
            let query = location.query::<NewTransactionQuery>().ok();
            let from_account_id = query.as_ref().and_then(|q| q.from_account);
            let duplicate_from_id = query.as_ref().and_then(|q| q.duplicate_from);
            info!("Parsed IDs from URL: from_account={:?}, duplicate_from={:?}", from_account_id, duplicate_from_id);

            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(response) = Request::get("/api/accounts").send().await {
                    if let Ok(accounts) = response.json::<Vec<AccountWithBalance>>().await {
                        let postable = accounts
                            .into_iter()
                            .filter(|a| !a.is_group)
                            .map(|a| (a.id, a.name))
                            .collect();
                        postable_accounts.set(postable);
                    }
                }

                if let Some(id) = from_account_id {
                    if let Ok(response) =
                        Request::get(&format!("/api/accounts/{}", id)).send().await
                    {
                        if let Ok(acc) = response.json::<Account>().await {
                            from_account.set(Some(acc));
                        }
                    }
                }

                let mut new_req = CreateTransactionRequest::default();

                if let Some(id) = duplicate_from_id {
                    if let Ok(response) = Request::get(&format!("/api/transactions/{}", id)).send().await {
                        if let Ok(detail) = response.json::<shared_core::dtos::transaction_detail::TransactionDetail>().await {
                            new_req.description = detail.transaction.description;
                            new_req.reference = detail.transaction.reference;
                            new_req.entries = detail.entries.into_iter().map(|e| JournalEntryLine {
                                line_id: Uuid::new_v4(),
                                account_id: e.account_id,
                                debit: e.debit,
                                credit: e.credit,
                                description: e.description,
                            }).collect();
                        }
                    }
                } else {
                    let mut entries = vec![JournalEntryLine::default(), JournalEntryLine::default()];
                    if let Some(id) = from_account_id {
                        entries[0].account_id = id;
                    }
                    new_req.entries = entries;
                }
                request.set(new_req);
            });
            || ()
        });
    }

    let on_entry_change = {
        let request = request.clone();
        Callback::from(move |(index, updated_entry): (usize, JournalEntryLine)| {
            let mut new_req = (*request).clone();
            if let Some(entry) = new_req.entries.get_mut(index) {
                *entry = updated_entry;

                let total_debits: i64 = new_req.entries.iter().map(|e| e.debit).sum();
                let total_credits: i64 = new_req.entries.iter().map(|e| e.credit).sum();
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

    let on_date_change = {
        let request = request.clone();
        Callback::from(move |e: Event| {
            let value = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            if let Ok(date) = NaiveDate::parse_from_str(&value, "%Y-%m-%d") {
                let mut new_req = (*request).clone();
                new_req.date = date;
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
                req.entries.retain(|entry| {
                    !entry.account_id.is_nil() && (entry.debit != 0 || entry.credit != 0)
                });
                let navigator = navigator.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let resp = Request::post("/api/transactions")
                        .json(&req)
                        .unwrap()
                        .send()
                        .await;
                    if resp.is_ok() {
                        navigator.back();
                    } else {
                        // TODO: Handle error
                    }
                });
            }
        })
    };

    let on_cancel = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.back();
        })
    };

    let page_header = if let Some(acc) = &*from_account {
        html! {
            <div class="page-subheader">
                <h3>{ "New Transaction for Account: " }<Link<Route> to={Route::AccountLedger { id: acc.id }}>{ &acc.name }</Link<Route>></h3>
            </div>
        }
    } else {
        html! {}
    };

    let value = focus_index.clone();
    let add_line = {
        let request = request.clone();
        Callback::from(move |_| {
            let mut new_req = (*request).clone();
            let new_idx = new_req.entries.len();
            new_req.entries.push(JournalEntryLine::default());
            request.set(new_req);
            value.set(Some(new_idx));
        })
    };

    html! {
        <Layout>
            <h1>{ "New Journal Transaction" }</h1>
            { page_header }
            <form onsubmit={on_submit} class="transaction-form">
                <div class="transaction-header">
                    <label>
                        { "Date:" }
                        <input type="date" value={request.date.to_string()} onchange={on_date_change} />
                    </label>
                </div>

                <div class="journal-entries">
                    <div class="journal-entry-header">
                        <span>{ "Account" }</span>
                        <span>{ "Description" }</span>
                        <span>{ "Debit" }</span>
                        <span>{ "Credit" }</span>
                        <span></span>
                    </div>
                    { for request.entries.iter().enumerate().map(|(i, entry)| {
                        let on_change = { let on_entry_change = on_entry_change.clone(); Callback::from(move |updated_entry| { on_entry_change.emit((i, updated_entry)); }) };
                        let on_delete = { let on_delete_line = on_delete_line.clone(); Callback::from(move |_| { on_delete_line.emit(i); }) };
                        html!{
                            <JournalEntryRow
                                key={entry.line_id.to_string()}
                                entry={entry.clone()}
                                on_change={on_change}
                                on_delete={on_delete}
                                accounts={(*postable_accounts).clone()}
                                should_focus={*focus_index == Some(i)}
                            />
                        }
                    })}
                </div>
                <div class="form-actions">
                    <button type="button" onclick={add_line} class="button-add-row">{ "Add Line" }</button>
                </div>
                <div class="totals">
                    <div>{ format!("Debits: {:.2}", total_debits as f64 / 100.0) }</div>
                    <div>{ format!("Credits: {:.2}", total_credits as f64 / 100.0) }</div>
                    <div class={if is_balanced { "balanced" } else { "unbalanced" }}>
                        { if is_balanced { "Balanced" } else { "Unbalanced" } }
                    </div>
                </div>

                <div class="form-actions">
                    <button type="button" onclick={on_cancel} class="button-secondary">{ "Cancel" }</button>
                    <button type="submit" disabled={!is_balanced}>{ "Save Transaction" }</button>
                </div>
            </form>
        </Layout>
     }
}
