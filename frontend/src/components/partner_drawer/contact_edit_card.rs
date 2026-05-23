/*
 * Copyright (c) 2026-2026. Trevor Campbell and others.
 *
 * This file is part of KelpieBooks.
 *
 * KelpieBooks is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License,or
 * (at your option) any later version.
 *
 * KelpieBooks is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with KelpieBooks; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 *
 * Contributors:
 *      Trevor Campbell
 *
 */

use shared_core::models::partner_contact::PartnerContact;
use uuid::Uuid;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ContactEditCardProps {
    pub contact: Option<PartnerContact>,
    pub on_save: Callback<PartnerContact>,
    pub on_cancel: Callback<()>,
}

#[function_component(ContactEditCard)]
pub fn contact_edit_card(props: &ContactEditCardProps) -> Html {
    let contact_state = use_state(|| {
        props.contact.clone().unwrap_or_else(|| PartnerContact {
            id: Uuid::nil(),
            organization_id: Uuid::nil(),
            partner_id: Uuid::nil(),
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
                        { if props.contact.is_some() { "Edit Contact" } else { "Add Contact" } }
                                </strong>
                </div>
            </div>
            <div class="card-form-compact">
                    <label>{"Full Name"}</label>
                    <input type="text" placeholder="Full Name" value={contact_state.full_name.clone()} oninput={on_input(|c, v| c.full_name = v)} />
                    <label>{"Preferred Name"}</label>
                    <input type="text" placeholder="Preferred Name" value={contact_state.preferred_name.clone()} oninput={on_input(|c, v| c.preferred_name = v)} />
                    <label>{"Email address"}</label>
                    <input type="email"  placeholder="Email" value={contact_state.email.clone().unwrap_or_default()} oninput={on_input(|c, v| c.email = Some(v))} />
                    <label>{"Phone number"}</label>
                    <input type="tel"  placeholder="Phone" value={contact_state.phone.clone().unwrap_or_default()} oninput={on_input(|c, v| c.phone = Some(v))} />
                    <label>{"Role/Title"}</label>
                    <input type="text" value={contact_state.role_title.clone().unwrap_or_default()} oninput={on_input(|c, v| c.role_title = Some(v))} />
                    <label class="checkbox-label-compact" for="is_primary">
                        <input type="checkbox" id="is_primary_contact" checked={contact_state.is_primary} onchange={on_primary_change} />
                        <span>{"Primary"}</span>
                    </label>
            </div>
            // Tightly bundled actionable context footer links
            <div style="display: flex; justify-content: flex-end; gap: 6px; margin-top: 0.75rem; padding-top: 0.5rem; border-top: 1px dashed rgba(0,0,0,0.05);">
                <button class="button-secondary" style="padding: 4px 10px; font-size: 0.8rem;" onclick={on_cancel_click}>{ "Cancel" }</button>
                <button class="button-primary" style="padding: 4px 10px; font-size: 0.8rem;" onclick={on_save_click}>{ "Save Contact" }</button>
            </div>
        </div>
    }
}
