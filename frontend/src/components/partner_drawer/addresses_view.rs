/*
 * Copyright (c) 2026. Trevor Campbell and others.
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

use yew::prelude::*;
use shared_core::models::partner_address::PartnerAddress;
use shared_core::models::address_type::AddressType;
use uuid::Uuid;
use crate::components::partner_drawer::address_edit_card::AddressEditCard;

#[derive(Clone, Debug, PartialEq)]
enum EditState {
    None,
    Adding,
    Editing(Uuid),
}

#[derive(Properties, PartialEq)]
pub struct AddressesViewProps {
    pub addresses: Vec<PartnerAddress>,
    pub on_addresses_change: Callback<Vec<PartnerAddress>>,
}

#[function_component(AddressesView)]
pub fn addresses_view(props: &AddressesViewProps) -> Html {
    let editing_state = use_state(|| EditState::None);
    let local_addresses = use_state(|| props.addresses.clone());

    {
        let local_addresses = local_addresses.clone();
        let props_addresses = props.addresses.clone();
        use_effect_with(props_addresses, move |props_addresses| {
            local_addresses.set(props_addresses.clone());
            || ()
        });
    }

    let on_add_click = {
        let editing_state = editing_state.clone();
        Callback::from(move |_| editing_state.set(EditState::Adding))
    };

    let on_edit_click = |id: Uuid| {
        let editing_state = editing_state.clone();
        Callback::from(move |_| editing_state.set(EditState::Editing(id)))
    };

    let on_cancel = {
        let editing_state = editing_state.clone();
        Callback::from(move |_| editing_state.set(EditState::None))
    };

    let on_save = {
        let editing_state = editing_state.clone();
        let local_addresses = local_addresses.clone();
        let on_addresses_change = props.on_addresses_change.clone();
        Callback::from(move |address: PartnerAddress| {
            let mut addresses = (*local_addresses).clone();
            if let Some(pos) = addresses.iter().position(|a| a.id == address.id) {
                addresses[pos] = address;
            } else {
                addresses.push(address);
            }
            local_addresses.set(addresses.clone());
            on_addresses_change.emit(addresses);
            editing_state.set(EditState::None);
        })
    };

    let on_delete = |id: Uuid| {
        let local_addresses = local_addresses.clone();
        let on_addresses_change = props.on_addresses_change.clone();
        Callback::from(move |_| {
            let mut addresses = (*local_addresses).clone();
            addresses.retain(|a| a.id != id);
            local_addresses.set(addresses.clone());
            on_addresses_change.emit(addresses);
        })
    };

    html! {
        <div class="addresses-view">
            <div class="table-actions">
                <button class="button-primary" onclick={on_add_click} disabled={*editing_state != EditState::None}>{ "Add Address" }</button>
            </div>

            { if *editing_state == EditState::Adding {
                html!{ <AddressEditCard address={None} on_save={on_save.clone()} on_cancel={on_cancel.clone()} /> }
            } else { html!{} }}

            <div class="card-grid">
                { for (*local_addresses).iter().map(|address| {
                    if *editing_state == EditState::Editing(address.id) {
                        html!{ <AddressEditCard address={address.clone()} on_save={on_save.clone()} on_cancel={on_cancel.clone()} /> }
                    } else {

                        let mut card_classes = vec!["card"];
                        if address.is_primary && address.address_type == AddressType::Billing {
                            card_classes.push("card--primary-billing");
                        } else if address.is_primary && address.address_type == AddressType::Shipping {
                            card_classes.push("card--primary-shipping");
                        }

                        let badge_class = match address.address_type {
                            AddressType::Billing => "badge badge--billing",
                            AddressType::Shipping => "badge badge--shipping",
                            AddressType::General => "badge",
                        };

                        html! {
                            <div class={classes!(card_classes)}>
                                <div class="card__content-wrapper">

                                    // Meta Row houses Type, Primary Flag, and Actions side-by-side
                                    <div class="card__meta-line">
                                        <div class="card__title">
                                            <span class={badge_class}>{ address.address_type.to_string() }</span>
                                            if address.is_primary {
                                                <span class="badge" style="background-color: #333; color: white;">{ "Primary" }</span>
                                            }
                                        </div>

                                        // Actions sit tightly pinned on the top right
                                        <div class="card__actions">
                                            <button class="icon-button" onclick={on_edit_click(address.id)} disabled={*editing_state != EditState::None}>
                                                <img src="/images/edit.svg" alt="Edit" />
                                            </button>
                                            <button class="icon-button" onclick={on_delete(address.id)} disabled={*editing_state != EditState::None}>
                                                <img src="/images/delete.svg" alt="Delete" />
                                            </button>
                                        </div>
                                    </div>

                                    // Consolidated address body values
                                    <p class="card__address-text"><strong>{ &address.address_line1 }</strong></p>
                                    if let Some(line2) = &address.address_line2 {
                                        if !line2.is_empty() {
                                            <p class="card__address-text">{ line2 }</p>
                                        }
                                    }
                                    <p class="card__address-text">
                                        { format!("{}, {} {}", address.city, address.state_province.as_deref().unwrap_or(""), address.postal_code.as_deref().unwrap_or("")) }
                                    </p>
                                    <p class="card__address-text" style="font-size: 0.75rem; color: var(--text-color-light);">
                                        { &address.country }
                                    </p>
                                </div>
                            </div>
                        }
                    }
                })}
            </div>
        </div>
    }
}
