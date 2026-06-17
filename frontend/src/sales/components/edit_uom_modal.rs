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

#[derive(Properties, PartialEq)]
pub struct EditUomModalProps {
    pub uom: UnitOfMeasure,
    pub on_close: Callback<()>,
    pub on_submit: Callback<()>,
}

#[function_component(EditUomModal)]
pub fn edit_uom_modal(props: &EditUomModalProps) -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();
    let request = use_state(|| props.uom.clone());
    let error = use_state(|| None::<String>);

    let on_input = |field_updater: fn(&mut UnitOfMeasure, String)| {
        let state = request.clone();
        Callback::from(move |e: InputEvent| {
            let mut info = (*state).clone();
            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
            field_updater(&mut info, value);
            state.set(info);
        })
    };

    let on_is_active_change = {
        let state = request.clone();
        Callback::from(move |e: Event| {
            let mut info = (*state).clone();
            info.is_active = e.target_unchecked_into::<web_sys::HtmlInputElement>().checked();
            state.set(info);
        })
    };

    let on_form_submit = {
        let on_submit = props.on_submit.clone();
        let request = request.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let on_submit = on_submit.clone();
            let request = request.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Api::put(&format!("/api/sales/uoms/{}", request.id), &*request, user_ctx, navigator).await;
                if resp.is_ok() {
                    on_submit.emit(());
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
                <h2>{ i18n.t("uom-edit-title") }</h2>
                <form onsubmit={on_form_submit} class="modal__form">
                    <label>{i18n.t("uom-code-label")}</label>
                    <input type="text" value={request.code.clone()} oninput={on_input(|r, v| r.code = v)} required=true />

                    <label>{i18n.t("uom-name-label")}</label>
                    <input type="text" value={request.name.clone()} oninput={on_input(|r, v| r.name = v)} required=true />

                    <label>{i18n.t("uom-is-active-label")}</label>
                    <input type="checkbox" checked={request.is_active} onchange={on_is_active_change} />

                    <div class="modal__form__actions">
                        <button type="button" onclick={on_cancel} class="button-secondary">{ i18n.t("common-cancel") }</button>
                        <button type="submit">{ i18n.t("common-save") }</button>
                    </div>
                    if let Some(err) = &*error {
                        <div class="error">{ err }</div>
                    }
                </form>
            </div>
        </div>
    }
}
