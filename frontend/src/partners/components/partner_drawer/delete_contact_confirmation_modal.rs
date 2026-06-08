/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use shared_core::partners::models::partner_contact::PartnerContact;
use yew::prelude::*;

use crate::contexts::locale_context::use_locale;

#[derive(Properties, PartialEq)]
pub struct DeleteContactConfirmationModalProps {
    pub contact: PartnerContact,
    pub on_close: Callback<()>,
    pub on_confirm: Callback<()>,
}

#[function_component(DeleteContactConfirmationModal)]
pub fn delete_contact_confirmation_modal(props: &DeleteContactConfirmationModalProps) -> Html {
    let i18n = use_locale();

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
                <h2>{ i18n.t("common-confirm-deletion") }</h2>
                <p>
                    { i18n.t("delete-address-confirm-prefix") }
                    <strong>{ &props.contact.full_name }</strong>
                    { i18n.t("delete-address-confirm-suffix") }
                </p>
                <p class="warning-text">
                    { i18n.t("reversal-confirm-warning") }
                </p>
                <div class="modal__form__actions">
                    <button type="button" onclick={on_cancel} class="button-secondary">{ i18n.t("common-cancel") }</button>
                    <button type="button" onclick={on_confirm_delete} class="button-danger">{ i18n.t("common-confirm-delete-button") }</button>
                </div>
            </div>
        </div>
    }
}
