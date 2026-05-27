/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use crate::services::web::detect_browser_locale;
use chrono::{Datelike, NaiveDate};
use fluent::FluentArgs;
use shared_core::i18n::{format_currency_icu, format_date_icu};
use yew::prelude::*;
// Your shared core!

#[derive(Clone, PartialEq)]
pub struct LocaleContext {
    pub current: UseStateHandle<String>,
}

impl LocaleContext {
    pub fn as_str(&self) -> &str {
        &self.current
    }

    pub fn set(&self, new_locale: String) {
        self.current.set(new_locale);
    }

    pub fn t(&self, key: &str) -> String {
        shared_core::i18n::t(key, Some(self.as_str()))
    }

    /// Dynamic translation with arguments pass-through
    pub fn t_args(&self, key: &str, args: &FluentArgs) -> String {
        shared_core::i18n::t_args(key, args, Some(self.as_str()))
    }

    // Pass-through wrapper for currency
    pub fn format_currency(&self, amount_cents: i64) -> String {
        format_currency_icu(amount_cents, Some(self.as_str()))
    }

    // Pass-through wrapper for dates
    pub fn format_date(&self, date: NaiveDate) -> String {

        format_date_icu(date.year(), date.month(), date.day(), Some(self.as_str()))
    }
}

#[derive(Properties, PartialEq)]
pub struct LocaleProviderProps {
    pub children: Children,
}

#[component]
pub fn LocaleProvider(props: &LocaleProviderProps) -> Html {
    let current = use_state(|| detect_browser_locale());
    let context = LocaleContext { current };

    html! {
        <ContextProvider<LocaleContext> context={context}>
            { props.children.clone() }
        </ContextProvider<LocaleContext>>
    }
}

#[hook]
pub fn use_locale() -> LocaleContext {
    use_context::<LocaleContext>().expect("use_locale must be used inside a LocaleProvider")
}