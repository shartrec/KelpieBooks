/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::contexts::locale_context::use_locale;
use shared_core::models::address_type::AddressType;
use shared_core::models::partner_address::PartnerAddress;
use uuid::Uuid;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AddressEditCardProps {
    pub address: Option<PartnerAddress>,
    pub on_save: Callback<PartnerAddress>,
    pub on_cancel: Callback<()>,
}

#[function_component(AddressEditCard)]
pub fn address_edit_card(props: &AddressEditCardProps) -> Html {
    let i18n = use_locale();

    let address_state = use_state(|| {
        props.address.clone().unwrap_or_else(|| PartnerAddress {
            id: Uuid::nil(),
            organization_id: Uuid::nil(),
            partner_id: Uuid::nil(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            address_type: AddressType::General,
            is_primary: false,
            address_line1: String::new(),
            address_line2: None,
            city: String::new(),
            state_province: None,
            postal_code: None,
            country: String::new(),
        })
    });

    let on_input = |field_updater: fn(&mut PartnerAddress, String)| {
        let address_state = address_state.clone();
        Callback::from(move |e: InputEvent| {
            let mut address = (*address_state).clone();
            let value = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            field_updater(&mut address, value);
            address_state.set(address);
        })
    };

    let on_select_change = {
        let address_state = address_state.clone();
        Callback::from(move |e: Event| {
            let value = e
                .target_unchecked_into::<web_sys::HtmlSelectElement>()
                .value();
            if let Ok(address_type) = value.parse::<AddressType>() {
                let mut address = (*address_state).clone();
                address.address_type = address_type;
                address_state.set(address);
            }
        })
    };

    let on_primary_change = {
        let address_state = address_state.clone();
        Callback::from(move |e: Event| {
            let mut address = (*address_state).clone();
            address.is_primary = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .checked();
            address_state.set(address);
        })
    };

    let on_save_click = {
        let on_save = props.on_save.clone();
        let address_state = address_state.clone();
        Callback::from(move |_| {
            on_save.emit((*address_state).clone());
        })
    };

    let on_cancel_click = {
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
                        { if props.address.is_some() { i18n.t("address-edit-card-edit-title") } else { i18n.t("address-edit-card-add-title") } }
                    </strong>
                </div>
            </div>

            <div class="card-form-compact">
                // Full-width entry 1
                <label>{i18n.t("address-edit-card-line1-label")}</label>
                <input type="text" placeholder={i18n.t("address-edit-card-line1-placeholder")} value={address_state.address_line1.clone()} oninput={on_input(|a, v| a.address_line1 = v)} />

                <label>{i18n.t("address-edit-card-line1-label")}</label>
                <input type="text" placeholder={i18n.t("address-edit-card-line2-placeholder")} value={address_state.address_line2.clone().unwrap_or_default()} oninput={on_input(|a, v| a.address_line2 = Some(v))} />

                // Row split: City & State side-by-side
                <label>{i18n.t("address-edit-card-city-label")}</label>
                <input type="text" placeholder={i18n.t("address-edit-card-city-placeholder")} value={address_state.city.clone()} oninput={on_input(|a, v| a.city = v)} />
                <label>{i18n.t("address-edit-card-state-label")}</label>
                <input type="text" placeholder={i18n.t("address-edit-card-state-placeholder")} value={address_state.state_province.clone().unwrap_or_default()} oninput={on_input(|a, v| a.state_province = Some(v))} />

                // Row split: Postcode & Country side-by-side
                <label>{i18n.t("address-edit-card-post-code-label")}</label>
                <input type="text" placeholder={i18n.t("address-edit-card-post-code-placeholder")} value={address_state.postal_code.clone().unwrap_or_default()} oninput={on_input(|a, v| a.postal_code = Some(v))} />
                <label>{i18n.t("address-edit-card-country-label")}</label>
                <input type="text" placeholder={i18n.t("address-edit-card-country-placeholder")} value={address_state.country.clone()} oninput={on_input(|a, v| a.country = v)} />
                <label>{i18n.t("address-edit-card-country-label")}</label>
                <div class="select-wrapper-compact">
                    <select onchange={on_select_change}>
                        { for AddressType::iterator().map(|t| html!{
                            <option value={t.to_string()} selected={address_state.address_type == t}>{t.display_name()}</option>
                        })}
                    </select>
                </div>

                <label class="checkbox-label-compact" for="is_primary">
                    <input type="checkbox" id="is_primary" checked={address_state.is_primary} onchange={on_primary_change} />
                    <span>{i18n.t("common-primary")}</span>
                </label>
            </div>

            // Tightly bundled actionable context footer links
            <div style="display: flex; justify-content: flex-end; gap: 6px; margin-top: 0.75rem; padding-top: 0.5rem; border-top: 1px dashed rgba(0,0,0,0.05);">
                <button class="button-secondary" style="padding: 4px 10px; font-size: 0.8rem;" onclick={on_cancel_click}>{ i18n.t("common-cancel") }</button>
                <button class="button-primary" style="padding: 4px 10px; font-size: 0.8rem;" onclick={on_save_click}>{ i18n.t("address-edit-card-save-button") }</button>
            </div>
        </div>
    }
}
