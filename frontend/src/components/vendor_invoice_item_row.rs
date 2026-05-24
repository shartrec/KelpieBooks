/*
 * Copyright (c) 2026-2026. Trevor Campbell and others.
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

use crate::components::currency_input::CurrencyInput;
use shared_core::i18n::t;
use shared_core::models::account::Account;
use shared_core::models::vendor_invoice_item::VendorInvoiceItem;
use uuid::Uuid;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct VendorInvoiceItemRowProps {
    pub item: VendorInvoiceItem,
    pub accounts: Vec<Account>,
    pub on_change: Callback<VendorInvoiceItem>,
    pub on_delete: Callback<Uuid>,
}

#[function_component(VendorInvoiceItemRow)]
pub fn vendor_invoice_item_row(props: &VendorInvoiceItemRowProps) -> Html {
    let on_description_change = {
        let on_change = props.on_change.clone();
        let item = props.item.clone();
        Callback::from(move |e: InputEvent| {
            let mut new_item = item.clone();
            new_item.description = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
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
            new_item.account_id = Uuid::parse_str(&value).unwrap_or_default();
            on_change.emit(new_item);
        })
    };

    let on_net_amount_change = {
        let on_change = props.on_change.clone();
        let item = props.item.clone();
        Callback::from(move |value: i64| {
            let mut new_item = item.clone();
            new_item.net_amount = value;
            new_item.total_amount = new_item.net_amount + new_item.tax_amount;
            on_change.emit(new_item);
        })
    };

    let on_tax_amount_change = {
        let on_change = props.on_change.clone();
        let item = props.item.clone();
        Callback::from(move |value: i64| {
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
                <option value="" disabled=true selected={props.item.account_id.is_nil()}>{t("journal-entry-select-account")}</option>
                { for props.accounts.iter().map(|acc| html! {
                    <option value={acc.id.to_string()} selected={props.item.account_id == acc.id}>{&acc.name}</option>
                })}
            </select>
            <CurrencyInput value={props.item.net_amount} on_change={on_net_amount_change} />
            <CurrencyInput value={props.item.tax_amount} on_change={on_tax_amount_change} />
            <CurrencyInput value={props.item.total_amount} on_change={Callback::noop()} />
            <button type="button" onclick={on_delete_click} class="icon-button">{ t("journal-entry-delete-button") }</button>
        </div>
    }
}
