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

use fluent::fluent_args;
use shared_core::i18n::{t, t_args};
use shared_core::models::partner_contact::PartnerContact;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DeleteContactConfirmationModalProps {
    pub contact: PartnerContact,
    pub on_close: Callback<()>,
    pub on_confirm: Callback<()>,
}

#[function_component(DeleteContactConfirmationModal)]
pub fn delete_contact_confirmation_modal(props: &DeleteContactConfirmationModalProps) -> Html {
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
                <h2>{ t("common-confirm-deletion") }</h2>
                <p>
                    { t("delete-address-confirm-prefix") }
                    <strong>{ &props.contact.full_name }</strong>
                    { t("delete-address-confirm-suffix") }
                </p>
                <p class="warning-text">
                    { t("reversal-confirm-warning") }
                </p>
                <div class="modal__form__actions">
                    <button type="button" onclick={on_cancel} class="button-secondary">{ t("common-cancel") }</button>
                    <button type="button" onclick={on_confirm_delete} class="button-danger">{ t("common-confirm-delete-button") }</button>
                </div>
            </div>
        </div>
    }
}
