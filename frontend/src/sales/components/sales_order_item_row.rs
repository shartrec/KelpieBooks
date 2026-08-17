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
    sales_order_item::SalesOrderItem,
    tax::TaxRate,
};
use uuid::Uuid;
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use shared_core::inventory::dtos::inventory::ItemStockBalancesResponse;
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
pub struct SalesOrderItemRowProps {
    pub item: SalesOrderItem,
    pub on_change: Callback<SalesOrderItem>,
    pub on_delete: Callback<Uuid>,
    pub quantity_available: Option<Decimal>,
}

#[function_component(SalesOrderItemRow)]
pub fn sales_order_item_row(props: &SalesOrderItemRowProps) -> Html {
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
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        let error = error.clone();
        let i18n = i18n.clone();

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
                let user_ctx_tax = user_ctx.clone();
                let navigator_tax = navigator.clone();
                let error_tax = error.clone();
                let i18n_tax = i18n.clone();

                // 1. Future A: Resolves directly to Option<Decimal>
                let tax_rate_future = async move {
                    if let Some(tax_category_id) = new_item.tax_category_id {
                        let res = Api::get(
                            &format!("/api/sales/tax-categories/{}/current-rate", tax_category_id),
                            user_ctx_tax,
                            navigator_tax,
                        )
                            .await;

                        match res {
                            Ok(response) if response.ok() => {
                                match response.json::<Option<TaxRate>>().await {
                                    Ok(Some(tax_rate_data)) => Some(tax_rate_data.rate),
                                    Ok(None) => {
                                        info!("No current tax rate found for category {}", tax_category_id);
                                        None
                                    }
                                    Err(e) => {
                                        error_tax.set(Some(i18n_tax.t_args(
                                            "new-sales-invoice-error-parse-tax-rate",
                                            &fluent_args!["error" => e.to_string()],
                                        )));
                                        None
                                    }
                                }
                            }
                            Ok(response) => {
                                error_tax.set(Some(i18n_tax.t_args(
                                    "new-sales-invoice-error-fetch-tax-rate",
                                    &fluent_args!["status" => response.status()],
                                )));
                                None
                            }
                            Err(e) => {
                                error_tax.set(Some(i18n_tax.t_args(
                                    "common-network-error",
                                    &fluent_args!["error" => e.to_string()],
                                )));
                                None
                            }
                        }
                    } else {
                        None
                    }
                };

                // 2. Future B: Inventory Balances Request
                let url = format!("/api/inventory/items/{}/balances", selected_item.id);
                let balances_future = Api::get(
                    &url,
                    user_ctx.clone(),
                    navigator.clone(),
                );

                // 3. Await both concurrently
                let (fetched_tax_rate, balances_response) = futures::join!(tax_rate_future, balances_future);

                // Apply fetched tax rate if retrieved
                if let Some(rate) = fetched_tax_rate {
                    new_item.tax_rate = rate;
                }

                // Process Inventory Balances
                match balances_response {
                    Ok(response) if response.ok() => {
                        match response.json::<ItemStockBalancesResponse>().await {
                            Ok(balance_data) => {
                                new_item.quantity_available = balance_data.total_available;
                            }
                            Err(e) => error.set(Some(i18n.t_args(
                                "inventory-error-parse-balances",
                                &fluent_args!["error" => e.to_string()],
                            ))),
                        }
                    }
                    Ok(response) => {
                        info!("Failed to retrieve item balances, status: {}", response.status());
                    }
                    Err(e) => error.set(Some(i18n.t_args(
                        "common-network-error",
                        &fluent_args!["error" => e.to_string()],
                    ))),
                }

                // 4. Recalculate net_amount and tax_amount
                new_item.tax_amount =
                    new_item.net_amount * (new_item.tax_rate / Decimal::new(100, 0));

                on_change.emit(new_item);
                items.set(vec![]);
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

    // Render the availability badge based on quantity_available vs entered quantity
    let availability_badge = {
        let quantity = props.item.quantity;
        match props.quantity_available {
            None => html! {
                <span class="badge badge--neutral">{ "" }</span>
            },
            Some(avail) if quantity <= avail => {
                let badge = i18n.t_args("sales-order-item-available", &fluent_args!["qty" => avail.to_string()]);
                html! {
                    <span class="badge badge--success" title={badge.clone()}>
                        { badge }
                    </span>
                }
            },
            Some(avail) => {
                let badge = i18n.t_args("sales-order-item-insufficient-stock", &fluent_args!["qty" => avail.to_string()]);
                html! {
                    <span class="badge badge--warning" title={ badge.clone() }>
                        { badge }
                    </span>
                }
            },
        }
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
            <input type="text" class="table__text-col" value={props.item.description.clone()} readonly=true />
            <DecimalInput class="table__value-col" value={props.item.quantity} on_change={on_quantity_change} />
            { availability_badge }
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
