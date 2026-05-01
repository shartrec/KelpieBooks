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
use crate::utils::csv::download_csv;
use crate::utils::typst::download_typst;
use crate::Route;
use gloo_net::http::Request;
use log::info;
use shared_core::dtos::journal_entry_with_balance::JournalEntryWithBalance;
use shared_core::requests::transaction::ReverseTransactionRequest;
use shared_core::models::{Account, AccountCategory};
use std::collections::HashMap;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::je_reversal_confirmation_modal::ReversalConfirmationModal;

#[derive(Debug, Properties, PartialEq)]
pub struct AccountLedgerPageProps {
    pub account_id: Uuid,
}

#[function_component(AccountLedgerPage)]
pub fn account_ledger_page(props: &AccountLedgerPageProps) -> Html {
    let report_ctx = use_report_context();
    let entries = use_state(|| Vec::<JournalEntryWithBalance>::new());
    let account = use_state(|| None::<Account>);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);
    let transaction_to_reverse = use_state(|| None::<JournalEntryWithBalance>);

    {
        let report_ctx = report_ctx.clone();
        let entries = entries.clone();
        use_effect_with(entries.clone(), move |_| {
            let entries1 = entries.clone();
            let on_export_csv = Callback::from(move |_| {
                let mut csv_content = String::new();
                csv_content.push_str("Date,Description,Debit,Credit,Balance\n");
                for entry in entries1.iter() {
                    let debit = if entry.debit > 0 { (entry.debit as f64) / 100.0 } else { 0.0 };
                    let credit = if entry.credit > 0 { (entry.credit as f64) / 100.0 } else { 0.0 };
                    let balance = ((entry.debit - entry.credit) as f64) / 100.0;
                    csv_content.push_str(&format!("{},\"{}\",{},{},{}\n", entry.date, entry.description.clone().unwrap_or("".to_string()), debit, credit, balance));
                }
                if let Err(e) = download_csv("account_ledger.csv", &csv_content) {
                    gloo_console::error!("Failed to download CSV:", e);
                }
            });

            let on_export_typst = Callback::from(move |_| {
                let mut typst_content = String::new();
                typst_content.push_str("#set text(size: 10pt)\n");
                typst_content.push_str("#set page(margin: (top: 2cm, bottom: 2cm, left: 1.5cm, right: 1.5cm))\n\n");
                typst_content.push_str("= Account Ledger\n\n");
                typst_content.push_str("#table(\n");
                typst_content.push_str("  columns: (auto, 1fr, 1fr, 1fr, 1fr),\n");
                typst_content.push_str("  [*Date*], [*Description*], [*Debit*], [*Credit*], [*Balance*],\n");
                for entry in entries.iter() {
                    let debit = if entry.debit > 0 { format!("{:.2}", (entry.debit as f64) / 100.0) } else { "".to_string() };
                    let credit = if entry.credit > 0 { format!("{:.2}", (entry.credit as f64) / 100.0) } else { "".to_string() };
                    let balance = format!("{:.2}", ((entry.debit - entry.credit) as f64) / 100.0);
                    typst_content.push_str(&format!("  \"{}\", \"{}\", align(right)[{}], align(right)[{}], align(right)[{}],\n", entry.date, entry.description.clone().unwrap_or("".to_string()), debit, credit, balance));
                }
                typst_content.push_str(")\n");

                if let Err(e) = download_typst("account_ledger.typ", &typst_content) {
                    gloo_console::error!("Failed to download Typst file:", e);
                }
            });

            report_ctx.dispatch(ReportAction::SetOnExportCsv(Some(on_export_csv)));
            report_ctx.dispatch(ReportAction::SetOnExportTypst(Some(on_export_typst)));
            move || {
                report_ctx.dispatch(ReportAction::SetOnExportCsv(None));
                report_ctx.dispatch(ReportAction::SetOnExportTypst(None));
            }
        });
    }

    info!("Account Ledger Props {:?}", props);

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

    let on_modal_close = {
        let transaction_to_reverse = transaction_to_reverse.clone();
        Callback::from(move |_: ()| {
            transaction_to_reverse.set(None);
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

    let on_reverse_confirm = {
        let on_modal_close = on_modal_close.clone();
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
    let on_reverse_click = {
        let transaction_to_reverse = transaction_to_reverse.clone();
        Callback::from(move |t| transaction_to_reverse.set(Some(t)))
    };


    let account_name = account.as_ref().map(|a| a.name.clone()).unwrap_or_default();
    let query = NewTransactionQuery {
        from_account: Some(props.account_id),
        ..Default::default()
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
                if let Some(acc) = &*account {
                    if acc.category == AccountCategory::Expense {
                        <Link<Route> to={Route::ProfitLoss} classes="button button-secondary">
                            { "View this account in P&L" }
                        </Link<Route>>
                    }
                }
            </div>
            if *loading {
                <p>{ "Loading..." }</p>
            } else if let Some(err) = &*error {
                <div class="error">{ err }</div>
            } else {

            if let Some(jeb) = &*transaction_to_reverse { <ReversalConfirmationModal jeb={jeb.clone()} on_close={on_modal_close.clone()} on_confirm={on_reverse_confirm.clone()} /> }

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
                                on_reverse={on_reverse_click.clone()}
                            />
                        })}
                    </tbody>
                </table>
            }
        </Layout>
    }
}
