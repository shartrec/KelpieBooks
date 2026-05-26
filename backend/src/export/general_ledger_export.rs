/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::export::utils::{build_table_header, wrap_report_layout};
use shared_core::i18n::{t, t_args};
use chrono::NaiveDate;
use fluent::fluent_args;
use shared_core::dtos::general_ledger_line::GeneralLedgerLine;
use shared_core::models::organization::Organization;
use shared_core::util::format_currency_icu_typ;
use crate::routes::security::AuthenticatedUser;

pub fn generate_general_ledger_csv(lines: &[GeneralLedgerLine]) -> String {
    let mut wtr = csv::Writer::from_writer(vec![]);
    wtr.write_record(&[
        t("common-account"),
        t("common-date"),
        t("common-description"),
        t("common-debit"),
        t("common-credit"),
        t("common-balance"),
    ])
    .unwrap();

    for line in lines {
        wtr.write_record(&[
            line.account_name.clone(),
            line.date.to_string(),
            line.description.clone().unwrap_or_default(),
            (line.debit as f64 / 100.0).to_string(),
            (line.credit as f64 / 100.0).to_string(),
            (line.balance as f64 / 100.0).to_string(),
        ])
        .unwrap();
    }
    String::from_utf8(wtr.into_inner().unwrap()).unwrap()
}

pub fn generate_general_ledger_typst(
    user: &AuthenticatedUser,
    lines: &[GeneralLedgerLine],
    start_date: &NaiveDate,
    end_date: &NaiveDate,
    org: &Option<Organization>,
) -> String {
    let mut typst_content = String::new();
    typst_content.push_str(&*build_table_header(
        &[t("common-date"), t("common-description"), t("common-debit"), t("common-credit")],
        &[false, false, true, true],
    ));

    let mut grouped = std::collections::BTreeMap::new();
    for line in lines {
        grouped
            .entry(line.account_name.clone())
            .or_insert_with(Vec::new)
            .push(line.clone());
    }

    for (account_name, account_lines) in grouped {
        typst_content.push_str(&format!(
            "text(weight: \"bold\", size: 1.2em)[{}], [], [], [],\n",
            account_name
        ));
        typst_content.push_str(" [],  [], [], [],\n");

        for entry in account_lines {
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
            typst_content.push_str(&format!(
                "[{}], [{}], align(right)[{}], align(right)[{}],\n",
                entry.date,
                entry.description.clone().unwrap_or_default(),
                debit,
                credit
            ));
        }
        typst_content.push_str(" [],  [], [], [],\n");
    }

    typst_content.push_str(")\n");

    let name = org.as_ref().map(|o| o.name.as_str());
    let report_qual = t_args(
        "general-ledger-export-period",
        &fluent_args!["start_date" => start_date.format("%d %b %Y").to_string(), "end_date" => end_date.format("%d %b %Y").to_string()],
    );
    wrap_report_layout(
        name,
        &t("account-ledger-export-title"),
        &*report_qual,
        typst_content.as_str(),
    )
}
