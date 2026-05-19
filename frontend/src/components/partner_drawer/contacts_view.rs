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
use crate::components::partner_drawer::delete_contact_confirmation_modal::DeleteContactConfirmationModal;
use crate::api::Api;
use crate::contexts::auth_context::use_user_context;
use yew_router::prelude::use_navigator;

#[derive(Clone, Debug, PartialEq)]
enum EditState {
    None,
    Adding,
    Editing(Uuid),
}

#[derive(Properties, PartialEq)]
pub struct ContactsViewProps {
    pub contacts: Vec<PartnerContact>,
    pub partner_id: Uuid,
    pub on_change: Callback<()>,
}

#[function_component(ContactsView)]
pub fn contacts_view(props: &ContactsViewProps) -> Html {
    let user_ctx = use_user_context();
    let navigator = use_navigator().unwrap();
    let editing_state = use_state(|| EditState::None);
    let contact_to_delete = use_state(|| None::<PartnerContact>);
    let error = use_state(|| None::<String>);

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
        let on_change = props.on_change.clone();
        let partner_id = props.partner_id;
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        let error = error.clone();
        Callback::from(move |contact: PartnerContact| {
            let on_change = on_change.clone();
            let editing_state = editing_state.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            let error = error.clone();
            let is_new = *editing_state == EditState::Adding;
            wasm_bindgen_futures::spawn_local(async move {
                let resp = if is_new {
                    Api::post(&format!("/api/partners/{}/contacts", partner_id), &contact, user_ctx, navigator).await
                } else {
                    Api::put(&format!("/api/partners/{}/contacts/{}", partner_id, contact.id), &contact, user_ctx, navigator).await
                };
                match resp {
                    Ok(r) if r.ok() => {
                        on_change.emit(());
                        editing_state.set(EditState::None);
                    }
                    Ok(r) => error.set(Some(format!("Failed to save contact: {}", r.status()))),
                    Err(e) => error.set(Some(format!("Network error: {}", e))),
                }
            });
        })
    };

    let on_delete_click = |contact: PartnerContact| {
        let contact_to_delete = contact_to_delete.clone();
        Callback::from(move |_| {
            contact_to_delete.set(Some(contact.clone()));
        })
    };

    let on_delete_confirm = {
        let contact_to_delete = contact_to_delete.clone();
        let on_change = props.on_change.clone();
        let partner_id = props.partner_id;
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        let error = error.clone();
        Callback::from(move |_: ()| {
            if let Some(contact) = &*contact_to_delete {
                let on_change = on_change.clone();
                let contact_to_delete = contact_to_delete.clone();
                let user_ctx = user_ctx.clone();
                let navigator = navigator.clone();
                let error = error.clone();
                let contact_id = contact.id;
                wasm_bindgen_futures::spawn_local(async move {
                    let resp = Api::delete(&format!("/api/partners/{}/contacts/{}", partner_id, contact_id), user_ctx, navigator).await;
                    match resp {
                        Ok(r) if r.ok() => {
                            on_change.emit(());
                            contact_to_delete.set(None);
                        }
                        Ok(r) => error.set(Some(format!("Failed to delete contact: {}", r.status()))),
                        Err(e) => error.set(Some(format!("Network error: {}", e))),
                    }
                });
            }
        })
    };

    html! {
        <div class="contacts-view">
            <div class="table-actions">
                <button class="button-primary" onclick={on_add_click} disabled={*editing_state != EditState::None}>{ "Add Contact" }</button>
            </div>

            if let Some(e) = &*error {
                <div class="error">{e}</div>
            }

            { if *editing_state == EditState::Adding {
                html!{ <ContactEditCard contact={None} on_save={on_save.clone()} on_cancel={on_cancel.clone()} /> }
            } else { html!{} }}

            <div class="card-grid">
                { for props.contacts.iter().map(|contact| {
                    if *editing_state == EditState::Editing(contact.id) {
                        html!{ <ContactEditCard contact={contact.clone()} on_save={on_save.clone()} on_cancel={on_cancel.clone()} /> }
                    } else {
                        let mut card_class = classes!("card");
                        if contact.is_primary {
                            card_class.push("card--primary-contact");
                        }

                        html! {
                            <div class={card_class}>
                                <div class="card__header">
                                    <h5>{ format!("{} {}", contact.first_name, contact.last_name) }</h5>
                                    if contact.is_primary {
                                        <span class="badge badge--contact">{ "Primary" }</span>
                                    }
                                </div>
                                <div class="card__body">
                                    <p>{ contact.role_title.as_deref().unwrap_or("No role specified") }</p>
                                    <p>{ contact.email.as_deref().unwrap_or("") }</p>
                                    <p>{ contact.phone.as_deref().unwrap_or("") }</p>
                                </div>
                                <div class="card__footer">
                                    <button class="icon-button" onclick={on_edit_click(contact.id)} disabled={*editing_state != EditState::None}>
                                        <img src="/images/edit.svg" alt="Edit" />
                                    </button>
                                    <button class="icon-button" onclick={on_delete_click(contact.clone())} disabled={*editing_state != EditState::None}>
                                        <img src="/images/delete.svg" alt="Delete" />
                                    </button>
                                </div>
                            </div>
                        }
                    }
                })}
            </div>
            if let Some(contact) = &*contact_to_delete {
                <DeleteContactConfirmationModal contact={contact.clone()} on_close={Callback::from(move |_| contact_to_delete.set(None))} on_confirm={on_delete_confirm} />
            }
        </div>
    }
}
