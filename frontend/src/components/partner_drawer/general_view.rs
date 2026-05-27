/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::contexts::locale_context::use_locale;
use gloo_timers::callback::Timeout;
use shared_core::models::partner::Partner;
use shared_core::requests::partner::UpdatePartnerRequest;
use uuid::Uuid;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GeneralViewProps {
    pub partner: Partner,
    pub on_submit: Callback<UpdatePartnerRequest>,
    pub ap_accounts: Vec<(Uuid, String)>,
    pub ar_accounts: Vec<(Uuid, String)>,
}

#[function_component(GeneralView)]
pub fn general_view(props: &GeneralViewProps) -> Html {
    let i18n = use_locale();

    let request = use_state(|| UpdatePartnerRequest {
        legal_name: props.partner.legal_name.clone(),
        trade_name: props.partner.trade_name.clone(),
        tax_identifier: props.partner.tax_identifier.clone(),
        is_vendor: props.partner.is_vendor,
        is_customer: props.partner.is_customer,
        default_ap_account_id: props.partner.default_ap_account_id,
        default_ar_account_id: props.partner.default_ar_account_id,
        addresses: vec![],
        contacts: vec![],
    });
    let show_saved = use_state(|| false);

    let on_input = |field_updater: fn(&mut UpdatePartnerRequest, String)| {
        let request = request.clone();
        Callback::from(move |e: InputEvent| {
            let mut req = (*request).clone();
            let value = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            field_updater(&mut req, value);
            request.set(req);
        })
    };

    let on_checkbox_change = |field_updater: fn(&mut UpdatePartnerRequest, bool)| {
        let request = request.clone();
        Callback::from(move |e: Event| {
            let mut req = (*request).clone();
            let checked = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .checked();
            field_updater(&mut req, checked);
            request.set(req);
        })
    };

    let on_select_change = |field_updater: fn(&mut UpdatePartnerRequest, Option<Uuid>)| {
        let request = request.clone();
        Callback::from(move |e: Event| {
            let mut req = (*request).clone();
            let value = e
                .target_unchecked_into::<web_sys::HtmlSelectElement>()
                .value();
            let uuid = if value.is_empty() {
                None
            } else {
                Uuid::parse_str(&value).ok()
            };
            field_updater(&mut req, uuid);
            request.set(req);
        })
    };

    let on_submit = {
        let on_submit = props.on_submit.clone();
        let request = request.clone();
        let show_saved = show_saved.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            on_submit.emit((*request).clone());
            show_saved.set(true);
            let show_saved = show_saved.clone();
            let timeout = Timeout::new(2000, move || {
                show_saved.set(false);
            });
            timeout.forget();
        })
    };

    html! {
        <form onsubmit={on_submit} class="modal__form">
            <label>{i18n.t("add-partner-legal-name-label")}</label>
            <input type="text" value={request.legal_name.clone()} oninput={on_input(|r, v| r.legal_name = v)} required=true />

            <label>{i18n.t("add-partner-trade-name-label")}</label>
            <input type="text" value={request.trade_name.clone().unwrap_or_default()} oninput={on_input(|r, v| r.trade_name = Some(v))} />

            <label>{i18n.t("add-partner-tax-identifier-label")}</label>
            <input type="text" value={request.tax_identifier.clone().unwrap_or_default()} oninput={on_input(|r, v| r.tax_identifier = Some(v))} />

            <label>{i18n.t("add-partner-is-vendor-label")}</label>
            <input type="checkbox" checked={request.is_vendor} onchange={on_checkbox_change(|r, v| r.is_vendor = v)} />

            <label>{i18n.t("add-partner-is-customer-label")}</label>
            <input type="checkbox" checked={request.is_customer} onchange={on_checkbox_change(|r, v| r.is_customer = v)} />

            <label>{i18n.t("add-partner-default-ap-account-label")}</label>
            <select onchange={on_select_change(|r, v| r.default_ap_account_id = v)}>
                <option value="" selected={request.default_ap_account_id.is_none()}>{ i18n.t("common-none") }</option>
                { for props.ap_accounts.iter().map(|(id, name)| html! {
                    <option value={id.to_string()} selected={request.default_ap_account_id == Some(*id)}>{name}</option>
                })}
            </select>

            <label>{i18n.t("add-partner-default-ar-account-label")}</label>
            <select onchange={on_select_change(|r, v| r.default_ar_account_id = v)}>
                <option value="" selected={request.default_ar_account_id.is_none()}>{ i18n.t("common-none") }</option>
                { for props.ar_accounts.iter().map(|(id, name)| html! {
                    <option value={id.to_string()} selected={request.default_ar_account_id == Some(*id)}>{name}</option>
                })}
            </select>
            <div class="table-actions">
                <button type="submit" class="button-primary">{ i18n.t("account-modal-save-button") }</button>
            </div>
            if *show_saved {
                <span class="fade-out message__success" style="margin-left: 1rem;">{ i18n.t("common-saved") }</span>
            }
        </form>
    }
}
