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
                        { if props.address.is_some() { "Edit Address" } else { "Add Address" } }
                    </strong>
                </div>
            </div>

            <div class="card-form-compact">
                // Full-width entry 1
                <label>{"Addr line 1:"}</label>
                <input type="text" placeholder="Address Line 1" value={address_state.address_line1.clone()} oninput={on_input(|a, v| a.address_line1 = v)} />

                <label>{"Addr line 1:"}</label>
                <input type="text" placeholder="Address Line 2 (Optional)" value={address_state.address_line2.clone().unwrap_or_default()} oninput={on_input(|a, v| a.address_line2 = Some(v))} />

                // Row split: City & State side-by-side
                <label>{"City:"}</label>
                <input type="text" placeholder="City" value={address_state.city.clone()} oninput={on_input(|a, v| a.city = v)} />
                <label>{"State:"}</label>
                <input type="text" placeholder="State" value={address_state.state_province.clone().unwrap_or_default()} oninput={on_input(|a, v| a.state_province = Some(v))} />

                // Row split: Postcode & Country side-by-side
                <label>{"Post Code:"}</label>
                <input type="text" placeholder="Postcode" value={address_state.postal_code.clone().unwrap_or_default()} oninput={on_input(|a, v| a.postal_code = Some(v))} />
                <label>{"Country:"}</label>
                <input type="text" placeholder="Country" value={address_state.country.clone()} oninput={on_input(|a, v| a.country = v)} />
                <label>{"Country:"}</label>
                <div class="select-wrapper-compact">
                    <select onchange={on_select_change}>
                        { for AddressType::iterator().map(|t| html!{
                            <option value={t.to_string()} selected={address_state.address_type == t}>{t.display_name()}</option>
                        })}
                    </select>
                </div>

                <label class="checkbox-label-compact" for="is_primary">
                    <input type="checkbox" id="is_primary" checked={address_state.is_primary} onchange={on_primary_change} />
                    <span>{"Primary"}</span>
                </label>
            </div>

            // Tightly bundled actionable context footer links
            <div style="display: flex; justify-content: flex-end; gap: 6px; margin-top: 0.75rem; padding-top: 0.5rem; border-top: 1px dashed rgba(0,0,0,0.05);">
                <button class="button-secondary" style="padding: 4px 10px; font-size: 0.8rem;" onclick={on_cancel_click}>{ "Cancel" }</button>
                <button class="button-primary" style="padding: 4px 10px; font-size: 0.8rem;" onclick={on_save_click}>{ "Save Address" }</button>
            </div>
        </div>
    }
}
