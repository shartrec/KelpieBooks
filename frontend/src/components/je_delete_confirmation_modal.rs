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
use gloo_net::http::Request;
use shared_core::dtos::journal_entry_with_balance::JournalEntryWithBalance;
use shared_core::dtos::transaction_detail::TransactionDetail;
use shared_core::util::format_currency;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DeleteConfirmationModalProps {
    pub jeb: JournalEntryWithBalance,
    pub on_close: Callback<()>,
    pub on_confirm: Callback<()>,
}

#[function_component(DeleteConfirmationModal)]
pub fn delete_confirmation_modal(props: &DeleteConfirmationModalProps) -> Html {
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
                        Err(e) => error.set(Some(format!("Failed to parse transaction: {}", e))),
                    },
                    Ok(r) => {
                        error.set(Some(format!("Failed to fetch transaction: {}", r.status())))
                    }
                    Err(e) => error.set(Some(format!("Network error: {}", e))),
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

    html! {
        <div class="modal-overlay" onclick={on_cancel.clone()}>
            <div class="modal-content" onclick={|e: MouseEvent| e.stop_propagation()}>
                <h2>{ "Confirm Transaction Deletion" }</h2>

                if let Some(detail) = &*transaction_detail {
                    <div class="transaction-details-summary">
                        <p>
                            <strong>{ "Date: " }</strong> { detail.transaction.date.format("%Y-%m-%d").to_string() }
                        </p>
                        if let Some(desc) = &detail.transaction.description {
                             <p><strong>{ "Original Description: " }</strong> { desc }</p>
                        }
                        <table class="table min-width-400">
                            <thead>
                                <tr>
                                    <th>{ "Account" }</th>
                                    <th class="amount">{ "Debit" }</th>
                                    <th class="amount">{ "Credit" }</th>
                                </tr>
                            </thead>
                            <tbody>
                                { for detail.entries.iter().map(|entry| html! {
                                    <tr>
                                        <td>{ &entry.account_name }</td>
                                        <td class="amount">{ format_currency(&entry.debit) }</td>
                                        <td class="amount">{ format_currency(&entry.credit) }</td>
                                    </tr>
                                })}
                            </tbody>
                        </table>
                    </div>
                } else if let Some(err) = &*error {
                    <p class="error">{ err }</p>
                } else {
                    <p>{ "Loading transaction details..." }</p>
                }

                <p class="warning-text">
                    { "This action cannot be undone. Deleting this transaction will permanently remove it from your records." }
                </p>
                <div class="form-actions">
                    <button type="button" onclick={on_cancel} class="button-secondary">{ "Cancel" }</button>
                    <button type="button" onclick={on_confirm_delete} class="button-danger">{ "Confirm Deletion" }</button>
                </div>
            </div>
        </div>
    }
}
