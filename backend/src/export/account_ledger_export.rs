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
use crate::export::utils::{build_table_header, wrap_report_layout};
use chrono::NaiveDate;
use fluent::fluent_args;
use shared_core::dtos::journal_entry_with_balance::JournalEntryWithBalance;
use shared_core::i18n::{t, t_args};
use shared_core::models::organization::Organization;
use shared_core::util::format_currency_typ;

pub fn generate_ledger_csv(entries: &[JournalEntryWithBalance]) -> String {
    let mut csv_content = String::new();
    csv_content.push_str(
        &format!(
            "{},{},{},{},{}\n",
            t("common-date"),
            t("common-description"),
            t("common-debit"),
            t("common-credit"),
            t("common-balance"),
        ));

    for entry in entries.iter() {
        let debit = if entry.debit > 0 {
            format_currency_typ(&entry.debit)
        } else {
            "".to_string()
        };
        let credit = if entry.credit > 0 {
            format_currency_typ(&entry.credit)
        } else {
            "".to_string()
        };
        let balance = format_currency_typ(&(entry.debit - entry.credit));
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
    entries: &[JournalEntryWithBalance],
    account_name: &str,
    start_date: &NaiveDate,
    end_date: &NaiveDate,
    org: &Option<Organization>,
) -> String {
    let mut typst_content = String::new();

    typst_content.push_str(&*build_table_header(
        &[t("common-date"), t("common-description"), t("common-debit"), t("common-credit"), t("common-balance")],
        &[false, false, true, true, true],
    ));

    for entry in entries.iter() {
        let debit = if entry.debit > 0 {
            format_currency_typ(&entry.debit)
        } else {
            "".to_string()
        };
        let credit = if entry.credit > 0 {
            format_currency_typ(&entry.credit)
        } else {
            "".to_string()
        };
        let balance = format_currency_typ(&(entry.debit - entry.credit));
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

    let report_qual = t_args(
        "account-ledger-export-report-qualifier",
        &fluent_args!["account_name" => account_name, "start_date" => start_date.format("%d %b %Y").to_string(), "end_date" => end_date.format("%d %b %Y").to_string()],
    );
    wrap_report_layout(
        name,
        &t("account-ledger-export-title"),
        &*report_qual,
        typst_content.as_str(),
    )
}
