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
use crate::contexts::report_context::{use_report_context, ReportAction};
use crate::pages::new_transaction::NewTransactionQuery;
use crate::router::Route;
use gloo_net::http::Request;
use shared_core::dtos::journal_entry_with_balance::JournalEntryWithBalance;
use shared_core::requests::transaction::ReverseTransactionRequest;
use shared_core::models::Account;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;
use shared_core::util::format_currency;
use crate::components::je_reversal_confirmation_modal::ReversalConfirmationModal;
use crate::components::je_delete_confirmation_modal::DeleteConfirmationModal;
use crate::components::report_options::ReportOptions;
use crate::contexts::org_context::use_org_context;

#[derive(Debug, Properties, PartialEq)]
pub struct AccountLedgerPageProps {
    pub account_id: Uuid,
}

#[function_component(AccountLedgerPage)]
pub fn account_ledger_page(props: &AccountLedgerPageProps) -> Html {
    let report_ctx = use_report_context();
    let org_ctx = use_org_context();
    let navigator = use_navigator().unwrap();
    let entries = use_state(|| Rc::new(Vec::<JournalEntryWithBalance>::new()));
    let account = use_state(|| None::<Account>);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);
    let transaction_to_reverse = use_state(|| None::<JournalEntryWithBalance>);
    let transaction_to_delete = use_state(|| None::<JournalEntryWithBalance>);

    {
        let report_ctx = report_ctx.clone();
        let account_id = props.account_id;
        use_effect_with((report_ctx.date_range.clone(),), move |_| {
            let start_date = report_ctx.date_range.start_date;
            let end_date = report_ctx.date_range.end_date;
            report_ctx.dispatch(ReportAction::SetOnExportCsv(Some(Callback::from(move |_| {
                let url = format!("/api/accounts/{}/export/csv?start={}&end={}", account_id, start_date, end_date);
                web_sys::window().unwrap().location().set_href(&url).unwrap();
            }))));
            report_ctx.dispatch(ReportAction::SetOnExportTypst(Some(Callback::from(move |_| {
                let url = format!("/api/accounts/{}/export/pdf?start={}&end={}", account_id, start_date, end_date);
                web_sys::window().unwrap().location().set_href(&url).unwrap();
            }))));
            move || {
                report_ctx.dispatch(ReportAction::SetOnExportCsv(None));
                report_ctx.dispatch(ReportAction::SetOnExportTypst(None));
            }
        });
    }

    let fetch_entries = {
        let entries = entries.clone();
        let error = error.clone();
        let loading = loading.clone();
        let account_id = props.account_id;
        let report_ctx = use_report_context();
        Callback::from(move |()| {
            let entries = entries.clone();
            let error = error.clone();
            let loading = loading.clone();
            let start_date = report_ctx.date_range.start_date;
            let end_date = report_ctx.date_range.end_date;
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                let entries_url = format!("/api/accounts/{}/entries?start={}&end={}", account_id, start_date, end_date);
                let fetched_entries = Request::get(&entries_url).send().await;
                loading.set(false);
                match fetched_entries {
                    Ok(response) if response.ok() => {
                        match response.json::<Vec<JournalEntryWithBalance>>().await {
                            Ok(data) => entries.set(Rc::new(data)),
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

    let on_reverse_modal_close = {
        let transaction_to_reverse = transaction_to_reverse.clone();
        Callback::from(move |_: ()| {
            transaction_to_reverse.set(None);
        })
    };

    let on_delete_modal_close = {
        let transaction_to_delete = transaction_to_delete.clone();
        Callback::from(move |_: ()| {
            transaction_to_delete.set(None);
        })
    };

    {
        let account = account.clone();
        let account_id = props.account_id;
        let fetch_entries = fetch_entries.clone();
        let report_ctx = use_report_context();
        use_effect_with((account_id, report_ctx.date_range.clone()), move |(account_id, _)| {
            let account_id = *account_id;
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

    let on_reverse_confirm = {
        let on_modal_close = on_reverse_modal_close.clone();
        let fetch_entries = fetch_entries.clone();
        let error = error.clone();
        let transaction_id = transaction_to_reverse.as_ref().map(|t| t.transaction_id);
        Callback::from(move |description: String| {
            if let Some(id) = transaction_id {
                let on_modal_close = on_modal_close.clone();
                let fetch_entries = fetch_entries.clone();
                let error = error.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let url = format!("/api/transactions/{}/reverse", id);
                    let req_body = ReverseTransactionRequest { description };
                    let resp = Request::post(&url)
                        .json(&req_body)
                        .map_err(|e| e.to_string());

                    match resp {
                        Ok(req) => {
                            let resp = req.send().await;
                            match resp {
                                Ok(r) if r.ok() => {
                                    on_modal_close.emit(());
                                    fetch_entries.emit(());
                                }
                                Ok(r) => {
                                    error.set(Some(format!("Failed to reverse transaction: {}", r.status())))
                                }
                                Err(e) => error.set(Some(format!("Network error: {}", e))),
                            }
                        }
                        Err(e) => error.set(Some(format!("Serialization error: {}", e))),
                    }
                });
            }
        })
    };

    let on_delete_confirm = {
        let on_modal_close = on_delete_modal_close.clone();
        let fetch_entries = fetch_entries.clone();
        let error = error.clone();
        let transaction_id = transaction_to_delete.as_ref().map(|t| t.transaction_id);
        Callback::from(move |()| {
            if let Some(id) = transaction_id {
                let on_modal_close = on_modal_close.clone();
                let fetch_entries = fetch_entries.clone();
                let error = error.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let url = format!("/api/transactions/{}", id);
                    let resp = Request::delete(&url).send().await;
                    match resp {
                        Ok(r) if r.ok() => {
                            on_modal_close.emit(());
                            fetch_entries.emit(());
                        }
                        Ok(r) => {
                            error.set(Some(format!("Failed to delete transaction: {}", r.status())))
                        }
                        Err(e) => error.set(Some(format!("Network error: {}", e))),
                    }
                });
            }
        })
    };

    let on_reverse_click = {
        let transaction_to_reverse = transaction_to_reverse.clone();
        Callback::from(move |t| transaction_to_reverse.set(Some(t)))
    };

    let on_edit_click = {
        let navigator = navigator.clone();
        Callback::from(move |id: Uuid| {
            let query = NewTransactionQuery {
                edit_id: Some(id),
                ..Default::default()
            };
            navigator.push_with_query(&Route::NewTransaction, &query).unwrap();
        })
    };

    let on_delete_click = {
        let transaction_to_delete = transaction_to_delete.clone();
        Callback::from(move |t: JournalEntryWithBalance| {
            transaction_to_delete.set(Some(t));
        })
    };

    let account_name = account.as_ref().map(|a| a.name.clone()).unwrap_or_default();
    let query = NewTransactionQuery {
        from_account: Some(props.account_id),
        ..Default::default()
    };

    let transaction_groups = use_memo(entries.clone(), |entries| {
        let mut groups: HashMap<Uuid, TransactionGroup> = HashMap::new();
        for entry in entries.iter() {
            if entry.description != Some("Opening Balance".to_string()) {
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
    });

    let opening_balance_entry = entries.iter().find(|e| e.description == Some("Opening Balance".to_string()));

    html! {
        <Layout>
            <div class="report-header">
                <h3>{ format!("Ledger: {}", account_name) }</h3>
                <ReportOptions show_start_date={true} show_end_date={true} />
            </div>
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

            if let Some(jeb) = &*transaction_to_reverse { <ReversalConfirmationModal jeb={jeb.clone()} on_close={on_reverse_modal_close.clone()} on_confirm={on_reverse_confirm.clone()} /> }
            if let Some(jeb) = &*transaction_to_delete { <DeleteConfirmationModal jeb={jeb.clone()} on_close={on_delete_modal_close.clone()} on_confirm={on_delete_confirm.clone()} /> }

                <table class="report-table">
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
                        if let Some(entry) = opening_balance_entry {
                            <tr>
                                <td>{ &entry.date.to_string() }</td>
                                <td>{ "Opening Balance" }</td>
                                <td class="amount">{ if entry.debit > 0 { format_currency(&entry.debit) } else { "".to_string() } }</td>
                                <td class="amount">{ if entry.credit > 0 { format_currency(&entry.credit) } else { "".to_string() } }</td>
                                <td class="amount">{ format_currency(&entry.running_balance) }</td>
                                <td></td>
                            </tr>
                        }
                        { for transaction_groups.iter().map(|group| html! {
                            <TransactionRow
                                key={group.transaction_id.to_string()}
                                transaction_group={group.clone()}
                                on_reverse={on_reverse_click.clone()}
                                on_edit={on_edit_click.clone()}
                                on_delete={on_delete_click.clone()}
                                org_ctx={org_ctx.clone()}
                            />
                        })}
                    </tbody>
                </table>
            }
        </Layout>
    }
}
