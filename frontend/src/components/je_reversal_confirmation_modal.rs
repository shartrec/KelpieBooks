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
use shared_core::dtos::transaction_detail::TransactionDetail;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ReversalConfirmationModalProps {
    pub transaction: TransactionDetail,
    pub on_close: Callback<()>,
    pub on_confirm: Callback<()>,
}

#[function_component(ReversalConfirmationModal)]
pub fn reversal_confirmation_modal(props: &ReversalConfirmationModalProps) -> Html {
    let on_cancel = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| {
            on_close.emit(());
        })
    };

    let on_confirm_reverse = {
        let on_confirm = props.on_confirm.clone();
        Callback::from(move |_| {
            on_confirm.emit(());
        })
    };

    html! {
        <div class="modal-overlay" onclick={on_cancel.clone()}>
            <div class="modal-content" onclick={|e: MouseEvent| e.stop_propagation()}>
                <h2>{ "Confirm Transaction Reversal" }</h2>
                <p>
                    //TODO more details to be added here
                    { "Are you sure you want to reverse transaction: " }
                    <strong>{ &props.transaction.transaction.date.format("%Y-%m-%d").to_string() }</strong>
                    { "?" }
                </p>
                <p class="warning-text">
                    { "This action cannot be undone." }
                </p>
                <div class="form-actions">
                    <button type="button" onclick={on_cancel} class="button-secondary">{ "Cancel" }</button>
                    <button type="button" onclick={on_confirm_reverse} class="button-danger">{ "Confirm Reversal" }</button>
                </div>
            </div>
        </div>
    }
}
