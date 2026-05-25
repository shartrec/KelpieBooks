/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::api::Api;
use crate::components::partner_drawer::address_edit_card::AddressEditCard;
use crate::components::partner_drawer::delete_address_confirmation_modal::DeleteAddressConfirmationModal;
use crate::contexts::auth_context::use_user_context;
use fluent::fluent_args;
use shared_core::i18n::{t, t_args};
use shared_core::models::address_type::AddressType;
use shared_core::models::partner_address::PartnerAddress;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[derive(Clone, Debug, PartialEq)]
enum EditState {
    None,
    Adding,
    Editing(Uuid),
}

#[derive(Properties, PartialEq)]
pub struct AddressesViewProps {
    pub addresses: Vec<PartnerAddress>,
    pub partner_id: Uuid,
    pub on_change: Callback<()>,
}

#[function_component(AddressesView)]
pub fn addresses_view(props: &AddressesViewProps) -> Html {
    let user_ctx = use_user_context();
    let navigator = use_navigator().unwrap();
    let editing_state = use_state(|| EditState::None);
    let address_to_delete = use_state(|| None::<PartnerAddress>);
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
        Callback::from(move |address: PartnerAddress| {
            let on_change = on_change.clone();
            let editing_state = editing_state.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            let error = error.clone();
            let is_new = *editing_state == EditState::Adding;
            wasm_bindgen_futures::spawn_local(async move {
                let resp = if is_new {
                    Api::post(
                        &format!("/api/partners/{}/addresses", partner_id),
                        &address,
                        user_ctx,
                        navigator,
                    )
                    .await
                } else {
                    Api::put(
                        &format!("/api/partners/{}/addresses/{}", partner_id, address.id),
                        &address,
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
                    Ok(r) => error.set(Some(t_args(
                        "addresses-view-error-save",
                        &fluent_args!["status" => r.status()],
                    ))),
                    Err(e) => error.set(Some(t_args(
                        "common-network-error",
                        &fluent_args!["error" => e.to_string()],
                    ))),
                }
            });
        })
    };

    let on_delete_click = |address: PartnerAddress| {
        let address_to_delete = address_to_delete.clone();
        Callback::from(move |_| {
            address_to_delete.set(Some(address.clone()));
        })
    };

    let on_delete_confirm = {
        let address_to_delete = address_to_delete.clone();
        let on_change = props.on_change.clone();
        let partner_id = props.partner_id;
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        let error = error.clone();
        Callback::from(move |_: ()| {
            if let Some(address) = &*address_to_delete {
                let on_change = on_change.clone();
                let address_to_delete = address_to_delete.clone();
                let user_ctx = user_ctx.clone();
                let navigator = navigator.clone();
                let error = error.clone();
                let address_id = address.id;
                wasm_bindgen_futures::spawn_local(async move {
                    let resp = Api::delete(
                        &format!("/api/partners/{}/addresses/{}", partner_id, address_id),
                        user_ctx,
                        navigator,
                    )
                    .await;
                    match resp {
                        Ok(r) if r.ok() => {
                            on_change.emit(());
                            address_to_delete.set(None);
                        }
                        Ok(r) => {
                            error.set(Some(t_args(
                                "addresses-view-error-delete",
                                &fluent_args!["status" => r.status()],
                            )))
                        }
                        Err(e) => error.set(Some(t_args(
                            "common-network-error",
                            &fluent_args!["error" => e.to_string()],
                        ))),
                    }
                });
            }
        })
    };

    html! {
        <div class="addresses-view">
            <div class="table-actions">
                <button class="button-primary" onclick={on_add_click} disabled={*editing_state != EditState::None}>{ t("addresses-view-add-button") }</button>
            </div>

            if let Some(e) = &*error {
                <div class="error">{e}</div>
            }

            { if *editing_state == EditState::Adding {
                html!{ <AddressEditCard address={None} on_save={on_save.clone()} on_cancel={on_cancel.clone()} /> }
            } else { html!{} }}

            <div class="card-grid">
                { for props.addresses.iter().map(|address| {
                    if *editing_state == EditState::Editing(address.id) {
                        html!{ <AddressEditCard address={address.clone()} on_save={on_save.clone()} on_cancel={on_cancel.clone()} /> }
                    } else {
                        let mut card_class = classes!("card");
                        if address.is_primary {
                            match address.address_type {
                                AddressType::Billing => card_class.push("card--primary-billing"),
                                AddressType::Shipping => card_class.push("card--primary-shipping"),
                                _ => {}
                            }
                        }

                        html! {
                            <div class={card_class}>
                                <div class="card__header">
                                    <h5>{ address.address_type.to_string() }</h5>
                                    if address.is_primary {
                                        <span class="badge badge--primary">{ t("common-primary") }</span>
                                    }
                                </div>
                                <div class="card__body">
                                    <p>{ &address.address_line1 }</p>
                                    if let Some(line2) = &address.address_line2 {
                                        if !line2.is_empty() {
                                            <p>{ line2 }</p>
                                        }
                                    }
                                    <p>{ format!("{}, {} {}", address.city, address.state_province.as_deref().unwrap_or(""), address.postal_code.as_deref().unwrap_or("")) }</p>
                                    <p>{ &address.country }</p>
                                </div>
                                <div class="card__footer">
                                    <button class="icon-button" onclick={on_edit_click(address.id)} disabled={*editing_state != EditState::None}>
                                        <img src="/images/edit.svg" alt={t("common-edit")} />
                                    </button>
                                    <button class="icon-button" onclick={on_delete_click(address.clone())} disabled={*editing_state != EditState::None}>
                                        <img src="/images/delete.svg" alt={t("common-delete")} />
                                    </button>
                                </div>
                            </div>
                        }
                    }
                })}
            </div>
            if let Some(address) = &*address_to_delete {
                <DeleteAddressConfirmationModal address={address.clone()} on_close={Callback::from(move |_| address_to_delete.set(None))} on_confirm={on_delete_confirm} />
            }
        </div>
    }
}
