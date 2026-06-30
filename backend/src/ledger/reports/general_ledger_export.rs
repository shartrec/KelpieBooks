/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::NaiveDate;
use fluent::fluent_args;
use rust_decimal::dec;
use shared_core::{
    core::models::organization::Organization,
    i18n::format_currency_icu_typ,
    ledger::dtos::general_ledger_line::GeneralLedgerLine,
};

use crate::{
    core::routes::security::AuthenticatedUser,
    util::{
        locale_context::LocaleContext,
        reports::build_table_header,
    },
};

pub(crate) fn generate_general_ledger_csv(
    user: &AuthenticatedUser,
    lines: &[GeneralLedgerLine],
) -> String {
    let i18n = LocaleContext::new(&user.locale);

    let mut wtr = csv::Writer::from_writer(vec![]);
    wtr.write_record(&[
        i18n.t("common-account"),
        i18n.t("common-date"),
        i18n.t("common-description"),
        i18n.t("common-debit"),
        i18n.t("common-credit"),
        i18n.t("common-balance"),
    ])
    .unwrap();

    for line in lines {
        wtr.write_record(&[
            line.account_name.clone(),
            line.date.to_string(),
            line.description.clone().unwrap_or_default(),
            i18n.format_money(line.debit),
            i18n.format_money(line.credit),
            i18n.format_money(line.balance),
        ])
        .unwrap();
    }
    String::from_utf8(wtr.into_inner().unwrap()).unwrap()
}

pub(crate) fn generate_general_ledger_typst(
    user: &AuthenticatedUser,
    lines: &[GeneralLedgerLine],
) -> String {
    let i18n = LocaleContext::new(&user.locale);

    let mut typst_content = String::new();

    typst_content.push_str(&*build_table_header(
        &[
            i18n.t("common-date"),
            i18n.t("common-description"),
            i18n.t("common-debit"),
            i18n.t("common-credit"),
        ],
        &[false, false, true, true],
    ));

    let mut grouped = std::collections::BTreeMap::new();
    for line in lines {
        grouped
            .entry(line.code.clone())
            .or_insert_with(Vec::new)
            .push(line.clone());
    }

    for (_account_name, account_lines) in grouped {
        account_lines.iter().enumerate().for_each(|(index, entry)| {
            if index == 0 {
                typst_content.push_str(&format!(
                    "text(weight: \"bold\", size: 1.2em)[{}], [], [], [],\n",
                    entry.account_name
                ));
                typst_content.push_str(" [],  [], [], [],\n");
            }
            let debit = if entry.debit > dec!(0.00) {
                format_currency_icu_typ(entry.debit, Some(&user.locale))
            } else {
                "".to_string()
            };
            let credit = if entry.credit > dec!(0.00) {
                format_currency_icu_typ(entry.credit, Some(&user.locale))
            } else {
                "".to_string()
            };
            typst_content.push_str(&format!(
                "[{}], [{}], align(right)[{}], align(right)[{}],\n",
                entry.date,
                entry.description.clone().unwrap_or_default(),
                debit,
                credit
            ));
        });
        typst_content.push_str(" [],  [], [], [],\n");
    }

    typst_content.push_str(")\n");

    typst_content
}
