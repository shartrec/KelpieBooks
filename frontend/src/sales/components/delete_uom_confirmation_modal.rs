/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use yew::prelude::*;
use shared_core::sales::models::item::UnitOfMeasure;
use crate::api::Api;
use crate::contexts::auth_context::use_user_context;
use crate::contexts::locale_context::use_locale;
use yew_router::prelude::use_navigator;
use fluent::fluent_args;

#[derive(Properties, PartialEq)]
pub struct DeleteUomConfirmationModalProps {
    pub uom: UnitOfMeasure,
    pub on_close: Callback<()>,
    pub on_confirm: Callback<()>,
}

#[function_component(DeleteUomConfirmationModal)]
pub fn delete_uom_confirmation_modal(props: &DeleteUomConfirmationModalProps) -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();
    let error = use_state(|| None::<String>);

    let on_confirm = {
        let on_confirm = props.on_confirm.clone();
        let uom_id = props.uom.id;
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        let error = error.clone();
        let i18n = i18n.clone();

        Callback::from(move |_| {
            let on_confirm = on_confirm.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            let error = error.clone();
            let i18n = i18n.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let resp = Api::delete(&format!("/api/sales/uoms/{}", uom_id), user_ctx, navigator).await;
                if resp.is_ok() {
                    on_confirm.emit(());
                } else {
                    error.set(Some(i18n.t("uom-delete-error")));
                }
            });
        })
    };

    let on_cancel = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| {
            on_close.emit(());
        })
    };

    html! {
        <div class="modal-overlay" onclick={on_cancel.clone()}>
            <div class="modal-content" onclick={|e: MouseEvent| e.stop_propagation()}>
                <h2>{ i18n.t("uom-delete-title") }</h2>
                <p>{ i18n.t_args("uom-delete-confirm-message", &fluent_args!["name" => props.uom.name.clone()]) }</p>
                if let Some(err) = &*error {
                    <div class="error">{ err }</div>
                }
                <div class="modal__form__actions">
                    <button type="button" onclick={on_cancel} class="button-secondary">{ i18n.t("common-cancel") }</button>
                    <button type="button" onclick={on_confirm} class="button-danger">{ i18n.t("common-delete") }</button>
                </div>
            </div>
        </div>
    }
}
