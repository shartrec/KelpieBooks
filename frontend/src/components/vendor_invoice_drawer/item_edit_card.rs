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
use shared_core::util::format_currency;
use uuid::Uuid;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ItemEditCardProps {
    pub item: VendorInvoiceItem,
    pub accounts: Vec<Account>,
    pub on_save: Callback<VendorInvoiceItem>,
    pub on_cancel: Callback<()>,
}

#[function_component(ItemEditCard)]
pub fn item_edit_card(props: &ItemEditCardProps) -> Html {
    let item = use_state(|| props.item.clone());

    let on_input = |field_updater: fn(&mut VendorInvoiceItem, String)| {
        let state = item.clone();
        Callback::from(move |e: InputEvent| {
            let mut info = (*state).clone();
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            field_updater(&mut info, value);
            state.set(info);
        })
    };

    let on_select_change = |field_updater: fn(&mut VendorInvoiceItem, String)| {
        let state = item.clone();
        Callback::from(move |e: Event| {
            let mut info = (*state).clone();
            let value = e.target_unchecked_into::<HtmlSelectElement>().value();
            field_updater(&mut info, value);
            state.set(info);
        })
    };

    let on_net_amount_change = {
        let item = item.clone();
        Callback::from(move |value: i64| {
            let mut new_item = (*item).clone();
            new_item.net_amount = value;
            new_item.total_amount = new_item.net_amount + new_item.tax_amount;
            item.set(new_item);
        })
    };

    let on_tax_amount_change = {
        let item = item.clone();
        Callback::from(move |value: i64| {
            let mut new_item = (*item).clone();
            new_item.tax_amount = value;
            new_item.total_amount = new_item.net_amount + new_item.tax_amount;
            item.set(new_item);
        })
    };

    let on_save = {
        let on_save = props.on_save.clone();
        let item = item.clone();
        Callback::from(move |_| {
            on_save.emit((*item).clone());
        })
    };

    let on_cancel = {
        let on_cancel = props.on_cancel.clone();
        Callback::from(move |_| {
            on_cancel.emit(());
        })
    };

    html! {
        <div class="card card--editing" style="padding: 1rem; margin-bottom: 1rem;">
            <div class="card__meta-line" style="margin-bottom: 0.75rem;">
                <div class="card__title">
                    <strong style="font-size: 0.85rem; text-transform: uppercase; color: var(--brand-dark);">
                        { if props.item.description.is_empty() { t("item-edit-card-add-title") } else { t("item-edit-card-edit-title") } }
                    </strong>
                </div>
            </div>

            <div class="card-form-compact">
                    <label>{t("common-description")}</label>
                    <input type="text" value={item.description.clone()} oninput={on_input(|i, v| i.description = v)} />

                    <label>{t("common-account")}</label>
                    <select onchange={on_select_change(|i, v| i.account_id = Uuid::parse_str(&v).unwrap_or_default())}>
                        <option value="" disabled=true selected={item.account_id.is_nil()}>{t("journal-entry-select-account")}</option>
                        { for props.accounts.iter().map(|account| html! {
                            <option value={account.id.to_string()} selected={item.account_id == account.id}>{&account.name}</option>
                        })}
                    </select>

                    <label>{t("item-edit-card-net-amount-label")}</label>
                    <CurrencyInput value={item.net_amount} on_change={on_net_amount_change} />

                    <label>{t("item-edit-card-tax-amount-label")}</label>
                    <CurrencyInput value={item.tax_amount} on_change={on_tax_amount_change} />

                    <label>{t("common-total")}</label>
                    <div class="total-amount">
                        {format_currency(&item.total_amount)}
                    </div>
            </div>
            <div class="card-footer">
                <button class="button-primary" onclick={on_save}>{ t("common-save") }</button>
                <button class="button-secondary" onclick={on_cancel}>{ t("common-cancel") }</button>
            </div>
        </div>
    }
}
