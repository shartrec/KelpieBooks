/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use fluent::fluent_args;
use log::info;
use rust_decimal::Decimal;
use shared_core::sales::models::{
    item::Item,
    sales_invoice_item::SalesInvoiceItem,
    tax::TaxRate, // Import TaxRate
};
use uuid::Uuid;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{
    api::Api,
    contexts::{
        auth_context::use_user_context,
        locale_context::use_locale,
    },
    core::components::{
        currency_input::DecimalInput,
        progressive_search::ProgressiveSearch,
        SearchableItem,
    },
};

#[derive(Properties, PartialEq)]
pub struct SalesInvoiceItemRowProps {
    pub item: SalesInvoiceItem,
    pub on_change: Callback<SalesInvoiceItem>,
    pub on_delete: Callback<Uuid>,
}

#[function_component(SalesInvoiceItemRow)]
pub fn sales_invoice_item_row(props: &SalesInvoiceItemRowProps) -> Html {
    let i18n = use_locale();
    let items = use_state(Vec::new);
    let item_search = use_state(String::new);
    let error = use_state(|| None::<String>);
    let user_ctx = use_user_context();
    let navigator = use_navigator().unwrap();

    {
        let item_search = item_search.clone();
        let current_item_name = props.item.name.clone();
        use_effect_with(current_item_name, move |name| {
            item_search.set(name.clone());
            || ()
        });
    }

    let on_item_search = {
        let items = items.clone();
        let item_search = item_search.clone();
        let error = error.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        Callback::from(move |text: String| {
            item_search.set(text.clone());
            let items = items.clone();
            let error = error.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_items = Api::get(
                    &format!("/api/sales/items?search_term={}&limit=20", text),
                    user_ctx,
                    navigator,
                )
                .await;
                match fetched_items {
                    Ok(response) if response.ok() => match response.json::<Vec<Item>>().await {
                        Ok(data) => items.set(data),
                        Err(e) => error.set(Some(i18n.t_args(
                            "new-sales-invoice-error-parse-items",
                            &fluent_args!["error" => e.to_string()],
                        ))),
                    },
                    Ok(response) => error.set(Some(i18n.t_args(
                        "new-sales-invoice-error-fetch-items",
                        &fluent_args!["status" => response.status()],
                    ))),
                    Err(e) => error.set(Some(i18n.t_args(
                        "common-network-error",
                        &fluent_args!["error" => e.to_string()],
                    ))),
                }
            });
        })
    };
    let on_item_select = {
        let on_change = props.on_change.clone();
        let item = props.item.clone();
        let items = items.clone();
        let item_search = item_search.clone();
        let user_ctx = user_ctx.clone(); // Clone user_ctx for the async block
        let navigator = navigator.clone(); // Clone navigator for the async block
        let error = error.clone(); // Clone error for the async block
        let i18n = i18n.clone(); // Clone i18n for the async block

        Callback::from(move |selected_item: Item| {
            item_search.set(selected_item.display_label());

            let mut new_item = item.clone();
            new_item.item_id = selected_item.id;
            new_item.name = selected_item.name.clone();
            new_item.description = selected_item.description.unwrap_or("".to_string()).clone();
            new_item.unit_price = selected_item.unit_price;
            new_item.tax_category_id = selected_item.tax_category_id;
            new_item.tax_rate = Decimal::new(0, 4); // Default to 0
            new_item.net_amount = new_item.quantity * new_item.unit_price;

            let on_change = on_change.clone();
            let items = items.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            let error = error.clone();
            let i18n = i18n.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if let Some(tax_category_id) = new_item.tax_category_id {
                    let fetched_tax_rate = Api::get(
                        &format!("/api/sales/tax-categories/{}/current-rate", tax_category_id),
                        user_ctx,
                        navigator,
                    )
                    .await;

                    match fetched_tax_rate {
                        Ok(response) if response.ok() => {
                            match response.json::<Option<TaxRate>>().await {
                                Ok(Some(tax_rate_data)) => {
                                    new_item.tax_rate = tax_rate_data.rate;
                                }
                                Ok(None) => {
                                    info!(
                                        "No current tax rate found for category {}",
                                        tax_category_id
                                    );
                                }
                                Err(e) => error.set(Some(i18n.t_args(
                                    "new-sales-invoice-error-parse-tax-rate",
                                    &fluent_args!["error" => e.to_string()],
                                ))),
                            }
                        }
                        Ok(response) => error.set(Some(i18n.t_args(
                            "new-sales-invoice-error-fetch-tax-rate",
                            &fluent_args!["status" => response.status()],
                        ))),
                        Err(e) => error.set(Some(i18n.t_args(
                            "common-network-error",
                            &fluent_args!["error" => e.to_string()],
                        ))),
                    }
                    // Recalculate net_amount and tax_amount
                    new_item.tax_amount =
                        new_item.net_amount * (new_item.tax_rate / Decimal::new(100, 0));

                    on_change.emit(new_item);
                    items.set(vec![]);
                }

            });
        })
    };

    let on_quantity_change = {
        let on_change = props.on_change.clone();
        let item = props.item.clone();
        Callback::from(move |value: Decimal| {
            let mut new_item = item.clone();
            new_item.quantity = value;
            new_item.net_amount = new_item.quantity * new_item.unit_price;
            new_item.tax_amount = new_item.net_amount * (new_item.tax_rate / Decimal::new(100, 0));
            on_change.emit(new_item);
        })
    };

    let on_price_change = {
        let on_change = props.on_change.clone();
        let item = props.item.clone();
        Callback::from(move |value: Decimal| {
            let mut new_item = item.clone();
            new_item.unit_price = value;
            new_item.net_amount = new_item.quantity * new_item.unit_price;
            new_item.tax_amount = new_item.net_amount * (new_item.tax_rate / Decimal::new(100, 0));
            on_change.emit(new_item);
        })
    };

    let on_delete_click = {
        let on_delete = props.on_delete.clone();
        let item_id = props.item.id;
        Callback::from(move |_| {
            on_delete.emit(item_id);
        })
    };

    html! {
        <div class="sale__entry-row">
            <ProgressiveSearch<Item>
                placeholder="Search catalog items..."
                query={(*item_search).clone()}
                suggestions={(*items).clone()}
                on_input={on_item_search}
                on_select={on_item_select}
            />
            <input type="text"  class="table__text-col" value={props.item.description.clone()} readonly=true />
            <DecimalInput class="table__value-col" value={props.item.quantity} on_change={on_quantity_change} />
            <DecimalInput class="table__value-col" value={props.item.unit_price} on_change={on_price_change} />
            // Display the tax rate
            <DecimalInput class="table__value-col" value={props.item.tax_rate} on_change={Callback::noop()} readonly=true />
            <DecimalInput class="table__value-col" value={props.item.tax_amount} on_change={Callback::noop()} />
            <DecimalInput class="table__value-col" value={props.item.net_amount} on_change={Callback::noop()} />
            <button class="icon-button btn-action" onclick={on_delete_click}>
                <img src="/images/delete.svg" alt={i18n.t("common-delete")} />
            </button>
        </div>
    }
}
