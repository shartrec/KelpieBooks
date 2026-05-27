/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use crate::export::utils::{build_table_header, wrap_report_layout};
use crate::routes::security::AuthenticatedUser;
use chrono::NaiveDate;
use fluent::fluent_args;
use shared_core::dtos::journal_entry_with_balance::JournalEntryWithBalance;
use shared_core::i18n::format_currency_icu_typ;
use shared_core::models::organization::Organization;
use crate::util::locale_context::LocaleContext;

pub fn generate_ledger_csv(user: &AuthenticatedUser, entries: &[JournalEntryWithBalance]) -> String {
    let i18n = LocaleContext::new(&user.locale);

    let mut csv_content = String::new();
    csv_content.push_str(
        &format!(
            "{},{},{},{},{}\n",
            i18n.t("common-date"),
            i18n.t("common-description"),
            i18n.t("common-debit"),
            i18n.t("common-credit"),
            i18n.t("common-balance"),
        ));

    for entry in entries.iter() {
        let debit = if entry.debit > 0 {
            format_currency_icu_typ(entry.debit, Some(&user.locale))
        } else {
            "".to_string()
        };
        let credit = if entry.credit > 0 {
            format_currency_icu_typ(entry.credit, Some(&user.locale))
        } else {
            "".to_string()
        };
        let balance = format_currency_icu_typ(entry.debit - entry.credit, Some(&user.locale));
        csv_content.push_str(&format!(
            "{},\"{}\",{},{},{}\n",
            entry.date,
            entry.description.clone().unwrap_or_default(),
            debit,
            credit,
            balance
        ));
    }
    csv_content
}

pub fn generate_ledger_typst(
    user: &AuthenticatedUser,
    entries: &[JournalEntryWithBalance],
    account_name: &str,
    start_date: &NaiveDate,
    end_date: &NaiveDate,
    org: &Option<Organization>,
) -> String {
    let i18n = LocaleContext::new(&user.locale);

    let mut typst_content = String::new();

    typst_content.push_str(&*build_table_header(
        &[i18n.t("common-date"),
            i18n.t("common-description"),
            i18n.t("common-debit"),
            i18n.t("common-credit"),
            i18n.t("common-balance")],
        &[false, false, true, true, true],
    ));

    for entry in entries.iter() {
        let debit = if entry.debit > 0 {
            format_currency_icu_typ(entry.debit, Some(&user.locale))
        } else {
            "".to_string()
        };
        let credit = if entry.credit > 0 {
            format_currency_icu_typ(entry.credit, Some(&user.locale))
        } else {
            "".to_string()
        };
        let balance = format_currency_icu_typ(entry.debit - entry.credit, Some(&user.locale));
        typst_content.push_str(&format!(
            "[{}], [{}], align(right)[{}], align(right)[{}], align(right)[{}],\n",
            entry.date,
            entry.description.clone().unwrap_or_default(),
            debit,
            credit,
            balance
        ));
    }
    typst_content.push_str(")\n");
    let name = org.as_ref().map(|o| o.name.as_str());

    let report_qual = i18n.t_args(
        "account-ledger-export-report-qualifier",
        &fluent_args!["account_name" => account_name, "start_date" => start_date.format("%d %b %Y").to_string(), "end_date" => end_date.format("%d %b %Y").to_string()],
    );
    wrap_report_layout(
        name,
        &i18n.t("account-ledger-export-title"),
        &*report_qual,
        typst_content.as_str(),
    )
}
