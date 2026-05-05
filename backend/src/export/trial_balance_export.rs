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
use shared_core::models::AccountCategory;
use std::collections::HashMap;
use chrono::NaiveDate;
use uuid::Uuid;
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

pub fn generate_trial_balance_csv(accounts: &[AccountWithBalance]) -> String {
    let account_nodes = build_account_nodes(accounts);
    let (total_debit, total_credit) = AccountWithBalance::calculate_totals(accounts);

    let mut csv_content = String::new();
    csv_content.push_str("Account,Debit,Credit\n");

    fn build_csv_rows(node: &AccountNode, depth: usize, content: &mut String) {
        let indent = " ".repeat(depth * 2);
        let (debit_display, credit_display) = match node.account.category {
            AccountCategory::Asset | AccountCategory::Expense => {
                if node.account.balance >= 0 {
                    (format_currency_typ(&node.account.balance), "".to_string())
                } else {
                    ("".to_string(), format_currency_typ(&node.account.balance.abs()))
                }
            },
            AccountCategory::Liability | AccountCategory::Equity | AccountCategory::Revenue => {
                if node.account.balance <= 0 {
                    ("".to_string(), format_currency_typ(&node.account.balance.abs()))
                } else {
                    (format_currency_typ(&node.account.balance), "".to_string())
                }
            },
        };
        content.push_str(&format!("\"{}{}\",\"{}\",\"{}\"\n", indent, node.account.name, debit_display, credit_display));
        for child in &node.children {
            build_csv_rows(child, depth + 1, content);
        }
    }

    for node in &account_nodes {
        build_csv_rows(node, 0, &mut csv_content);
    }
    let debit = format_currency_typ(&total_debit);
    let credit = format_currency_typ(&total_credit);
    csv_content.push_str(&format!("\"Total\",\"{}\",\"{}\"\n", debit, credit));
    csv_content
}

pub fn generate_trial_balance_typst(accounts: &[AccountWithBalance], report_date: NaiveDate) -> String {
    let account_nodes = build_account_nodes(accounts);
    let (total_debit, total_credit) = AccountWithBalance::calculate_totals(accounts);

    let mut typst_content = String::new();
    typst_content.push_str(&*build_table_header(&["Account", "Debit", "Credit"], &[false, true, true]));

    fn build_typst_rows(node: &AccountNode, depth: usize, content: &mut String) {
        let indent = "#h(2.0em)".repeat(depth);
        let (debit_display, credit_display) = match node.account.category {
            AccountCategory::Asset | AccountCategory::Expense => {
                if node.account.balance >= 0 {
                    (format_currency_typ(&node.account.balance), "".to_string())
                } else {
                    ("".to_string(), format_currency_typ(&node.account.balance.abs()))
                }
            },
            AccountCategory::Liability | AccountCategory::Equity | AccountCategory::Revenue => {
                if node.account.balance <= 0 {
                    ("".to_string(), format_currency_typ(&node.account.balance.abs()))
                } else {
                    (format_currency_typ(&node.account.balance), "".to_string())
                }
            },
        };
        content.push_str(&format!("  [{} {}], align(right)[{}], align(right)[{}],\n", indent, node.account.name, debit_display, credit_display));
        for child in &node.children {
            build_typst_rows(child, depth + 1, content);
        }
    }

    for node in &account_nodes {
        build_typst_rows(node, 0, &mut typst_content);
    }
    typst_content.push_str(&format!("  [*Total*], align(right)[*{}*], align(right)[*{}*],\n", (total_debit as f64) / 100.0, (total_credit as f64) / 100.0));
    typst_content.push_str(")\n");

    let report_qual = format!("As at {}", report_date.format("%d %b %Y").to_string().as_str());
    wrap_report_layout("Alice St", "Trial Balance", &*report_qual, typst_content.as_str() )

}
