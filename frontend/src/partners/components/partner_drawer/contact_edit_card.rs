/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use shared_core::partners::models::partner_contact::PartnerContact;
use yew::prelude::*;
use shared_core::{ContactId, OrgId, PartnerId};
use crate::contexts::locale_context::use_locale;

#[derive(Properties, PartialEq)]
pub struct ContactEditCardProps {
    pub contact: Option<PartnerContact>,
    pub on_save: Callback<PartnerContact>,
    pub on_cancel: Callback<()>,
}

#[function_component(ContactEditCard)]
pub fn contact_edit_card(props: &ContactEditCardProps) -> Html {
    let i18n = use_locale();

    let contact_state = use_state(|| {
        props.contact.clone().unwrap_or_else(|| PartnerContact {
            id: ContactId::default(),
            organization_id: OrgId::default(),
            partner_id: PartnerId::default(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            is_primary: false,
            full_name: String::new(),
            preferred_name: String::new(),
            email: None,
            phone: None,
            role_title: None,
        })
    });

    let on_input = |field_updater: fn(&mut PartnerContact, String)| {
        let contact_state = contact_state.clone();
        Callback::from(move |e: InputEvent| {
            let mut contact = (*contact_state).clone();
            let value = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            field_updater(&mut contact, value);
            contact_state.set(contact);
        })
    };

    let on_primary_change = {
        let contact_state = contact_state.clone();
        Callback::from(move |e: Event| {
            let mut contact = (*contact_state).clone();
            contact.is_primary = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .checked();
            contact_state.set(contact);
        })
    };

    let on_save_click = {
        let on_save = props.on_save.clone();
        let contact_state = contact_state.clone();
        Callback::from(move |_| {
            on_save.emit((*contact_state).clone());
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
                        { if props.contact.is_some() { i18n.t("contact-edit-card-edit-title") } else { i18n.t("contact-edit-card-add-title") } }
                                </strong>
                </div>
            </div>
            <div class="card-form-compact">
                    <label>{i18n.t("contact-edit-card-full-name-label")}</label>
                    <input type="text" placeholder={i18n.t("contact-edit-card-full-name-label")} value={contact_state.full_name.clone()} oninput={on_input(|c, v| c.full_name = v)} />
                    <label>{i18n.t("contact-edit-card-preferred-name-label")}</label>
                    <input type="text" placeholder={i18n.t("contact-edit-card-preferred-name-label")} value={contact_state.preferred_name.clone()} oninput={on_input(|c, v| c.preferred_name = v)} />
                    <label>{i18n.t("contact-edit-card-email-label")}</label>
                    <input type="email"  placeholder={i18n.t("contact-edit-card-email-placeholder")} value={contact_state.email.clone().unwrap_or_default()} oninput={on_input(|c, v| c.email = Some(v))} />
                    <label>{i18n.t("contact-edit-card-phone-label")}</label>
                    <input type="tel"  placeholder={i18n.t("contact-edit-card-phone-placeholder")} value={contact_state.phone.clone().unwrap_or_default()} oninput={on_input(|c, v| c.phone = Some(v))} />
                    <label>{i18n.t("contact-edit-card-role-title-label")}</label>
                    <input type="text" value={contact_state.role_title.clone().unwrap_or_default()} oninput={on_input(|c, v| c.role_title = Some(v))} />
                    <label class="checkbox-label-compact" for="is_primary">
                        <input type="checkbox" id="is_primary_contact" checked={contact_state.is_primary} onchange={on_primary_change} />
                        <span>{i18n.t("common-primary")}</span>
                    </label>
            </div>
            // Tightly bundled actionable context footer links
            <div style="display: flex; justify-content: flex-end; gap: 6px; margin-top: 0.75rem; padding-top: 0.5rem; border-top: 1px dashed rgba(0,0,0,0.05);">
                <button class="button-secondary" style="padding: 4px 10px; font-size: 0.8rem;" onclick={on_cancel_click}>{ i18n.t("common-cancel") }</button>
                <button class="button-primary" style="padding: 4px 10px; font-size: 0.8rem;" onclick={on_save_click}>{ i18n.t("contact-edit-card-save-button") }</button>
            </div>
        </div>
    }
}
