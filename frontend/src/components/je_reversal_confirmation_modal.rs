/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use fluent::fluent_args;
use gloo_net::http::Request;
use shared_core::dtos::journal_entry_with_balance::JournalEntryWithBalance;
use shared_core::dtos::transaction_detail::TransactionDetail;
use shared_core::i18n::{t, t_args};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use crate::contexts::locale_context::use_locale;

#[derive(Properties, PartialEq)]
pub struct ReversalConfirmationModalProps {
    pub jeb: JournalEntryWithBalance,
    pub on_close: Callback<()>,
    pub on_confirm: Callback<String>,
}

#[function_component(ReversalConfirmationModal)]
pub fn reversal_confirmation_modal(props: &ReversalConfirmationModalProps) -> Html {
    let i18n = use_locale();
    let transaction_detail = use_state(|| None::<TransactionDetail>);
    let error = use_state(|| None::<String>);
    let description =
        use_state(|| format!("Rev Trans {}", &props.jeb.transaction_id.to_string()[..8]));

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
                        Err(e) => error.set(Some(t_args(
                            "transaction-error-parse",
                            &fluent_args!["error" => e.to_string()],
                        ))),
                    },
                    Ok(r) => {
                        error.set(Some(t_args(
                            "transaction-error-fetch",
                            &fluent_args!["status" => r.status()],
                        )))
                    }
                    Err(e) => error.set(Some(t_args(
                        "coa-error-network",
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

    let on_confirm_reverse = {
        let on_confirm = props.on_confirm.clone();
        let description = description.clone();
        Callback::from(move |_| {
            on_confirm.emit((*description).clone());
        })
    };

    let on_description_change = {
        let description = description.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            description.set(input.value());
        })
    };

    html! {
        <div class="modal-overlay" onclick={on_cancel.clone()}>
            <div class="modal-content" onclick={|e: MouseEvent| e.stop_propagation()}>
                <h2>{ t("reversal-confirm-title") }</h2>

                if let Some(detail) = &*transaction_detail {
                    <div class="transaction-details-summary">
                        <p>
                            <strong>{ t("common-date") }</strong> { i18n.format_date(detail.transaction.date) }
                        </p>
                        if let Some(desc) = &detail.transaction.description {
                             <p><strong>{ t("reversal-confirm-original-description") }</strong> { desc }</p>
                        }
                        <table class="table min-width-400">
                            <thead>
                                <tr>
                                    <th>{ t("common-account") }</th>
                                    <th class="amount">{ t("common-debit") }</th>
                                    <th class="amount">{ t("common-credit") }</th>
                                </tr>
                            </thead>
                            <tbody>
                                { for detail.entries.iter().map(|entry| html! {
                                    <tr>
                                        <td>{ &entry.account_name }</td>
                                        <td class="amount">{ if entry.debit != 0 { format!("{:.2}", entry.debit as f64 / 100.0) } else { "".to_string() } }</td>
                                        <td class="amount">{ if entry.credit != 0 { format!("{:.2}", entry.credit as f64 / 100.0) } else { "".to_string() } }</td>
                                    </tr>
                                })}
                            </tbody>
                        </table>
                    </div>
                } else if let Some(err) = &*error {
                    <p class="error">{ err }</p>
                } else {
                    <p>{ t("transaction-row-loading-details") }</p>
                }

                <div class="form-group">
                    <label for="reversal-description">{ t("reversal-confirm-reversal-description") }</label>
                    <input
                        id="reversal-description"
                        type="text"
                        value={(*description).clone()}
                        onchange={on_description_change}
                        class="form-control"
                    />
                </div>

                <p class="warning-text">
                    { t("reversal-confirm-warning") }
                </p>
                <div class="form-actions">
                    <button type="button" onclick={on_cancel} class="button-secondary">{ t("common-cancel") }</button>
                    <button type="button" onclick={on_confirm_reverse} class="button-danger">{ t("reversal-confirm-button") }</button>
                </div>
            </div>
        </div>
    }
}
