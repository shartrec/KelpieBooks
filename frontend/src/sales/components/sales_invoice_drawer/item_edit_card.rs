/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 * (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use fluent::fluent_args;
use log::info;
use rust_decimal::Decimal;
use shared_core::sales::models::{
    item::Item,
    sales_invoice_item::SalesInvoiceItem,
};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use shared_core::sales::models::tax::TaxRate;
use crate::{
    api::Api,
    contexts::{auth_context::use_user_context, locale_context::use_locale},
    core::components::{
        currency_input::DecimalInput,
        progressive_search::ProgressiveSearch,
        SearchableItem,
    },
};

#[derive(Properties, PartialEq, Clone)]
pub struct ItemEditCardProps {
    pub item: SalesInvoiceItem,
    pub on_save: Callback<SalesInvoiceItem>,
    pub on_cancel: Callback<()>,
}

#[function_component(ItemEditCard)]
pub fn item_edit_card(props: &ItemEditCardProps) -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();

    // Internal state tracking for the row payload entity
    let item_state = use_state(|| props.item.clone());

    // State wrappers for the progressive item catalog auto-suggest dropdown
    let items = use_state(Vec::<Item>::new);
    let item_search = use_state(|| props.item.name.clone());
    let error = use_state(|| None::<String>);

    // Handle searching catalog items via the backend API hookup
    let on_item_search = {
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        let items = items.clone();
        let item_search = item_search.clone();
        let navigator = navigator.clone();
        let error = error.clone();
        let i18n = i18n.clone();
        Callback::from(move | q: String| {
            item_search.set(q.clone());
            if q.is_empty() {
                items.set(vec![]);
                return;
            }
            let user_ctx = user_ctx.clone();
            let items = items.clone();
            let navigator = navigator.clone();
            let error = error.clone();
            let i18n = i18n.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_items = Api::get(
                    &format!("/api/sales/items?search_term={}&limit=20", q),
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

    // Handle choosing an item from the progressive search autocomplete dropdown list
    let on_item_select = {
        let item_state = item_state.clone();
        let item_search = item_search.clone();
        let items = items.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let error = error.clone();
        let navigator = navigator.clone();
        Callback::from(move |selected_item: Item| {
            item_search.set(selected_item.display_label());

            let mut updated = (*item_state).clone();
            updated.item_id = selected_item.id;
            updated.name = selected_item.name.clone();
            updated.description = selected_item.description.unwrap_or_default();
            updated.unit_price = selected_item.unit_price;
            updated.tax_category_id = selected_item.tax_category_id;
            updated.net_amount = updated.quantity * updated.unit_price;

            let item_state = item_state.clone();
            let items = items.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            let error = error.clone();
            let i18n = i18n.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(tax_category_id) = updated.tax_category_id {
                    let fetched_tax_rate = Api::get(
                        &format!("/api/sales/tax-categories/{}/current-rate", tax_category_id),
                        user_ctx,
                        navigator,
                    ).await;

                    match fetched_tax_rate {
                        Ok(response) if response.ok() => {
                            match response.json::<Option<TaxRate>>().await {
                                Ok(Some(tax_rate_data)) => {
                                    updated.tax_rate = tax_rate_data.rate;
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
                    // Compute updated monetary fields cleanly
                    updated.tax_amount = updated.net_amount * (updated.tax_rate / Decimal::new(100, 0));
                }
                item_state.set(updated);
                items.set(vec![]); // Close suggestions dropdown
            });

        })
    };

    // Generic description text modifier change input
    let on_description_change = {
        let item_state = item_state.clone();
        Callback::from(move |e: InputEvent| {
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            let mut updated = (*item_state).clone();
            updated.description = value;
            item_state.set(updated);
        })
    };

    // Quantity manipulation updates
    let on_quantity_change = {
        let item_state = item_state.clone();
        Callback::from(move |val: Decimal| {
            let mut updated = (*item_state).clone();
            updated.quantity = val;
            updated.net_amount = updated.quantity * updated.unit_price;
            updated.tax_amount = updated.net_amount * (updated.tax_rate / Decimal::new(100, 0));
            item_state.set(updated);
        })
    };

    // Unit Price manipulation updates
    let on_price_change = {
        let item_state = item_state.clone();
        Callback::from(move |val: Decimal| {
            let mut updated = (*item_state).clone();
            updated.unit_price = val;
            updated.net_amount = updated.quantity * updated.unit_price;
            updated.tax_amount = updated.net_amount * (updated.tax_rate / Decimal::new(100, 0));
            item_state.set(updated);
        })
    };

    let on_save_click = {
        let on_save = props.on_save.clone();
        let item_state = item_state.clone();
        Callback::from(move |_| {
            on_save.emit((*item_state).clone());
        })
    };

    let on_cancel_click = {
        let on_cancel = props.on_cancel.clone();
        Callback::from(move |_| {
            on_cancel.emit(());
        })
    };

    let current_item = (*item_state).clone();

    html! {
        <div class="card edit-card">
            <div class="card-header">
                <h3>{ i18n.t("item-edit-card-add-title") }</h3>
            </div>
            <div class="card-body">
                <div class="data-form">
                    // Item Catalog Autocomplete Selector
                    <label>{ i18n.t("common-item") }</label>
                    <ProgressiveSearch<Item>
                        placeholder={i18n.t("item-search-placeholder")}
                        query={(*item_search).clone()}
                        suggestions={(*items).clone()}
                        on_input={on_item_search}
                        on_select={on_item_select}
                    />

                    // Custom Line Override Description Field
                    <label>{ i18n.t("common-description") }</label>
                    <input
                        type="text"
                        value={current_item.description.clone()}
                        oninput={on_description_change}
                    />

                    // Quantity Field
                    <label>{ i18n.t("common-quantity") }</label>
                    <DecimalInput value={current_item.quantity} on_change={on_quantity_change} />

                    // Unit Price Field
                    <label>{ i18n.t("common-price") }</label>
                    <DecimalInput value={current_item.unit_price} on_change={on_price_change} />

                    // Display Fields for Tax Rate & Computed Values
                    <label>{ i18n.t("common-tax-rate") }</label>
                    <div class="form-display-value">{ i18n.format_percentage(current_item.tax_rate) }</div>

                    <label>{ i18n.t("item-edit-card-tax-amount-label") }</label>
                    <div class="form-display-value">{ i18n.format_currency(current_item.tax_amount) }</div>

                    <label>{ i18n.t("item-edit-card-net-amount-label") }</label>
                    <div class="form-display-value">{ i18n.format_currency(current_item.net_amount) }</div>

                    <label>{ i18n.t("common-total") }</label>
                    <div class="total-amount form-display-value" style="font-weight: bold;">
                        { i18n.format_currency(current_item.net_amount + current_item.tax_amount) }
                    </div>
                </div>
            </div>

            if let Some(err_msg) = &*error {
                <div class="error-message" style="padding: 0 1rem; color: var(--color-error);">{ err_msg }</div>
            }

            <div class="card-footer" style="display: flex; justify-content: flex-end; gap: 8px; padding: 1rem;">
                <button class="button-secondary" onclick={on_cancel_click}>{ i18n.t("common-cancel") }</button>
                <button class="button-primary" onclick={on_save_click}>{ i18n.t("common-save") }</button>
            </div>
        </div>
    }
}