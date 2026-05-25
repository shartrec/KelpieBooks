/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use fluent::fluent_args;
use shared_core::dtos::partner_list_item::PartnerListItem;
use shared_core::i18n::{t, t_args};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DeletePartnerConfirmationModalProps {
    pub partner: PartnerListItem,
    pub on_close: Callback<()>,
    pub on_confirm: Callback<()>,
}

#[function_component(DeletePartnerConfirmationModal)]
pub fn delete_partner_confirmation_modal(props: &DeletePartnerConfirmationModalProps) -> Html {
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
                    { t_args("delete-partner-confirm-message", &fluent_args!["name" => props.partner.legal_name.clone()]) }
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
