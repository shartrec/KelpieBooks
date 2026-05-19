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

use crate::export::utils::{build_table_header, wrap_report_layout};
use chrono::NaiveDate;
use shared_core::dtos::general_ledger_line::GeneralLedgerLine;
use shared_core::models::organization::Organization;
use shared_core::util::format_currency_typ;

pub fn generate_general_ledger_csv(lines: &[GeneralLedgerLine]) -> String {
    let mut wtr = csv::Writer::from_writer(vec![]);
    wtr.write_record(&[
        "Account",
        "Date",
        "Description",
        "Debit",
        "Credit",
        "Balance",
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
    lines: &[GeneralLedgerLine],
    start_date: &NaiveDate,
    end_date: &NaiveDate,
    org: &Option<Organization>,
) -> String {
    let mut typst_content = String::new();
    typst_content.push_str(&*build_table_header(
        &["Date", "Description", "Debit", "Credit"],
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
                format_currency_typ(&entry.debit)
            } else {
                "".to_string()
            };
            let credit = if entry.credit > 0 {
                format_currency_typ(&entry.credit)
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
    let report_qual = format!(
        "Period {} - {}",
        start_date.format("%d %b %Y").to_string().as_str(),
        end_date.format("%d %b %Y").to_string().as_str()
    );
    wrap_report_layout(
        name,
        "Journal Entries",
        &*report_qual,
        typst_content.as_str(),
    )
}
