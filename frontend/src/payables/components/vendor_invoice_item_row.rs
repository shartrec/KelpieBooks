/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use std::str::FromStr;

use rust_decimal::Decimal;
use shared_core::{
    ledger::models::account::Account,
    payables::models::vendor_invoice_item::VendorInvoiceItem,
    AccountId,
    InvoiceItemId,
};
use yew::prelude::*;

use crate::{
    contexts::locale_context::use_locale,
    core::components::currency_input::DecimalInput,
};

#[derive(Properties, PartialEq)]
pub struct VendorInvoiceItemRowProps {
    pub item: VendorInvoiceItem,
    pub accounts: Vec<Account>,
    pub on_change: Callback<VendorInvoiceItem>,
    pub on_delete: Callback<InvoiceItemId>,
}

#[function_component(VendorInvoiceItemRow)]
pub fn vendor_invoice_item_row(props: &VendorInvoiceItemRowProps) -> Html {
    let i18n = use_locale();

    let on_description_change = {
        let on_change = props.on_change.clone();
        let item = props.item.clone();
        Callback::from(move |e: InputEvent| {
            let mut new_item = item.clone();
            new_item.description = Some(
                e.target_unchecked_into::<web_sys::HtmlInputElement>()
                    .value(),
            );
            on_change.emit(new_item);
        })
    };

    let on_account_change = {
        let on_change = props.on_change.clone();
        let item = props.item.clone();
        Callback::from(move |e: Event| {
            let mut new_item = item.clone();
            let value = e
                .target_unchecked_into::<web_sys::HtmlSelectElement>()
                .value();
            new_item.account_id = AccountId::from_str(&value).unwrap_or_default();
            on_change.emit(new_item);
        })
    };

    let on_net_amount_change = {
        let on_change = props.on_change.clone();
        let item = props.item.clone();
        Callback::from(move |value: Decimal| {
            let mut new_item = item.clone();
            new_item.net_amount = value;
            new_item.total_amount = new_item.net_amount + new_item.tax_amount;
            on_change.emit(new_item);
        })
    };

    let on_tax_amount_change = {
        let on_change = props.on_change.clone();
        let item = props.item.clone();
        Callback::from(move |value: Decimal| {
            let mut new_item = item.clone();
            new_item.tax_amount = value;
            new_item.total_amount = new_item.net_amount + new_item.tax_amount;
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
            <input type="text" value={props.item.description.clone()} oninput={on_description_change} />
            <select onchange={on_account_change}>
                <option value="" disabled=true selected={props.item.account_id.is_nil()}>{i18n.t("journal-entry-select-account")}</option>
                { for props.accounts.iter().map(|acc| html! {
                    <option value={acc.id.to_string()} selected={props.item.account_id == acc.id}>{&acc.name}</option>
                })}
            </select>
            <DecimalInput value={props.item.net_amount} on_change={on_net_amount_change} />
            <DecimalInput value={props.item.tax_amount} on_change={on_tax_amount_change} />
            <DecimalInput value={props.item.total_amount} on_change={Callback::noop()} />
            <button class="icon-button btn-action" onclick={on_delete_click}>
                <img src="/images/delete.svg" alt={i18n.t("common-delete")} />
            </button>

        </div>
    }
}
