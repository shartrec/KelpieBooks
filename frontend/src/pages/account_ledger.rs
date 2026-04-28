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

use crate::components::layout::Layout;
use crate::components::transaction_row::{TransactionGroup, TransactionRow};
use crate::pages::new_transaction::NewTransactionQuery;
use crate::Route;
use gloo_net::http::Request;
use log::info;
use shared_core::dtos::journal_entry_with_balance::JournalEntryWithBalance;
use shared_core::models::Account;
use std::collections::HashMap;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct AccountLedgerPageProps {
    pub account_id: Uuid,
}

#[function_component(AccountLedgerPage)]
pub fn account_ledger_page(props: &AccountLedgerPageProps) -> Html {
    info!("Account Ledger Props {:?}", props);
    let entries = use_state(|| Vec::<JournalEntryWithBalance>::new());
    let account = use_state(|| None::<Account>);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);

    let fetch_entries = {
        let entries = entries.clone();
        let error = error.clone();
        let loading = loading.clone();
        let account_id = props.account_id;
        Callback::from(move |()| {
            let entries = entries.clone();
            let error = error.clone();
            let loading = loading.clone();
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                let entries_url = format!("/api/accounts/{}/entries", account_id);
                let fetched_entries = Request::get(&entries_url).send().await;
                loading.set(false);
                match fetched_entries {
                    Ok(response) if response.ok() => {
                        match response.json::<Vec<JournalEntryWithBalance>>().await {
                            Ok(data) => entries.set(data),
                            Err(e) => error.set(Some(format!("Failed to parse entries: {}", e))),
                        }
                    }
                    Ok(response) => error.set(Some(format!(
                        "Failed to fetch entries: {}",
                        response.status()
                    ))),
                    Err(e) => error.set(Some(format!("Network error: {}", e))),
                }
            });
        })
    };

    {
        let account = account.clone();
        let account_id = props.account_id;
        let fetch_entries = fetch_entries.clone();
        use_effect_with(account_id, move |&account_id| {
            wasm_bindgen_futures::spawn_local(async move {
                let acc_url = format!("/api/accounts/{}", account_id);
                if let Ok(response) = Request::get(&acc_url).send().await {
                    if let Ok(acc_data) = response.json::<Account>().await {
                        account.set(Some(acc_data));
                    }
                }
            });
            fetch_entries.emit(());
            || ()
        });
    }

    let on_reverse = {
        let fetch_entries = fetch_entries.clone();
        let error = error.clone();
        Callback::from(move |transaction_id: Uuid| {
            let fetch_entries = fetch_entries.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("/api/transactions/{}/reverse", transaction_id);
                let resp = Request::post(&url).send().await;
                if resp.is_ok() {
                    fetch_entries.emit(());
                } else {
                    error.set(Some("Failed to reverse transaction.".to_string()));
                }
            });
        })
    };

    let account_name = account.as_ref().map(|a| a.name.clone()).unwrap_or_default();
    let query = NewTransactionQuery {
        from_account: Some(props.account_id),
    };

    let transaction_groups = {
        let mut groups: HashMap<Uuid, TransactionGroup> = HashMap::new();
        for entry in entries.iter() {
            if entry.account_id == props.account_id {
                groups.insert(
                    entry.transaction_id,
                    TransactionGroup {
                        transaction_id: entry.transaction_id,
                        date: entry.date,
                        description: entry.description.clone(),
                        primary_entry: entry.clone(),
                    },
                );
            }
        }
        let mut sorted_groups: Vec<TransactionGroup> = groups.into_values().collect();
        sorted_groups.sort_by(|a, b| a.date.cmp(&b.date));
        sorted_groups
    };

    html! {
        <Layout>
            <h1>{ format!("Ledger: {}", account_name) }</h1>
            <div class="table-actions">
                <Link<Route, NewTransactionQuery> to={Route::NewTransaction} query={query} classes="button">
                    { "Add New Transaction" }
                </Link<Route, NewTransactionQuery>>
            </div>
            if *loading {
                <p>{ "Loading..." }</p>
            } else if let Some(err) = &*error {
                <div class="error">{ err }</div>
            } else {
                <table class="table">
                    <thead>
                        <tr>
                            <th>{ "Date" }</th>
                            <th>{ "Description" }</th>
                            <th class="amount">{ "Debit" }</th>
                            <th class="amount">{ "Credit" }</th>
                            <th class="amount">{ "Balance" }</th>
                            <th class="actions-col"></th>
                        </tr>
                    </thead>
                    <tbody>
                        { for transaction_groups.into_iter().map(|group| html! {
                            <TransactionRow
                                key={group.transaction_id.to_string()}
                                transaction_group={group.clone()}
                                on_reverse={on_reverse.clone()}
                            />
                        })}
                    </tbody>
                </table>
            }
        </Layout>
    }
}
