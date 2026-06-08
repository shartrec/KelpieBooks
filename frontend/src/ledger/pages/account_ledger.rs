/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use std::{
    collections::HashMap,
    rc::Rc,
};

use fluent::fluent_args;
use shared_core::{
    ledger::{
        dtos::journal_entry_with_balance::JournalEntryWithBalance,
        models::account::Account,
        requests::transaction::ReverseTransactionRequest,
    },
    models::auth::SystemPrivilege,
};
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    api::Api,
    components::{
        layout::Layout,
        report_options::ReportOptions,
    },
    contexts::{
        auth_context::use_user_context,
        locale_context::use_locale,
        org_context::use_org_context,
        report_context::{
            use_report_context,
            ReportAction,
        },
    },
    ledger::{
        components::{
            je_delete_confirmation_modal::DeleteConfirmationModal,
            je_reversal_confirmation_modal::ReversalConfirmationModal,
            transaction_row::{
                TransactionGroup,
                TransactionRow,
            },
        },
        pages::new_transaction::NewTransactionQuery,
    },
    router::Route,
};

#[derive(Debug, Properties, PartialEq)]
pub struct AccountLedgerPageProps {
    pub account_id: Uuid,
}

#[function_component(AccountLedgerPage)]
pub fn account_ledger_page(props: &AccountLedgerPageProps) -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
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
        let user_ctx = user_ctx.clone();
        use_effect_with((report_ctx.date_range.clone(),), move |_| {
            if user_ctx.has_privilege(&SystemPrivilege::use_transactions) {
                let start_date = report_ctx.date_range.start_date;
                let end_date = report_ctx.date_range.end_date;
                report_ctx.dispatch(ReportAction::SetOnExportCsv(Some(Callback::from(
                    move |_| {
                        let url = format!(
                            "/api/accounts/{}/export/csv?start={}&end={}",
                            account_id, start_date, end_date
                        );
                        web_sys::window()
                            .unwrap()
                            .location()
                            .set_href(&url)
                            .unwrap();
                    },
                ))));
                report_ctx.dispatch(ReportAction::SetOnExportTypst(Some(Callback::from(
                    move |_| {
                        let url = format!(
                            "/api/accounts/{}/export/pdf?start={}&end={}",
                            account_id, start_date, end_date
                        );
                        web_sys::window()
                            .unwrap()
                            .location()
                            .set_href(&url)
                            .unwrap();
                    },
                ))));
            }
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
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        Callback::from(move |()| {
            let entries = entries.clone();
            let error = error.clone();
            let loading = loading.clone();
            let start_date = report_ctx.date_range.start_date;
            let end_date = report_ctx.date_range.end_date;
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                let entries_url = format!(
                    "/api/accounts/{}/entries?start={}&end={}",
                    account_id, start_date, end_date
                );
                let fetched_entries = Api::get(&entries_url, user_ctx, navigator).await;
                loading.set(false);
                match fetched_entries {
                    Ok(response) if response.ok() => {
                        match response.json::<Vec<JournalEntryWithBalance>>().await {
                            Ok(data) => entries.set(Rc::new(data)),
                            Err(e) => error.set(Some(i18n.t_args(
                                "ledger-error-parse-entries",
                                &fluent_args!["error" => e.to_string()],
                            ))),
                        }
                    }
                    Ok(response) => error.set(Some(i18n.t_args(
                        "ledger-error-fetch-entries",
                        &fluent_args!["status" => response.status()],
                    ))),
                    Err(e) => error.set(Some(
                        i18n.t_args("coa-error-network", &fluent_args!["error" => e.to_string()]),
                    )),
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
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        use_effect_with(
            (account_id, report_ctx.date_range.clone()),
            move |(account_id, _)| {
                let account_id = *account_id;
                let user_ctx = user_ctx.clone();
                let navigator = navigator.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let acc_url = format!("/api/accounts/{}", account_id);
                    if let Ok(response) = Api::get(&acc_url, user_ctx, navigator).await {
                        if let Ok(acc_data) = response.json::<Account>().await {
                            account.set(Some(acc_data));
                        }
                    }
                });
                fetch_entries.emit(());
                || ()
            },
        );
    }

    let on_reverse_confirm =
        {
            let on_modal_close = on_reverse_modal_close.clone();
            let fetch_entries = fetch_entries.clone();
            let error = error.clone();
            let transaction_id = transaction_to_reverse.as_ref().map(|t| t.transaction_id);
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            Callback::from(move |description: String| {
                if let Some(id) = transaction_id {
                    let on_modal_close = on_modal_close.clone();
                    let fetch_entries = fetch_entries.clone();
                    let error = error.clone();
                    let user_ctx = user_ctx.clone();
                    let i18n = i18n.clone();
                    let navigator = navigator.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        let url = format!("/api/transactions/{}/reverse", id);
                        let req_body = ReverseTransactionRequest { description };
                        let resp = Api::post(&url, &req_body, user_ctx, navigator).await;

                        match resp {
                            Ok(r) if r.ok() => {
                                on_modal_close.emit(());
                                fetch_entries.emit(());
                            }
                            Ok(r) => error.set(Some(i18n.t_args(
                                "ledger-error-reverse-transaction",
                                &fluent_args!["status" => r.status()],
                            ))),
                            Err(e) => error.set(Some(i18n.t_args(
                                "coa-error-network",
                                &fluent_args!["error" => e.to_string()],
                            ))),
                        }
                    });
                }
            })
        };

    let on_delete_confirm =
        {
            let on_modal_close = on_delete_modal_close.clone();
            let fetch_entries = fetch_entries.clone();
            let error = error.clone();
            let transaction_id = transaction_to_delete.as_ref().map(|t| t.transaction_id);
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            Callback::from(move |()| {
                if let Some(id) = transaction_id {
                    let on_modal_close = on_modal_close.clone();
                    let fetch_entries = fetch_entries.clone();
                    let error = error.clone();
                    let user_ctx = user_ctx.clone();
                    let i18n = i18n.clone();
                    let navigator = navigator.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        let url = format!("/api/transactions/{}", id);
                        let resp = Api::delete(&url, user_ctx, navigator).await;
                        match resp {
                            Ok(r) if r.ok() => {
                                on_modal_close.emit(());
                                fetch_entries.emit(());
                            }
                            Ok(r) => error.set(Some(i18n.t_args(
                                "ledger-error-delete-transaction",
                                &fluent_args!["status" => r.status()],
                            ))),
                            Err(e) => error.set(Some(i18n.t_args(
                                "coa-error-network",
                                &fluent_args!["error" => e.to_string()],
                            ))),
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
            navigator
                .push_with_query(&Route::NewTransaction, &query)
                .unwrap();
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
            if entry.description != Some(i18n.t("ledger-opening-balance")) {
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

    let opening_balance_entry = entries
        .iter()
        .find(|e| e.description == Some(i18n.t("ledger-opening-balance")));

    html! {
        <Layout>
            <div class="report-header">
                <h3>{ i18n.t_args("ledger-title", &fluent_args!["name" => account_name]) }</h3>
                <ReportOptions show_start_date={true} show_end_date={true} />
            </div>
            <div class="table-actions">
                { if user_ctx.has_privilege(&SystemPrivilege::manage_transactions) {
                    html! {
                        <Link<Route, NewTransactionQuery> to={Route::NewTransaction} query={query} classes="button">
                            { i18n.t("ledger-add-transaction-button") }
                        </Link<Route, NewTransactionQuery>>
                    }
                } else {
                    html! {}
                }}
            </div>
            if *loading {
                <p>{ i18n.t("common-loading") }</p>
            } else if let Some(err) = &*error {
                <div class="error">{ err }</div>
            } else {

            if let Some(jeb) = &*transaction_to_reverse { <ReversalConfirmationModal jeb={jeb.clone()} on_close={on_reverse_modal_close.clone()} on_confirm={on_reverse_confirm.clone()} /> }
            if let Some(jeb) = &*transaction_to_delete { <DeleteConfirmationModal jeb={jeb.clone()} on_close={on_delete_modal_close.clone()} on_confirm={on_delete_confirm.clone()} /> }

                <table class="report-table">
                    <thead>
                        <tr>
                            <th class="table__text-col">{ i18n.t("common-date") }</th>
                            <th class="table__text-col">{ i18n.t("common-description") }</th>
                            <th class="table__value-col">{ i18n.t("common-debit") }</th>
                            <th class="table__value-col">{ i18n.t("common-credit") }</th>
                            <th class="table__value-col">{ i18n.t("common-balance") }</th>
                            <th class="table__col-actions"></th>
                        </tr>
                    </thead>
                    <tbody>
                        if let Some(entry) = opening_balance_entry {
                            <tr>
                                <td>{ i18n.format_date(entry.date) }</td>
                                <td>{ i18n.t("ledger-opening-balance") }</td>
                                <td class="table__value-col">{ if entry.debit > 0 { i18n.format_currency(entry.debit) } else { "".to_string() } }</td>
                                <td class="table__value-col">{ if entry.credit > 0 { i18n.format_currency(entry.credit) } else { "".to_string() } }</td>
                                <td class="table__value-col">{ i18n.format_currency(entry.running_balance) }</td>
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
