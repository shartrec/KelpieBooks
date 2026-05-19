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
use shared_core::models::partner_contact::PartnerContact;
use uuid::Uuid;
use crate::components::partner_drawer::contact_edit_card::ContactEditCard;

#[derive(Clone, Debug, PartialEq)]
enum EditState {
    None,
    Adding,
    Editing(Uuid),
}

#[derive(Properties, PartialEq)]
pub struct ContactsViewProps {
    pub contacts: Vec<PartnerContact>,
    pub on_contacts_change: Callback<Vec<PartnerContact>>,
}

#[function_component(ContactsView)]
pub fn contacts_view(props: &ContactsViewProps) -> Html {
    let editing_state = use_state(|| EditState::None);
    let local_contacts = use_state(|| props.contacts.clone());

    {
        let local_contacts = local_contacts.clone();
        let props_contacts = props.contacts.clone();
        use_effect_with(props_contacts, move |props_contacts| {
            local_contacts.set(props_contacts.clone());
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
        let local_contacts = local_contacts.clone();
        let on_contacts_change = props.on_contacts_change.clone();
        Callback::from(move |contact: PartnerContact| {
            let mut contacts = (*local_contacts).clone();
            if let Some(pos) = contacts.iter().position(|c| c.id == contact.id) {
                contacts[pos] = contact;
            } else {
                contacts.push(contact);
            }
            local_contacts.set(contacts.clone());
            on_contacts_change.emit(contacts);
            editing_state.set(EditState::None);
        })
    };

    let on_delete = |id: Uuid| {
        let local_contacts = local_contacts.clone();
        let on_contacts_change = props.on_contacts_change.clone();
        Callback::from(move |_| {
            let mut contacts = (*local_contacts).clone();
            contacts.retain(|c| c.id != id);
            local_contacts.set(contacts.clone());
            on_contacts_change.emit(contacts);
        })
    };

    html! {
        <div class="contacts-view">
            <div class="table-actions">
                <button class="button-primary" onclick={on_add_click} disabled={*editing_state != EditState::None}>{ "Add Contact" }</button>
            </div>

            { if *editing_state == EditState::Adding {
                html!{ <ContactEditCard contact={None} on_save={on_save.clone()} on_cancel={on_cancel.clone()} /> }
            } else { html!{} }}

            <div class="card-grid">
                { for (*local_contacts).iter().map(|contact| {
                    if *editing_state == EditState::Editing(contact.id) {
                        html!{ <ContactEditCard contact={contact.clone()} on_save={on_save.clone()} on_cancel={on_cancel.clone()} /> }
                    } else {
                        let mut card_class = classes!("card");
                        if contact.is_primary {
                            card_class.push("card--primary-contact");
                        }

                        html! {
                            <div class={card_class}>
                                <div class="card__content-wrapper">

                                    // Meta Row houses Type, Primary Flag, and Actions side-by-side
                                    <div class="card__meta-line">
                                        <div class="card__title">
                                            if contact.is_primary {
                                                <span class="badge" style="background-color: #333; color: white;">{ "Primary" }</span>
                                            }
                                        </div>

                                        // Actions sit tightly pinned on the top right
                                        <div class="card__actions">
                                            <button class="icon-button" onclick={on_edit_click(contact.id)} disabled={*editing_state != EditState::None}>
                                                <img src="/images/edit.svg" alt="Edit" />
                                            </button>
                                            <button class="icon-button" onclick={on_delete(contact.id)} disabled={*editing_state != EditState::None}>
                                                <img src="/images/delete.svg" alt="Delete" />
                                            </button>
                                        </div>
                                    </div>

                                    <h5>{ format!("{} {}", contact.first_name, contact.last_name) }</h5>
                                </div>
                                <p class="card__address-text">{ contact.role_title.as_deref().unwrap_or("No role specified") }</p>
                                <p class="card__address-text">{ contact.email.as_deref().unwrap_or("") }</p>
                                <p class="card__address-text">{ contact.phone.as_deref().unwrap_or("") }</p>
                            </div>
                        }
                    }
                })}
            </div>
        </div>
    }
}
