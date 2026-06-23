/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use shared_core::sales::models::invoice_address::{
    AddressType,
    InvoiceAddress,
};
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{
    api::Api,
    contexts::{
        auth_context::use_user_context,
        locale_context::use_locale,
    },
    sales::components::sales_invoice_drawer::address_edit_card::AddressEditCard,
};

#[derive(Clone, Debug, PartialEq)]
enum EditState {
    None,
    Editing(AddressType),
}

#[derive(Properties, PartialEq)]
pub struct AddressesViewProps {
    pub addresses: Vec<(AddressType, InvoiceAddress)>,
    pub invoice_id: Uuid,
    pub on_change: Callback<()>,
}

#[function_component(AddressesView)]
pub fn addresses_view(props: &AddressesViewProps) -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();
    let editing_state = use_state(|| EditState::None);
    let error = use_state(|| None::<String>);

    let on_edit_click = |address_type: AddressType| {
        let editing_state = editing_state.clone();
        let address_type = address_type.clone();
        Callback::from(move |_| editing_state.set(EditState::Editing(address_type.clone())))
    };

    let on_cancel = {
        let editing_state = editing_state.clone();
        Callback::from(move |_| editing_state.set(EditState::None))
    };

    // 💡 Handle saving changes from AddressEditCard to the backend API
    let on_address_save = {
        let user_ctx = user_ctx.clone();
        let invoice_id = props.invoice_id;
        let on_change = props.on_change.clone();
        let editing_state = editing_state.clone();
        let error = error.clone();
        let current_addresses = props.addresses.clone();
        let navigator = navigator.clone();

        Callback::from(
            move |(updated_address, address_type): (InvoiceAddress, AddressType)| {
                let user_ctx = user_ctx.clone();
                let on_change = on_change.clone();
                let editing_state = editing_state.clone();
                let error = error.clone();
                let current_addresses = current_addresses.clone();
                let navigator = navigator.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    // Find the existing opposite address so we don't clear it out
                    // when submitting the full invoice update request structure.
                    let mut bill_to = updated_address.clone();
                    let mut ship_to = updated_address.clone();

                    if address_type == AddressType::Billing {
                        if let Some((_, existing_ship)) = current_addresses
                            .iter()
                            .find(|(t, _)| *t == AddressType::Shipping)
                        {
                            ship_to = existing_ship.clone();
                        }
                    } else {
                        if let Some((_, existing_bill)) = current_addresses
                            .iter()
                            .find(|(t, _)| *t == AddressType::Billing)
                        {
                            bill_to = existing_bill.clone();
                        }
                    }

                    // Construct the full Update request required by your backend model spec
                    let request =
                        shared_core::sales::requests::sales_invoice::UpdateSalesInvoiceRequest {
                            id: invoice_id,
                            // If your backend endpoint updates JUST addresses via this route,
                            // these dates might be ignored, or you can map them from state.
                            issue_date: chrono::Local::now().date_naive(),
                            due_date: chrono::Local::now().date_naive(),
                            billing_address_id: None,
                            shipping_address_id: None,
                            bill_to,
                            ship_to,
                        };

                    let resp = Api::put(
                        &format!("/api/sales-invoices/{}", request.id),
                        &request,
                        user_ctx,
                        navigator,
                    )
                    .await;
                    match resp {
                        Ok(_) => {
                            error.set(None);
                            editing_state.set(EditState::None); // Drop edit card mode
                            on_change.emit(()); // Trigger top-level parent re-fetch
                        }
                        Err(e) => {
                            log::error!("Failed to update invoice address: {:?}", e);
                            error.set(Some(
                                "Failed to save address changes. Please try again.".to_string(),
                            ));
                        }
                    }
                });
            },
        )
    };

    html! {
        <div class="addresses-view">
            if let Some(e) = &*error {
                <div class="error">{e}</div>
            }

            <div class="card-grid">
                { for props.addresses.iter().map(|(address_type, address)| {
                    if *editing_state == EditState::Editing(address_type.clone()) {
                        html!{
                            <AddressEditCard
                                address_type={address_type.clone()}
                                address={address.clone()}
                                on_save={on_address_save.clone()}
                                on_cancel={on_cancel.clone()}
                            />
                        }
                    } else {
                        let mut card_class = classes!("card");
                            match address_type {
                                AddressType::Billing => card_class.push("card--primary-billing"),
                                AddressType::Shipping => card_class.push("card--primary-shipping"),
                            }

                        html! {
                            <div class={card_class}>
                                <div class="card__header">
                                    <h5>{ address_type.to_string() }</h5>
                                </div>
                                <div class="card__body">
                                    if let Some(line1) = &address.address_line1 {
                                        if !line1.is_empty() {
                                            <p>{ line1 }</p>
                                        }
                                    }
                                    if let Some(line2) = &address.address_line2 {
                                        if !line2.is_empty() {
                                            <p>{ line2 }</p>
                                        }
                                    }
                                    <p>{ format!("{}, {} {}", address.city.as_deref().unwrap_or(""), address.state_province.as_deref().unwrap_or(""), address.postal_code.as_deref().unwrap_or("")) }</p>
                                    <p>{ &address.country.as_deref().unwrap_or("") }</p>
                                </div>
                                <div class="card__footer">
                                    <button class="icon-button" onclick={on_edit_click(address_type.clone())} disabled={*editing_state != EditState::None}>
                                        <img src="/images/edit.svg" alt={i18n.t("common-edit")} />
                                    </button>
                                </div>
                            </div>
                        }
                    }
                })}
            </div>

        </div>
    }
}
