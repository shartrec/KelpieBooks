/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::{
    Datelike,
    NaiveDate,
};
use fluent::FluentArgs;
use rust_decimal::Decimal;
use shared_core::i18n::format_date_icu;

/// A lightweight, stateless context helper to match frontend ergonomics on the backend.
pub(crate) struct LocaleContext<'a> {
    locale: &'a str,
}

impl<'a> LocaleContext<'a> {
    /// Create a new, non-magical localization instance bound to a target locale
    pub(crate) fn new(locale: &'a str) -> Self {
        LocaleContext { locale }
    }

    /// Read the active locale string
    pub(crate) fn as_str(&self) -> &str {
        self.locale
    }

    /// Translates a plain string key matching the active locale context
    pub(crate) fn t(&self, key: &str) -> String {
        shared_core::i18n::t(key, Some(self.locale)) // Delegates to your core function
    }

    /// Translates a string key with dynamic fluent variable interpolation arguments
    pub(crate) fn t_args(&self, key: &str, args: &FluentArgs) -> String {
        shared_core::i18n::t_args(key, args, Some(self.locale)) // Delegates to your core function
    }

    /// Standard currency formatting ("1,234.56")
    pub(crate) fn format_money(&self, amount: Decimal) -> String {
        shared_core::i18n::format_currency_icu(amount, Some(self.locale)) //
    }

    /// Typst-safe currency formatting ("−1,234.56")
    pub(crate) fn format_money_typ(&self, amount: Decimal) -> String {
        shared_core::i18n::format_currency_icu_typ(amount, Some(self.locale))
        //
    }

    /// Date formatting ("25 May 2026")
    pub(crate) fn format_date(&self, date: NaiveDate) -> String {
        format_date_icu(date.year(), date.month(), date.day(), Some(self.as_str()))
    }
}
