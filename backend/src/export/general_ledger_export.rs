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

use shared_core::dtos::general_ledger_line::GeneralLedgerLine;
use chrono::NaiveDate;

pub fn generate_general_ledger_csv(lines: &[GeneralLedgerLine]) -> String {
    let mut wtr = csv::Writer::from_writer(vec![]);
    wtr.write_record(&["Account", "Date", "Description", "Debit", "Credit", "Balance"]).unwrap();

    for line in lines {
        wtr.write_record(&[
            line.account_name.clone(),
            line.date.to_string(),
            line.description.clone().unwrap_or_default(),
            (line.debit as f64 / 100.0).to_string(),
            (line.credit as f64 / 100.0).to_string(),
            (line.balance as f64 / 100.0).to_string(),
        ]).unwrap();
    }
    String::from_utf8(wtr.into_inner().unwrap()).unwrap()
}

pub fn generate_general_ledger_typst(lines: &[GeneralLedgerLine], start_date: NaiveDate, end_date: NaiveDate) -> String {
    let mut typst_string = String::new();
    typst_string.push_str("#import \"../templates/typst/main.typ\": *\n\n");
    typst_string.push_str(&format!("#show: main_doc.with(\n  title: \"General Ledger Detail\",\n  author: \"KelpieBooks\",\n  doc_date: \"{start_date} - {end_date}\",\n)\n\n"));

    let mut grouped = std::collections::BTreeMap::new();
    for line in lines {
        grouped.entry(line.account_name.clone()).or_insert_with(Vec::new).push(line.clone());
    }

    for (account_name, account_lines) in grouped {
        typst_string.push_str(&format!("#block(width: 100%, inset: 0pt, [\n#text(weight: \"bold\", size: 1.2em)[{}]\n])\n\n", account_name));
        typst_string.push_str("#table(\n");
        typst_string.push_str("  columns: (1fr, 2fr, 1fr, 1fr, 1fr),\n");
        typst_string.push_str("  [*Date*], [*Description*], [*Debit*], [*Credit*], [*Balance*],\n");

        for line in account_lines {
            typst_string.push_str(&format!(
                "  \"{}\", \"{}\", \"{}\", \"{}\", \"{}\",\n",
                line.date,
                line.description.clone().unwrap_or_default(),
                if line.debit == 0 { "".to_string() } else { format!("{:.2}", line.debit as f64 / 100.0) },
                if line.credit == 0 { "".to_string() } else { format!("{:.2}", line.credit as f64 / 100.0) },
                format!("{:.2}", line.balance as f64 / 100.0)
            ));
        }
        typst_string.push_str(")\n\n");
    }

    typst_string
}
