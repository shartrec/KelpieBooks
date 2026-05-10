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

use shared_core::dtos::account_with_balance::AccountWithBalance;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DeleteConfirmationModalProps {
    pub account: AccountWithBalance,
    pub on_close: Callback<()>,
    pub on_confirm: Callback<()>,
}

#[function_component(DeleteConfirmationModal)]
pub fn delete_confirmation_modal(props: &DeleteConfirmationModalProps) -> Html {
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
                <h2>{ "Confirm Deletion" }</h2>
                <p>
                    { "Are you sure you want to delete the account: " }
                    <strong>{ &props.account.name }</strong>
                    { "?" }
                </p>
                <p class="warning-text">
                    { "This action cannot be undone. You can only delete accounts with no transactions." }
                </p>
                <div class="modal__form__actions">
                    <button type="button" onclick={on_cancel} class="button-secondary">{ "Cancel" }</button>
                    <button type="button" onclick={on_confirm_delete} class="button-danger">{ "Confirm Delete" }</button>
                </div>
            </div>
        </div>
    }
}
