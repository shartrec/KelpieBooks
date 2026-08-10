/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use shared_core::sales::models::{
    invoice_address::AddressType,
    sales_order::SalesOrder,
};
use yew::prelude::*;

use crate::contexts::locale_context::use_locale;

#[derive(Properties, PartialEq, Clone)]
pub struct AddressesViewProps {
    pub order: SalesOrder,
}

#[function_component(AddressesView)]
pub fn addresses_view(props: &AddressesViewProps) -> Html {
    let i18n = use_locale();
    let order = &props.order;

    let addresses = vec![
        (AddressType::Billing, &order.bill_to),
        (AddressType::Shipping, &order.ship_to),
    ];

    html! {
        <div class="addresses-view">
            <div class="card-grid">
                { for addresses.iter().map(|(address_type, address)| {
                    let card_class = match address_type {
                        AddressType::Billing  => classes!("card", "card--primary-billing"),
                        AddressType::Shipping => classes!("card", "card--primary-shipping"),
                    };

                    html! {
                        <div class={card_class}>
                            <div class="card__header">
                                <h5>{ address_type.to_string() }</h5>
                            </div>
                            <div class="card__body">
                                if let Some(name) = &address.name {
                                    if !name.is_empty() { <p><strong>{ name }</strong></p> }
                                }
                                if let Some(attn) = &address.attention {
                                    if !attn.is_empty() {
                                        <p class="card-item-compact__desc">{ attn }</p>
                                    }
                                }
                                if let Some(line1) = &address.address_line1 {
                                    if !line1.is_empty() { <p>{ line1 }</p> }
                                }
                                if let Some(line2) = &address.address_line2 {
                                    if !line2.is_empty() { <p>{ line2 }</p> }
                                }
                                <p>{ format!("{}, {} {}",
                                    address.city.as_deref().unwrap_or(""),
                                    address.state_province.as_deref().unwrap_or(""),
                                    address.postal_code.as_deref().unwrap_or(""))
                                }</p>
                                if let Some(country) = &address.country {
                                    if !country.is_empty() { <p>{ country }</p> }
                                }
                            </div>
                            <div class="card__footer">
                                <button
                                    class="icon-button"
                                    disabled=true
                                    title={ i18n.t("sales-order-drawer-address-edit-future") }
                                >
                                    <img src="/images/edit.svg" alt={ i18n.t("common-edit") } />
                                </button>
                            </div>
                        </div>
                    }
                }) }
            </div>
        </div>
    }
}
