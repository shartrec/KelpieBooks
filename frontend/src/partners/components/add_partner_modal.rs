/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use shared_core::partners::requests::partner::CreatePartnerRequest;
use uuid::Uuid;
use yew::prelude::*;

use crate::contexts::locale_context::use_locale;

#[derive(Properties, PartialEq)]
pub struct AddPartnerModalProps {
    pub on_close: Callback<()>,
    pub on_submit: Callback<CreatePartnerRequest>,
    pub ap_accounts: Vec<(Uuid, String)>,
    pub ar_accounts: Vec<(Uuid, String)>,
}

#[function_component(AddPartnerModal)]
pub fn add_partner_modal(props: &AddPartnerModalProps) -> Html {
    let i18n = use_locale();
    let request = use_state(CreatePartnerRequest::default);
    let error = use_state(|| None::<String>);

    let on_legal_name_input = {
        let state = request.clone();
        Callback::from(move |e: InputEvent| {
            let mut info = (*state).clone();
            info.legal_name = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            state.set(info);
        })
    };

    let on_trade_name_input = {
        let state = request.clone();
        Callback::from(move |e: InputEvent| {
            let mut info = (*state).clone();
            info.trade_name = Some(
                e.target_unchecked_into::<web_sys::HtmlInputElement>()
                    .value(),
            );
            state.set(info);
        })
    };

    let on_tax_identifier_input = {
        let state = request.clone();
        Callback::from(move |e: InputEvent| {
            let mut info = (*state).clone();
            info.tax_identifier = Some(
                e.target_unchecked_into::<web_sys::HtmlInputElement>()
                    .value(),
            );
            state.set(info);
        })
    };

    let on_is_vendor_change = {
        let state = request.clone();
        Callback::from(move |e: Event| {
            let mut info = (*state).clone();
            info.is_vendor = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .checked();
            state.set(info);
        })
    };

    let on_is_customer_change = {
        let state = request.clone();
        Callback::from(move |e: Event| {
            let mut info = (*state).clone();
            info.is_customer = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .checked();
            state.set(info);
        })
    };

    let on_ap_account_change = {
        let state = request.clone();
        Callback::from(move |e: Event| {
            let mut info = (*state).clone();
            let value = e
                .target_unchecked_into::<web_sys::HtmlSelectElement>()
                .value();
            info.default_ap_account_id = Uuid::parse_str(&value).ok();
            state.set(info);
        })
    };

    let on_ar_account_change = {
        let state = request.clone();
        Callback::from(move |e: Event| {
            let mut info = (*state).clone();
            let value = e
                .target_unchecked_into::<web_sys::HtmlSelectElement>()
                .value();
            info.default_ar_account_id = Uuid::parse_str(&value).ok();
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
                <h2>{ i18n.t("add-partner-title") }</h2>
                <form onsubmit={on_form_submit} class="modal__form">
                    <label>{i18n.t("add-partner-legal-name-label")}</label>
                    <input type="text" oninput={on_legal_name_input} required=true />

                    <label>{i18n.t("add-partner-trade-name-label")}</label>
                    <input type="text" oninput={on_trade_name_input} />

                    <label>{i18n.t("add-partner-tax-identifier-label")}</label>
                    <input type="text" oninput={on_tax_identifier_input} />

                    <label>{i18n.t("add-partner-is-vendor-label")}</label>
                    <input type="checkbox" onchange={on_is_vendor_change} />

                    <label>{i18n.t("add-partner-is-customer-label")}</label>
                    <input type="checkbox" onchange={on_is_customer_change} />

                    <label>{i18n.t("add-partner-default-ap-account-label")}</label>
                    <select onchange={on_ap_account_change}>
                        <option value="" selected=true>{ i18n.t("common-none") }</option>
                        { for props.ap_accounts.iter().map(|(id, name)| html! {
                            <option value={id.to_string()}>{name}</option>
                        })}
                    </select>

                    <label>{i18n.t("add-partner-default-ar-account-label")}</label>
                    <select onchange={on_ar_account_change}>
                        <option value="" selected=true>{ i18n.t("common-none") }</option>
                        { for props.ar_accounts.iter().map(|(id, name)| html! {
                            <option value={id.to_string()}>{name}</option>
                        })}
                    </select>

                    <div class="modal__form__actions">
                        <button type="button" onclick={on_cancel_click} class="button-secondary">{ i18n.t("common-cancel") }</button>
                        <button type="submit">{ i18n.t("partner-list-add-partner-button") }</button>
                    </div>
                    if let Some(err) = &*error {
                        <div class="message__error">{ err }</div>
                    }
                </form>
            </div>
        </div>
    }
}
