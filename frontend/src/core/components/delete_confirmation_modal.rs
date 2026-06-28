/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use yew::prelude::*;

use crate::contexts::locale_context::use_locale;

#[derive(Properties, PartialEq)]
pub struct DeleteConfirmationModalProps {
    pub title: String,
    pub message: String,
    #[prop_or_else(|| None)]
    pub warning: Option<String>,
    pub on_confirm: Callback<()>,
    pub on_cancel: Callback<()>,
}

#[function_component(DeleteConfirmationModal)]
pub fn delete_confirmation_modal(props: &DeleteConfirmationModalProps) -> Html {
    let i18n = use_locale();

    let on_cancel = {
        let on_cancel = props.on_cancel.clone();
        Callback::from(move |_| {
            on_cancel.emit(());
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
                <h2>{ &props.title }</h2>
                <p>{ &props.message }</p>
                {
                    if let Some(warning) = &props.warning {
                       html!{ <p>{ warning }</p> }
                    } else {
                       html! {<></>}
                    }
                }
                <div class="modal__form__actions">
                    <button type="button" onclick={on_cancel} class="button-secondary">{ i18n.t("common-cancel") }</button>
                    <button type="button" onclick={on_confirm_delete} class="button-danger">{ i18n.t("common-confirm-delete-button") }</button>
                </div>
            </div>
        </div>
    }
}
