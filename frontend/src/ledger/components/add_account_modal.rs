/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use std::str::FromStr;

use shared_core::ledger::{
    models::account_category::AccountCategory,
    requests::account::CreateAccountRequest,
};
use uuid::Uuid;
use yew::prelude::*;

use crate::contexts::locale_context::use_locale;

#[derive(Properties, PartialEq)]
pub struct AddAccountModalProps {
    pub on_close: Callback<()>,
    pub on_submit: Callback<CreateAccountRequest>,
    pub parent_accounts: Vec<(Uuid, String)>, // Vec of (id, name) for the dropdown
}

#[function_component(AddAccountModal)]
pub fn add_account_modal(props: &AddAccountModalProps) -> Html {
    let i18n = use_locale();
    let request = use_state(CreateAccountRequest::default);
    let error = use_state(|| None::<String>);

    let on_code_input = {
        let state = request.clone();
        Callback::from(move |e: InputEvent| {
            let mut info = (*state).clone();
            info.code = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            state.set(info);
        })
    };
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
    let on_category_change = {
        let state = request.clone();
        Callback::from(move |e: Event| {
            let mut info = (*state).clone();
            info.category = AccountCategory::from_str(
                &e.target_unchecked_into::<web_sys::HtmlSelectElement>()
                    .value(),
            )
            .unwrap();
            state.set(info);
        })
    };
    let on_parent_change = {
        let state = request.clone();
        Callback::from(move |e: Event| {
            let mut info = (*state).clone();
            let val = e
                .target_unchecked_into::<web_sys::HtmlSelectElement>()
                .value();
            info.parent_id = if val.is_empty() {
                None
            } else {
                Uuid::from_str(&val).ok()
            };
            state.set(info);
        })
    };
    let on_is_group_change = {
        let state = request.clone();
        Callback::from(move |e: Event| {
            let mut info = (*state).clone();
            info.is_group = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .checked();
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
                <h2>{ i18n.t("account-modal-add-title") }</h2>
                <form onsubmit={on_form_submit} class="modal__form">
                    <label>{i18n.t("account-modal-code-label")}</label>
                    <input type="text" oninput={on_code_input} required=true />

                    <label>{i18n.t("account-modal-name-label")}</label>
                    <input type="text" oninput={on_name_input} required=true />

                    <label>{i18n.t("account-modal-category-label")}</label>
                    <select onchange={on_category_change}>
                        <option value="Asset" selected=true>{ i18n.t("account-category-asset") }</option>
                        <option value="Liability">{ i18n.t("account-category-liability") }</option>
                        <option value="Equity">{ i18n.t("account-category-equity") }</option>
                        <option value="Revenue">{ i18n.t("account-category-revenue") }</option>
                        <option value="Expense">{ i18n.t("account-category-expense") }</option>
                    </select>

                    <label>{i18n.t("account-modal-parent-label")}</label>
                    <select onchange={on_parent_change}>
                        <option value="" selected=true>{ i18n.t("account-modal-parent-none") }</option>
                        { for props.parent_accounts.iter().map(|(id, name)| html! {
                            <option value={id.to_string()}>{name}</option>
                        })}
                    </select>

                    <label>{i18n.t("account-modal-is-group-label")}</label>
                    <input type="checkbox" onchange={on_is_group_change} />

                    <div class="modal__form__actions">
                        <button type="button" onclick={on_cancel_click} class="button-secondary">{ i18n.t("common-cancel") }</button>
                        <button type="submit">{ i18n.t("account-modal-add-button") }</button>
                    </div>
                    if let Some(err) = &*error {
                        <div class="error">{ err }</div>
                    }
                </form>
            </div>
        </div>
    }
}
