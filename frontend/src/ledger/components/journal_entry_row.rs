/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use std::str::FromStr;

use rust_decimal::{
    dec,
    Decimal,
};
use shared_core::{
    ledger::requests::transaction::JournalEntryLine,
    AccountId,
};
use web_sys::HtmlSelectElement;
use yew::prelude::*;

use crate::{
    contexts::locale_context::use_locale,
    core::components::currency_input::DecimalInput,
};

#[derive(Properties, PartialEq)]
pub struct JournalEntryRowProps {
    pub entry: JournalEntryLine,
    pub on_change: Callback<JournalEntryLine>,
    pub on_delete: Callback<()>,
    pub accounts: Vec<(AccountId, String)>,
    #[prop_or(false)]
    pub should_focus: bool,
}

#[function_component(JournalEntryRow)]
pub fn journal_entry_row(props: &JournalEntryRowProps) -> Html {
    let i18n = use_locale();

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
            if let Ok(id) = AccountId::from_str(&value) {
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

    // Updated: Accept Decimal directly, no more f64 parsing here!
    let on_debit_change = {
        let on_change = props.on_change.clone();
        let entry = props.entry.clone();
        Callback::from(move |amount: Decimal| {
            let mut updated_entry = entry.clone();
            updated_entry.debit = amount;
            updated_entry.credit = dec!(0.00); // Maintain mutual exclusivity
            on_change.emit(updated_entry);
        })
    };

    let on_credit_change = {
        let on_change = props.on_change.clone();
        let entry = props.entry.clone();
        Callback::from(move |amount: Decimal| {
            let mut updated_entry = entry.clone();
            updated_entry.credit = amount;
            updated_entry.debit = dec!(0.00); // Maintain mutual exclusivity
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
                <option value="" disabled=true selected={props.entry.account_id.is_nil()}>{ i18n.t("journal-entry-select-account") }</option>
                { for props.accounts.iter().map(|(id, name)| html! {
                    <option value={id.to_string()} selected={*id == props.entry.account_id}>{name}</option>
                })}
            </select>
            <input type="text" placeholder={i18n.t("journal-entry-description-placeholder")}
                   value={props.entry.description.clone().unwrap_or_default()}
                   oninput={on_description_change} />

            // USE THE SPECIALIZED COMPONENT HERE
            <DecimalInput
                value={props.entry.debit}
                on_change={on_debit_change}
                placeholder={i18n.t("journal-entry-currency-placeholder")}
            />
            <DecimalInput
                value={props.entry.credit}
                on_change={on_credit_change}
                placeholder={i18n.t("journal-entry-currency-placeholder")}
            />

            <button class="icon-button btn-action" onclick={on_delete_click}>
                <img src="/images/delete.svg" alt={i18n.t("common-delete")} />
            </button>
        </div>
    }
}
