/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use rust_decimal::Decimal;
use shared_core::sales::{
    models::{
        sales_invoice_item::SalesInvoiceLine,
        item::Item,
    },
};
use uuid::Uuid;
use yew::prelude::*;

use crate::{
    core::components::currency_input::DecimalInput,
    contexts::locale_context::use_locale,
};

#[derive(Properties, PartialEq)]
pub struct SalesInvoiceItemRowProps {
    pub item: SalesInvoiceLine,
    pub items: Vec<Item>,
    pub on_change: Callback<SalesInvoiceLine>,
    pub on_delete: Callback<Uuid>,
}

#[function_component(SalesInvoiceItemRow)]
pub fn sales_invoice_item_row(props: &SalesInvoiceItemRowProps) -> Html {
    let i18n = use_locale();

    let on_item_change = {
        let on_change = props.on_change.clone();
        let item = props.item.clone();
        let items = props.items.clone();
        Callback::from(move |e: Event| {
            let mut new_item = item.clone();
            let value = e
                .target_unchecked_into::<web_sys::HtmlSelectElement>()
                .value();
            new_item.item_id = Uuid::parse_str(&value).unwrap_or_default();
            if let Some(selected_item) = items.iter().find(|i| i.id == new_item.item_id) {
                new_item.description = selected_item.name.clone();
                new_item.unit_price = selected_item.unit_price;
                new_item.tax_category_id = selected_item.tax_category_id;
            }
            on_change.emit(new_item);
        })
    };

    let on_quantity_change = {
        let on_change = props.on_change.clone();
        let item = props.item.clone();
        Callback::from(move |value: Decimal| {
            let mut new_item = item.clone();
            new_item.quantity = value;
            on_change.emit(new_item);
        })
    };

    let on_price_change = {
        let on_change = props.on_change.clone();
        let item = props.item.clone();
        Callback::from(move |value: Decimal| {
            let mut new_item = item.clone();
            new_item.unit_price = value;
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
        <div class="voucher__entry-row">
            <input type="text" value={props.item.description.clone()} readonly=true />
            <select onchange={on_item_change}>
                <option value="" disabled=true selected={props.item.item_id.is_nil()}>{i18n.t("new-sales-invoice-select-item")}</option>
                { for props.items.iter().map(|i| html! {
                    <option value={i.id.to_string()} selected={props.item.item_id == i.id}>{&i.name}</option>
                })}
            </select>
            <DecimalInput value={props.item.quantity} on_change={on_quantity_change} />
            <DecimalInput value={props.item.unit_price} on_change={on_price_change} />
            <DecimalInput value={props.item.tax_amount} on_change={Callback::noop()} />
            <DecimalInput value={props.item.line_total} on_change={Callback::noop()} />
            <button class="icon-button btn-action" onclick={on_delete_click}>
                <img src="/images/delete.svg" alt={i18n.t("common-delete")} />
            </button>
        </div>
    }
}
