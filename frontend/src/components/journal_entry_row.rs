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
use crate::components::currency_input::CurrencyInput;
use shared_core::requests::transaction::JournalEntryLine;
use uuid::Uuid;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct JournalEntryRowProps {
    pub entry: JournalEntryLine,
    pub on_change: Callback<JournalEntryLine>,
    pub on_delete: Callback<()>,
    pub accounts: Vec<(Uuid, String)>,
    #[prop_or(false)]
    pub should_focus: bool,
}

#[function_component(JournalEntryRow)]
pub fn journal_entry_row(props: &JournalEntryRowProps) -> Html {
    let select_ref = use_node_ref(); // Create the reference

    // Effect that runs when 'should_focus' changes
    use_effect_with(props.should_focus, {
        let select_ref = select_ref.clone();
        move |&should_focus| {
            if should_focus {
                if let Some(element) = select_ref.cast::<HtmlSelectElement>() {
                    let _ = element.focus(); // Focus the account selector
                }
            }
            || ()
        }
    });

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

    // Updated: Accept i64 directly, no more f64 parsing here!
    let on_debit_change = {
        let on_change = props.on_change.clone();
        let entry = props.entry.clone();
        Callback::from(move |cents: i64| {
            let mut updated_entry = entry.clone();
            updated_entry.debit = cents;
            updated_entry.credit = 0; // Maintain mutual exclusivity
            on_change.emit(updated_entry);
        })
    };

    let on_credit_change = {
        let on_change = props.on_change.clone();
        let entry = props.entry.clone();
        Callback::from(move |cents: i64| {
            let mut updated_entry = entry.clone();
            updated_entry.credit = cents;
            updated_entry.debit = 0; // Maintain mutual exclusivity
            on_change.emit(updated_entry);
        })
    };

    let on_delete_click = {
        let on_delete = props.on_delete.clone();
        Callback::from(move |_| {
            on_delete.emit(());
        })
    };

    html! {
        <div class="journal__entry-row">
            <select ref={select_ref} onchange={on_account_change}>
                <option value="" disabled=true selected={props.entry.account_id.is_nil()}>{ "Select Account" }</option>
                { for props.accounts.iter().map(|(id, name)| html! {
                    <option value={id.to_string()} selected={*id == props.entry.account_id}>{name}</option>
                })}
            </select>
            <input type="text" placeholder="Description"
                   value={props.entry.description.clone().unwrap_or_default()}
                   oninput={on_description_change} />

            // USE THE SPECIALIZED COMPONENT HERE
            <CurrencyInput
                value={props.entry.debit}
                on_change={on_debit_change}
                placeholder="0.00"
            />
            <CurrencyInput
                value={props.entry.credit}
                on_change={on_credit_change}
                placeholder="0.00"
            />

            <button type="button" onclick={on_delete_click} class="icon-button">{ "X" }</button>
        </div>
    }
}
