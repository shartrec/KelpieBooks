/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use shared_core::core::{
    models::auth::SystemPrivilege,
    requests::role::CreateRoleRequest,
};
use yew::prelude::*;

use crate::contexts::locale_context::use_locale;

#[derive(Properties, PartialEq)]
pub struct AddRoleModalProps {
    pub on_close: Callback<()>,
    pub on_submit: Callback<CreateRoleRequest>,
}

#[function_component(AddRoleModal)]
pub fn add_role_modal(props: &AddRoleModalProps) -> Html {
    let i18n = use_locale();
    let request = use_state(CreateRoleRequest::default);
    let error = use_state(|| None::<String>);

    let on_name_input = {
        let state = request.clone();
        Callback::from(move |e: InputEvent| {
            let mut info = (*state).clone();
            info.name = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            state.set(info);
        })
    };

    let on_privilege_change = {
        let state = request.clone();
        Callback::from(move |(privilege, checked)| {
            let mut info = (*state).clone();
            if checked {
                info.privileges.push(privilege);
            } else {
                info.privileges.retain(|p| *p != privilege);
            }
            state.set(info);
        })
    };

    let on_form_submit = {
        let on_submit = props.on_submit.clone();
        let request = request.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            on_submit.emit((*request).clone());
        })
    };

    let on_overlay_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |_e: MouseEvent| {
            on_close.emit(());
        })
    };

    let on_cancel_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |_e: MouseEvent| {
            on_close.emit(());
        })
    };

    html! {
        <div class="modal-overlay" onclick={on_overlay_click}>
            <div class="modal-content" onclick={|e: MouseEvent| e.stop_propagation()}>
                <h2>{ i18n.t("role-modal-add-title") }</h2>
                <form onsubmit={on_form_submit} class="modal__form">
                    <label>{i18n.t("role-modal-name-label")}</label>
                    <input type="text" oninput={on_name_input} required=true />

                    <label>{i18n.t("role-modal-privileges-label")}</label>
                    <div class="privileges-grid">
                        { for SystemPrivilege::iterator().map(|privilege| {
                            let on_privilege_change = on_privilege_change.clone();
                            let privilege_clone = privilege.clone();
                            html! {
                                <div class="privilege-checkbox">
                                    <input
                                        type="checkbox"
                                        id={privilege.name_key()}
                                        onchange={move |e: Event| on_privilege_change.emit((privilege_clone, e.target_unchecked_into::<web_sys::HtmlInputElement>().checked()))}
                                    />
                                    <label for={privilege.name_key()}>{i18n.t(&privilege.name_key())}</label>
                                </div>
                            }
                        })}
                    </div>

                    <div class="modal__form__actions">
                        <button type="button" onclick={on_cancel_click} class="button-secondary">{ i18n.t("common-cancel") }</button>
                        <button type="submit">{ i18n.t("role-modal-add-button") }</button>
                    </div>
                    if let Some(err) = &*error {
                        <div class="error">{ err }</div>
                    }
                </form>
            </div>
        </div>
    }
}
