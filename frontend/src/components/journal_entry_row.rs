/*
 * Copyright (c) 2026. Trevor Campbell and others.
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
use log::info;
use shared_core::requests::transaction::JournalEntryLine;
use uuid::Uuid;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use shared_core::util::format_currency;

#[derive(Properties, PartialEq)]
pub struct JournalEntryRowProps {
    pub entry: JournalEntryLine,
    pub on_change: Callback<JournalEntryLine>,
    pub on_delete: Callback<()>,
    pub accounts: Vec<(Uuid, String)>, // All postable accounts
}

#[function_component(JournalEntryRow)]
pub fn journal_entry_row(props: &JournalEntryRowProps) -> Html {
    let on_account_change = {
        let on_change = props.on_change.clone();
        let entry = props.entry.clone();
        Callback::from(move |e: Event| {
            let value = e.target_unchecked_into::<HtmlSelectElement>().value();
            if let Ok(id) = Uuid::parse_str(&value) {
                let mut updated_entry = entry.clone();
                updated_entry.account_id = id;
                on_change.emit(updated_entry);
            }
        })
    };

    let on_description_change = {
        let on_change = props.on_change.clone();
        let entry = props.entry.clone();
        Callback::from(move |e: InputEvent| {
            let value = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            let mut updated_entry = entry.clone();
            updated_entry.description = if value.is_empty() { None } else { Some(value) };
            on_change.emit(updated_entry);
        })
    };

    let on_debit_change = {
        let on_change = props.on_change.clone();
        let entry = props.entry.clone();
        Callback::from(move |e: InputEvent| {
            let value = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            if let Ok(amount) = value.parse::<f64>() {
                let mut updated_entry = entry.clone();
                updated_entry.debit = (amount * 100.0).round() as i64;
                updated_entry.credit = 0; // Ensure debit and credit are mutually exclusive
                info!("Entry as cents = {}, formatted {}", amount, updated_entry.debit);
                on_change.emit(updated_entry);
            }
        })
    };

    let on_credit_change = {
        let on_change = props.on_change.clone();
        let entry = props.entry.clone();
        Callback::from(move |e: InputEvent| {
            let value = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            if let Ok(amount) = value.parse::<f64>() {
                let mut updated_entry = entry.clone();
                updated_entry.credit = (amount * 100.0).round() as i64;
                updated_entry.debit = 0; // Ensure debit and credit are mutually exclusive
                info!("Entry as cents = {}, formatted {}", amount,  updated_entry.credit);
                on_change.emit(updated_entry);
            }
        })
    };

    let on_delete_click = {
        let on_delete = props.on_delete.clone();
        Callback::from(move |_| {
            on_delete.emit(());
        })
    };

    html! {
        <div class="journal-entry-row">
            <select onchange={on_account_change}>
                <option value="" disabled=true selected={props.entry.account_id.is_nil()}>{ "Select Account" }</option>
                { for props.accounts.iter().map(|(id, name)| html! {
                    <option value={id.to_string()} selected={*id == props.entry.account_id}>{name}</option>
                })}
            </select>
            <input type="text" placeholder="Description" value={props.entry.description.clone().unwrap_or_default()} oninput={on_description_change} />
            <input type="number" step="0.01" placeholder="Debit" value={format_currency(&props.entry.debit)} oninput={on_debit_change} />
            <input type="number" step="0.01" placeholder="Credit" value={format_currency(&props.entry.credit)} oninput={on_credit_change} />
            <button type="button" onclick={on_delete_click} class="icon-button">{ "X" }</button>
        </div>
    }
}
