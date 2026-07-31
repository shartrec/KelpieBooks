/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use fluent::fluent_args;
use gloo_net::http::Request;
use shared_core::ledger::dtos::{
    journal_entry_with_balance::JournalEntryWithBalance,
    transaction_detail::TransactionDetail,
};
use yew::prelude::*;

use crate::contexts::locale_context::use_locale;

#[derive(Properties, PartialEq)]
pub struct DeleteConfirmationModalProps {
    pub jeb: JournalEntryWithBalance,
    pub on_close: Callback<()>,
    pub on_confirm: Callback<()>,
}

#[function_component(DeleteConfirmationModal)]
pub fn delete_confirmation_modal(props: &DeleteConfirmationModalProps) -> Html {
    let i18n = use_locale();

    let transaction_detail = use_state(|| None::<TransactionDetail>);
    let error = use_state(|| None::<String>);

    {
        let transaction_detail = transaction_detail.clone();
        let error = error.clone();
        let transaction_id = props.jeb.transaction_id;
        use_effect_with(transaction_id, move |&id| {
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("/api/transactions/{}", id);
                let resp = Request::get(&url).send().await;
                match resp {
                    Ok(r) if r.ok() => match r.json::<TransactionDetail>().await {
                        Ok(detail) => transaction_detail.set(Some(detail)),
                        Err(e) => error.set(Some(i18n.t_args(
                            "transaction-error-parse",
                            &fluent_args!["error" => e.to_string()],
                        ))),
                    },
                    Ok(r) => error.set(Some(i18n.t_args(
                        "transaction-error-fetch",
                        &fluent_args!["status" => r.status()],
                    ))),
                    Err(e) => error.set(Some(i18n.t_args(
                        "common-network-error",
                        &fluent_args!["error" => e.to_string()],
                    ))),
                }
            });
            || ()
        });
    }

    let on_cancel = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| {
            on_close.emit(());
        })
    };

    let on_confirm_delete = {
        let on_confirm = props.on_confirm.clone();
        Callback::from(move |_| {
            on_confirm.emit(());
        })
    };

    let i18n = use_locale();

    html! {
        <div class="modal-overlay" onclick={on_cancel.clone()}>
            <div class="modal-content" onclick={|e: MouseEvent| e.stop_propagation()}>
                <h2>{ i18n.t("deletion-confirm-title") }</h2>

                if let Some(detail) = &*transaction_detail {
                    <div class="transaction-details-summary">
                        <p>
                            <strong>{ i18n.t("common-date") }</strong> { i18n.format_date(detail.transaction.date) }
                        </p>
                        if let Some(desc) = &detail.transaction.description {
                             <p><strong>{ i18n.t("reversal-confirm-original-description") }</strong> { desc }</p>
                        }
                        <table class="table min-width-400">
                            <thead>
                                <tr>
                                    <th>{ i18n.t("common-account") }</th>
                                    <th class="amount">{ i18n.t("common-debit") }</th>
                                    <th class="amount">{ i18n.t("common-credit") }</th>
                                </tr>
                            </thead>
                            <tbody>
                                { for detail.entries.iter().map(|entry| html! {
                                    <tr>
                                        <td>{ &entry.account_name }</td>
                                        <td class="amount">{ i18n.format_currency(entry.debit) }</td>
                                        <td class="amount">{ i18n.format_currency(entry.credit) }</td>
                                    </tr>
                                })}
                            </tbody>
                        </table>
                    </div>
                } else if let Some(err) = &*error {
                    <p class="message__error">{ err }</p>
                } else {
                    <p>{ i18n.t("transaction-row-loading-details") }</p>
                }

                <p class="warning-text">
                    { i18n.t("deletion-confirm-warning") }
                </p>
                <div class="form-actions">
                    <button type="button" onclick={on_cancel} class="button-secondary">{ i18n.t("common-cancel") }</button>
                    <button type="button" onclick={on_confirm_delete} class="button-danger">{ i18n.t("common-confirm-delete-button") }</button>
                </div>
            </div>
        </div>
    }
}
