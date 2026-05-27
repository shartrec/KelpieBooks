/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::contexts::locale_context::use_locale;
use shared_core::dtos::account_with_balance::AccountWithBalance;
use shared_core::models::account_category::AccountCategory;
use shared_core::requests::account::UpdateAccountRequest;
use std::str::FromStr;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EditAccountModalProps {
    pub account: AccountWithBalance,
    pub on_close: Callback<()>,
    pub on_submit: Callback<UpdateAccountRequest>,
}

#[function_component(EditAccountModal)]
pub fn edit_account_modal(props: &EditAccountModalProps) -> Html {
    let i18n = use_locale();

    let request = use_state(|| UpdateAccountRequest {
        name: props.account.name.clone(),
        code: props.account.code.clone(),
        category: props.account.category,
        is_group: props.account.is_group,
        is_bank_account: props.account.is_bank_account,
        system_tag: props.account.system_tag,
    });

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

    let on_is_bank_account_change = {
        let state = request.clone();
        Callback::from(move |e: Event| {
            let mut info = (*state).clone();
            info.is_bank_account = e
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

    let on_cancel = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| {
            on_close.emit(());
        })
    };

    html! {
        <div class="modal-overlay" onclick={on_cancel.clone()}>
            <div class="modal-content" onclick={|e: MouseEvent| e.stop_propagation()}>
                <h2>{ i18n.t("account-modal-edit-title") }</h2>
                <form onsubmit={on_form_submit} class="modal__form">
                    <label>{i18n.t("account-modal-code-label")}</label>
                    <input type="text" value={request.code.clone()} oninput={on_code_input} required=true />

                    <label>{i18n.t("account-modal-name-label")}</label>
                    <input type="text" value={request.name.clone()} oninput={on_name_input} required=true />

                    <label>{i18n.t("account-modal-category-label")}</label>
                    <select onchange={on_category_change}>
                        <option value="Asset" selected={request.category == AccountCategory::Asset}>{ i18n.t("account-category-asset") }</option>
                        <option value="Liability" selected={request.category == AccountCategory::Liability}>{ i18n.t("account-category-liability") }</option>
                        <option value="Equity" selected={request.category == AccountCategory::Equity}>{ i18n.t("account-category-equity") }</option>
                        <option value="Revenue" selected={request.category == AccountCategory::Revenue}>{ i18n.t("account-category-revenue") }</option>
                        <option value="Expense" selected={request.category == AccountCategory::Expense}>{ i18n.t("account-category-expense") }</option>
                    </select>

                    <label>{i18n.t("account-modal-is-group-label")}</label>
                    <input type="checkbox" checked={request.is_group} onchange={on_is_group_change} />

                    <label>{i18n.t("account-modal-is-bank-account-label")}</label>
                    <input type="checkbox" checked={request.is_bank_account} onchange={on_is_bank_account_change} />

                    <div class="modal__form__actions">
                        <button type="button" onclick={on_cancel} class="button-secondary">{ i18n.t("common-cancel") }</button>
                        <button type="submit">{ i18n.t("account-modal-save-button") }</button>
                    </div>
                </form>
            </div>
        </div>
    }
}
