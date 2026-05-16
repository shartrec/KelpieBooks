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

use shared_core::dtos::account_with_balance::AccountWithBalance;
use shared_core::reports::balance_sheet::BalanceSheet;
use std::collections::HashMap;
use chrono::NaiveDate;
use uuid::Uuid;
use shared_core::models::Organization;
use shared_core::util::format_currency_typ;
use crate::export::utils::{build_table_header, wrap_report_layout};

#[derive(Clone, Debug)]
pub struct AccountNode {
    pub account: AccountWithBalance,
    pub children: Vec<AccountNode>,
}

fn build_account_nodes(accounts: &[AccountWithBalance]) -> Vec<AccountNode> {
    let mut acc_map: HashMap<Uuid, AccountWithBalance> = HashMap::new();
    let mut pc_map: HashMap<Uuid, Vec<Uuid>> = HashMap::new();

    for acc in accounts.iter() {
        acc_map.insert(acc.id, acc.clone());
        if let Some(parent_id) = acc.parent_id {
            pc_map.entry(parent_id).or_default().push(acc.id);
        }
    }

    fn build_node(
        acc_id: Uuid,
        acc_map: &HashMap<Uuid, AccountWithBalance>,
        pc_map: &HashMap<Uuid, Vec<Uuid>>,
    ) -> AccountNode {
        let acc = acc_map.get(&acc_id).unwrap();
        let children = pc_map.get(&acc_id)
            .map(|child_ids| {
                child_ids.iter()
                    .map(|&cid| build_node(cid, acc_map, pc_map))
                    .collect()
            })
            .unwrap_or_default();

        AccountNode {
            account: acc.clone(),
            children,
        }
    }

    let mut root_nodes = Vec::new();
    for acc in accounts.iter() {
        if acc.parent_id.is_none() {
            root_nodes.push(build_node(acc.id, &acc_map, &pc_map));
        }
    }
    root_nodes
}

pub fn generate_balance_sheet_csv(balance_sheet: &BalanceSheet) -> String {
    let asset_nodes = build_account_nodes(&balance_sheet.assets);
    let liability_nodes = build_account_nodes(&balance_sheet.liabilities);
    let equity_nodes = build_account_nodes(&balance_sheet.equity);
    let mut csv_content = String::new();
    csv_content.push_str("Account,Balance\n");

    fn build_csv_rows(node: &AccountNode, depth: usize, content: &mut String) {
        let indent = " ".repeat(depth * 2);
        content.push_str(&format!("\"{}{}\",\"{}\"\n", indent, node.account.name, (node.account.balance as f64) / 100.0));
        for child in &node.children {
            build_csv_rows(child, depth + 1, content);
        }
    }

    csv_content.push_str("Assets,\n");
    for node in &asset_nodes {
        build_csv_rows(node, 0, &mut csv_content);
    }
    csv_content.push_str(&format!("\"Total Assets\",\"{}\"\n", (balance_sheet.total_assets as f64) / 100.0));

    csv_content.push_str("Liabilities,\n");
    for node in &liability_nodes {
        build_csv_rows(node, 0, &mut csv_content);
    }
    csv_content.push_str(&format!("\"Total Liabilities\",\"{}\"\n", (balance_sheet.total_liabilities as f64) / 100.0));

    csv_content.push_str("Equity,\n");
    for node in &equity_nodes {
        build_csv_rows(node, 0, &mut csv_content);
    }
    csv_content.push_str(&format!("\"Current Year Earnings\",\"{}\"\n", (balance_sheet.net_income as f64) / 100.0));
    csv_content.push_str(&format!("\"Total Equity\",\"{}\"\n", (balance_sheet.total_equity as f64) / 100.0));
    csv_content.push_str(&format!("\"Total Liabilities & Equity\",\"{}\"\n", ((balance_sheet.total_liabilities + balance_sheet.total_equity) as f64) / 100.0));
    csv_content
}

pub fn generate_balance_sheet_typst(balance_sheet: &BalanceSheet, report_date: &NaiveDate, org: &Option<Organization>) -> String {
    let asset_nodes = build_account_nodes(&balance_sheet.assets);
    let liability_nodes = build_account_nodes(&balance_sheet.liabilities);
    let equity_nodes = build_account_nodes(&balance_sheet.equity);

    let mut typst_content = String::new();

    typst_content.push_str(&*build_table_header(&["Account", "", ""], &[false, true, true]));

    fn build_typst_rows(node: &AccountNode, depth: usize, content: &mut String) {
        let indent = "#h(2.0em)".repeat(depth);
        content.push_str(&format!("  [{} {}], align(right)[{}],[],\n", indent, node.account.name, format_currency_typ(&node.account.balance)));
        for child in &node.children {
            build_typst_rows(child, depth + 1, content);
        }
    }

    typst_content.push_str("[*Assets*],[],[],\n");
    for node in &asset_nodes {
        build_typst_rows(node, 0, &mut typst_content);
    }
    typst_content.push_str(&format!("align(right)[*Total Assets:*],[],align(right)[*{}*],\n", format_currency_typ(&balance_sheet.total_assets)));

    typst_content.push_str("[*Liabilities*],[],[],\n");
    for node in &liability_nodes {
        build_typst_rows(node, 0, &mut typst_content);
    }
    typst_content.push_str(&format!("align(right)[*Total Liabilities:*],[],align(right)[*{}*],\n", format_currency_typ(&balance_sheet.total_liabilities)));

    typst_content.push_str("[*Equity*],[],[],\n");
    for node in &equity_nodes {
        build_typst_rows(node, 0, &mut typst_content);
    }
    typst_content.push_str(&format!("[Current Year Earnings],align(right)[{}],[],", format_currency_typ(&balance_sheet.net_income)));

    typst_content.push_str(&format!("align(right)[*Total Equity*],[],align(right)[*{}*],\n", format_currency_typ(&balance_sheet.total_equity)));
    typst_content.push_str(&format!("align(right)[*Total Liabilities & Equity*],[],align(right)[*{}*],\n", format_currency_typ(&&(balance_sheet.total_liabilities + balance_sheet.total_equity))));

    typst_content.push_str(")\n");

    let name = org.as_ref().map(|o| o.name.as_str());
    let report_qual = format!("As at {}", report_date.format("%d %b %Y").to_string().as_str());
    wrap_report_layout(name, "Balance Sheet", &*report_qual, typst_content.as_str() )

}
