/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use fluent::fluent_args;
use shared_core::partners::models::partner_contact::PartnerContact;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{
    api::Api,
    contexts::{
        auth_context::use_user_context,
        locale_context::use_locale,
    },
    partners::components::partner_drawer::{
        contact_edit_card::ContactEditCard,
    },
};
use crate::core::components::delete_confirmation_modal::DeleteConfirmationModal;

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
    let i18n = use_locale();
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
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        let error = error.clone();
        Callback::from(move |contact: PartnerContact| {
            let on_change = on_change.clone();
            let editing_state = editing_state.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            let error = error.clone();
            let is_new = *editing_state == EditState::Adding;
            wasm_bindgen_futures::spawn_local(async move {
                let resp = if is_new {
                    Api::post(
                        &format!("/api/partners/{}/contacts", partner_id),
                        &contact,
                        user_ctx,
                        navigator,
                    )
                    .await
                } else {
                    Api::put(
                        &format!("/api/partners/{}/contacts/{}", partner_id, contact.id),
                        &contact,
                        user_ctx,
                        navigator,
                    )
                    .await
                };
                match resp {
                    Ok(r) if r.ok() => {
                        on_change.emit(());
                        editing_state.set(EditState::None);
                    }
                    Ok(r) => error.set(Some(i18n.t_args(
                        "contacts-view-error-save",
                        &fluent_args!["status" => r.status()],
                    ))),
                    Err(e) => error.set(Some(i18n.t_args(
                        "common-network-error",
                        &fluent_args!["error" => e.to_string()],
                    ))),
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
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        let error = error.clone();
        Callback::from(move |_: ()| {
            if let Some(contact) = &*contact_to_delete {
                let on_change = on_change.clone();
                let contact_to_delete = contact_to_delete.clone();
                let user_ctx = user_ctx.clone();
                let i18n = i18n.clone();
                let navigator = navigator.clone();
                let error = error.clone();
                let contact_id = contact.id;
                wasm_bindgen_futures::spawn_local(async move {
                    let resp = Api::delete(
                        &format!("/api/partners/{}/contacts/{}", partner_id, contact_id),
                        user_ctx,
                        navigator,
                    )
                    .await;
                    match resp {
                        Ok(r) if r.ok() => {
                            on_change.emit(());
                            contact_to_delete.set(None);
                        }
                        Ok(r) => error.set(Some(i18n.t_args(
                            "contacts-view-error-delete",
                            &fluent_args!["status" => r.status()],
                        ))),
                        Err(e) => error.set(Some(i18n.t_args(
                            "common-network-error",
                            &fluent_args!["error" => e.to_string()],
                        ))),
                    }
                });
            }
        })
    };

    html! {
        <div class="contacts-view">
            <div class="table-actions">
                <button class="button-primary" onclick={on_add_click} disabled={*editing_state != EditState::None}>{ i18n.t("contacts-view-add-button") }</button>
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
                                    <h5>{ format!("{} {}", contact.full_name, contact.preferred_name) }</h5>
                                    if contact.is_primary {
                                        <span class="badge badge--contact">{ i18n.t("common-primary") }</span>
                                    }
                                </div>
                                <div class="card__body">
                                    <p>{ contact.role_title.as_deref().unwrap_or(&i18n.t("contacts-view-no-role")) }</p>
                                    <p>{ contact.email.as_deref().unwrap_or("") }</p>
                                    <p>{ contact.phone.as_deref().unwrap_or("") }</p>
                                </div>
                                <div class="card__footer">
                                    <button class="icon-button" onclick={on_edit_click(contact.id)} disabled={*editing_state != EditState::None}>
                                        <img src="/images/edit.svg" alt={i18n.t("common-edit")} />
                                    </button>
                                    <button class="icon-button" onclick={on_delete_click(contact.clone())} disabled={*editing_state != EditState::None}>
                                        <img src="/images/delete.svg" alt={i18n.t("common-delete")} />
                                    </button>
                                </div>
                            </div>
                        }
                    }
                })}
            </div>
            if let Some(contact) = &*contact_to_delete {
                <DeleteConfirmationModal
                    title={i18n.t("common-confirm-deletion")}
                    message={i18n.t_args("delete-contact-confirm-message", &fluent_args!["name" => &contact.full_name, "preferred_name" => &contact.preferred_name])}
                    warning = {i18n.t("delete-confirm-warning")}
                    on_cancel={{Callback::from(move |_| contact_to_delete.set(None))}}
                    on_confirm={on_delete_confirm}
                />
            }
        </div>
    }
}
