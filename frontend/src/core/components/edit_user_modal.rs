/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use std::str::FromStr;

use shared_core::core::{
    dtos::user_detail::UserDetail,
    models::role::Role,
    requests::user::UpdateUserRequest,
};
use uuid::Uuid;
use yew::prelude::*;

use crate::contexts::locale_context::use_locale;

#[derive(Properties, PartialEq)]
pub struct EditUserModalProps {
    pub user: UserDetail,
    pub roles: Vec<Role>,
    pub on_close: Callback<()>,
    pub on_submit: Callback<(Uuid, UpdateUserRequest)>,
}

#[function_component(EditUserModal)]
pub fn edit_user_modal(props: &EditUserModalProps) -> Html {
    let i18n = use_locale();
    let request = use_state(|| {
        let role_id = props
            .roles
            .iter()
            .find(|r| Some(r.name.clone()) == props.user.role)
            .map(|r| r.id);
        UpdateUserRequest {
            email: props.user.email.clone(),
            full_name: props.user.full_name.clone(),
            display_name: props.user.display_name.clone(),
            role_id,
        }
    });
    let error = use_state(|| None::<String>);

    let on_email_input = {
        let state = request.clone();
        Callback::from(move |e: InputEvent| {
            let mut info = (*state).clone();
            info.email = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            state.set(info);
        })
    };

    let on_full_name_input = {
        let state = request.clone();
        Callback::from(move |e: InputEvent| {
            let mut info = (*state).clone();
            info.full_name = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            state.set(info);
        })
    };

    let on_display_name_input = {
        let state = request.clone();
        Callback::from(move |e: InputEvent| {
            let mut info = (*state).clone();
            info.display_name = Some(
                e.target_unchecked_into::<web_sys::HtmlInputElement>()
                    .value(),
            );
            state.set(info);
        })
    };

    let on_role_change = {
        let state = request.clone();
        Callback::from(move |e: Event| {
            let mut info = (*state).clone();
            let val = e
                .target_unchecked_into::<web_sys::HtmlSelectElement>()
                .value();
            info.role_id = Uuid::from_str(&val).ok();
            state.set(info);
        })
    };

    let on_form_submit = {
        let on_submit = props.on_submit.clone();
        let request = request.clone();
        let user_id = props.user.id;
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            on_submit.emit((user_id, (*request).clone()));
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
                <h2>{ i18n.t("user-modal-edit-title") }</h2>
                <form onsubmit={on_form_submit} class="modal__form">
                    <label>{i18n.t("user-modal-email-label")}</label>
                    <input type="email" value={request.email.clone()} oninput={on_email_input} required=true />

                    <label>{i18n.t("user-modal-full-name-label")}</label>
                    <input type="text" value={request.full_name.clone()} oninput={on_full_name_input} required=true />

                    <label>{i18n.t("user-modal-display-name-label")}</label>
                    <input type="text" value={request.display_name.clone().unwrap_or_default()} oninput={on_display_name_input} />

                    <label>{i18n.t("user-modal-role-label")}</label>
                    <select onchange={on_role_change}>
                        <option value="" selected={request.role_id.is_none()}>{ i18n.t("user-modal-select-role") }</option>
                        { for props.roles.iter().map(|role| html! {
                            <option value={role.id.to_string()} selected={request.role_id == Some(role.id)}>{role.name.clone()}</option>
                        })}
                    </select>

                    <div class="modal__form__actions">
                        <button type="button" onclick={on_cancel_click} class="button-secondary">{ i18n.t("common-cancel") }</button>
                        <button type="submit">{ i18n.t("user-modal-save-button") }</button>
                    </div>
                    if let Some(err) = &*error {
                        <div class="error">{ err }</div>
                    }
                </form>
            </div>
        </div>
    }
}
