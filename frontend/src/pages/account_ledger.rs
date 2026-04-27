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
use shared_core::dtos::journal_entry_with_balance::JournalEntryWithBalance;
use gloo_net::http::Request;
use crate::Route;
use shared_core::models::Account;
use serde::{Deserialize, Serialize};

#[derive(Properties, PartialEq)]
pub struct AccountLedgerPageProps {
    pub account_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct NewTransactionQuery {
    #[serde(rename = "from_account")]
    from_account: Option<Uuid>,
}

#[function_component(AccountLedgerPage)]
pub fn account_ledger_page(props: &AccountLedgerPageProps) -> Html {
    let entries = use_state(|| Vec::<JournalEntryWithBalance>::new());
    let account = use_state(|| None::<Account>);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);

    {
        let entries = entries.clone();
        let account = account.clone();
        let error = error.clone();
        let loading = loading.clone();
        let account_id = props.account_id;
        use_effect_with(account_id, move |&account_id| {
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);

                let acc_url = format!("/api/accounts/{}", account_id);
                let acc_resp = Request::get(&acc_url).send().await;
                if let Ok(response) = acc_resp {
                    if let Ok(acc_data) = response.json::<Account>().await {
                        account.set(Some(acc_data));
                    }
                }

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
                    Ok(response) => error.set(Some(format!("Failed to fetch entries: {}", response.status()))),
                    Err(e) => error.set(Some(format!("Network error: {}", e))),
                }
            });
            || ()
        });
    }

    let account_name = account.as_ref().map(|a| a.name.clone()).unwrap_or_default();
    let query = NewTransactionQuery { from_account: Some(props.account_id) };

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
                            <th style="text-align: right;">{ "Debit" }</th>
                            <th style="text-align: right;">{ "Credit" }</th>
                            <th style="text-align: right;">{ "Balance" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        { for entries.iter().map(|entry| html! {
                            <tr>
                                <td>{ entry.date.to_string() }</td>
                                <td>{ entry.description.clone().unwrap_or_default() }</td>
                                <td style="text-align: right;">{ format!("{:.2}", (entry.debit as f64) / 100.0) }</td>
                                <td style="text-align: right;">{ format!("{:.2}", (entry.credit as f64) / 100.0) }</td>
                                <td style="text-align: right;">{ format!("{:.2}", (entry.running_balance as f64) / 100.0) }</td>
                            </tr>
                        })}
                    </tbody>
                </table>
            }
        </Layout>
    }
}
