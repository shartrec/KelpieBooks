/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use rust_decimal::Decimal;
use shared_core::{
    ledger::models::account::Account,
    sales::models::tax::TaxRate,
};
use uuid::Uuid;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

use crate::{
    contexts::locale_context::use_locale,
};
use crate::core::components::currency_input::DecimalInput;

#[derive(Properties, PartialEq, Clone)]
pub struct TaxRateEditCardProps {
    pub rate: TaxRate,
    pub accounts: Vec<Account>,
    pub on_save: Callback<TaxRate>,
    pub on_cancel: Callback<()>,
}

#[function_component(TaxRateEditCard)]
pub fn tax_rate_edit_card(props: &TaxRateEditCardProps) -> Html {
    let rate = use_state(|| props.rate.clone());
    let i18n = use_locale();

    let on_input = |field_updater: fn(&mut TaxRate, String)| {
        let state = rate.clone();
        Callback::from(move |e: InputEvent| {
            let mut info = (*state).clone();
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            field_updater(&mut info, value);
            state.set(info);
        })
    };

    let on_select_change = |field_updater: fn(&mut TaxRate, String)| {
        let state = rate.clone();
        Callback::from(move |e: Event| {
            let mut info = (*state).clone();
            let value = e.target_unchecked_into::<HtmlSelectElement>().value();
            field_updater(&mut info, value);
            state.set(info);
        })
    };

    let on_rate_change = {
        let rate = rate.clone();
        Callback::from(move |value: Decimal| {
            let mut new_rate = (*rate).clone();
            new_rate.rate = value;
            rate.set(new_rate);
        })
    };

    let on_save = {
        let on_save = props.on_save.clone();
        let rate = rate.clone();
        Callback::from(move |_| {
            on_save.emit((*rate).clone());
        })
    };

    let on_cancel = {
        let on_cancel = props.on_cancel.clone();
        Callback::from(move |_| {
            on_cancel.emit(());
        })
    };

    html! {
        <div class="card card--editing" style="padding: 1rem; margin-bottom: 1rem;">
            <div class="card__meta-line" style="margin-bottom: 0.75rem;">
                <div class="card__title">
                    <strong style="font-size: 0.85rem; text-transform: uppercase; color: var(--brand-dark);">
                        { if props.rate.name.is_empty() { i18n.t("tax-rate-edit-card-add-title") } else { i18n.t("tax-rate-edit-card-edit-title") } }
                    </strong>
                </div>
            </div>

            <div class="card-form-compact">
                <label>{i18n.t("common-name")}</label>
                <input type="text" value={rate.name.clone()} oninput={on_input(|r, v| r.name = v)} />

                <label>{i18n.t("tax-rate-edit-card-rate-label")}</label>
                <DecimalInput value={rate.rate} on_change={on_rate_change} />

                <label>{i18n.t("common-account")}</label>
                <select onchange={on_select_change(|r, v| r.liability_account_id = Uuid::parse_str(&v).unwrap_or_default())}>
                    <option value="" disabled=true selected={rate.liability_account_id.is_nil()}>{i18n.t("journal-entry-select-account")}</option>
                    { for props.accounts.iter().map(|account| html! {
                        <option value={account.id.to_string()} selected={rate.liability_account_id == account.id}>{&account.name}</option>
                    })}
                </select>

                <label>{i18n.t("tax-rate-edit-card-valid-from-label")}</label>
                <input type="date" value={rate.valid_from.to_string()} oninput={on_input(|r, v| r.valid_from = v.parse().unwrap())} />

                <label>{i18n.t("tax-rate-edit-card-valid-to-label")}</label>
                <input type="date" value={rate.valid_to.map(|d| d.to_string()).unwrap_or_default()} oninput={on_input(|r, v| r.valid_to = v.parse().ok())} />
            </div>
            <div class="card-footer">
                <button class="button-primary" onclick={on_save}>{ i18n.t("common-save") }</button>
                <button class="button-secondary" onclick={on_cancel}>{ i18n.t("common-cancel") }</button>
            </div>
        </div>
    }
}