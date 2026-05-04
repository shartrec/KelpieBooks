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

use shared_core::dtos::journal_entry_with_balance::JournalEntryWithBalance;

fn format_currency(amount: &i64) -> String {
    format!("{:.2}", (*amount as f64) / 100.0)
}

pub fn generate_ledger_csv(entries: &[JournalEntryWithBalance]) -> String {
    let mut csv_content = String::new();
    csv_content.push_str("Date,Description,Debit,Credit,Balance\n");
    for entry in entries.iter() {
        let debit = if entry.debit > 0 { format_currency(&entry.debit) } else { "".to_string() };
        let credit = if entry.credit > 0 { format_currency(&entry.credit) } else { "".to_string() };
        let balance = format_currency(&(entry.debit - entry.credit));
        csv_content.push_str(&format!("{},\"{}\",{},{},{}\n", entry.date, entry.description.clone().unwrap_or_default(), debit, credit, balance));
        csv_content.push_str(&format!("{},\"{}\",{},{},{}\n", entry.date, entry.description.clone().unwrap_or_default(), debit, credit, balance));
    }
    csv_content
}

pub fn generate_ledger_typst(entries: &[JournalEntryWithBalance]) -> String {
    let mut typst_content = String::new();
    typst_content.push_str("#set text(size: 10pt)\n");
    typst_content.push_str("#set page(margin: (top: 2cm, bottom: 2cm, left: 1.5cm, right: 1.5cm))\n\n");
    typst_content.push_str("= Account Ledger\n\n");
    typst_content.push_str("#table(\n");
    typst_content.push_str("  columns: (auto, 1fr, 1fr, 1fr, 1fr),\n");
    typst_content.push_str("  [*Date*], [*Description*], [*Debit*], [*Credit*], [*Balance*],\n");
    for entry in entries.iter() {
        let debit = if entry.debit > 0 { format_currency(&entry.debit) } else { "".to_string() };
        let credit = if entry.credit > 0 { format_currency(&entry.credit) } else { "".to_string() };
        let balance = format_currency(&(entry.debit - entry.credit));
        typst_content.push_str(&format!("  \"{}\", \"{}\", align(right)[{}], align(right)[{}], align(right)[{}],\n", entry.date, entry.description.clone().unwrap_or_default(), debit, credit, balance));
    }
    typst_content.push_str(")\n");
    typst_content
}
