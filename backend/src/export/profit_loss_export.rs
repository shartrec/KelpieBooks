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
use shared_core::i18n::{t, t_args};
use chrono::NaiveDate;
use fluent::fluent_args;
use shared_core::dtos::account_with_balance::AccountWithBalance;
use shared_core::models::{account_category::AccountCategory, organization::Organization};
use shared_core::util::format_currency_typ;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct AccountNode {
    pub account: AccountWithBalance,
    pub children: Vec<AccountNode>,
}

fn build_account_nodes(
    accounts: &[AccountWithBalance],
) -> (Vec<AccountNode>, Vec<AccountNode>, i64) {
    let mut revenue_total = 0;
    let mut expense_total = 0;

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
        let children = pc_map
            .get(&acc_id)
            .map(|child_ids| {
                child_ids
                    .iter()
                    .map(|&cid| build_node(cid, acc_map, pc_map))
                    .collect()
            })
            .unwrap_or_default();

        AccountNode {
            account: acc.clone(),
            children,
        }
    }

    let mut rev_nodes = Vec::new();
    let mut exp_nodes = Vec::new();

    for acc in accounts.iter() {
        if acc.parent_id.is_none() {
            let node = build_node(acc.id, &acc_map, &pc_map);
            if acc.category == AccountCategory::Revenue {
                revenue_total += acc.balance;
                rev_nodes.push(node);
            } else if acc.category == AccountCategory::Expense {
                expense_total += acc.balance;
                exp_nodes.push(node);
            }
        }
    }

    (rev_nodes, exp_nodes, (-revenue_total) - expense_total)
}

pub fn generate_profit_loss_csv(accounts: &[AccountWithBalance]) -> String {
    let (revenue_nodes, expense_nodes, net_income) = build_account_nodes(accounts);
    let mut csv_content = String::new();
    csv_content.push_str(
        &format!(
            "{},{}\n",
            t("common-account"),
            t("common-balance"),
        ));

    fn build_csv_rows(node: &AccountNode, depth: usize, content: &mut String) {
        let indent = " ".repeat(depth * 2);
        let display_balance = if node.account.category == AccountCategory::Revenue {
            -node.account.balance
        } else {
            node.account.balance
        };
        content.push_str(&format!(
            "\"{}{}\",\"{}\"\n",
            indent,
            node.account.name,
            format_currency_typ(&display_balance)
        ));
        for child in &node.children {
            build_csv_rows(child, depth + 1, content);
        }
    }

    csv_content.push_str(&t("profit-loss-export-revenue-header"));
    for node in &revenue_nodes {
        build_csv_rows(node, 0, &mut csv_content);
    }
    csv_content.push_str(&t("profit-loss-export-expenses-header"));
    for node in &expense_nodes {
        build_csv_rows(node, 0, &mut csv_content);
    }
    csv_content.push_str(&format!(
        "\"{}\",\"{}\"\n",
        t("profit-loss-net-income"),
        format_currency_typ(&net_income)
    ));
    csv_content
}

pub fn generate_profit_loss_typst(
    accounts: &[AccountWithBalance],
    start_date: &NaiveDate,
    end_date: &NaiveDate,
    org: &Option<Organization>,
) -> String {
    let (revenue_nodes, expense_nodes, net_income) = build_account_nodes(accounts);
    let mut typst_content = String::new();
    typst_content.push_str(&*build_table_header(
        &[t("common-account"), "".to_string(), "".to_string()],
        &vec![false, true, true],
    ));

    fn build_typst_rows(node: &AccountNode, depth: usize, content: &mut String) {
        let indent = "#h(2.0em)".repeat(depth);
        let display_balance = if node.account.category == AccountCategory::Revenue {
            -node.account.balance
        } else {
            node.account.balance
        };
        if depth == 0 {
            content.push_str(&format!(
                "[{} {}], [], align(right)[{}],\n",
                indent,
                node.account.name,
                format_currency_typ(&display_balance)
            ));
        } else {
            content.push_str(&format!(
                "  [{} {}], align(right)[{}], [],\n",
                indent,
                node.account.name,
                format_currency_typ(&display_balance)
            ));
        }
        for child in &node.children {
            build_typst_rows(child, depth + 1, content);
        }
    }

    typst_content.push_str(&format!("[*{}*],[],[],\n", t("profit-loss-revenue-section")));
    for node in &revenue_nodes {
        build_typst_rows(node, 0, &mut typst_content);
    }
    typst_content.push_str(&format!("[*{}*],[],[],\n", t("profit-loss-expenses-section")));
    for node in &expense_nodes {
        build_typst_rows(node, 0, &mut typst_content);
    }
    typst_content.push_str(&format!(
        "[*{}*], [], align(right)[{}]\n",
        t("profit-loss-net-income"),
        format_currency_typ(&net_income)
    ));
    typst_content.push_str(")\n");

    let name = org.as_ref().map(|o| o.name.as_str());
    let report_qual = t_args(
        "general-ledger-export-period",
        &fluent_args!["start_date" => start_date.format("%d %b %Y").to_string(), "end_date" => end_date.format("%d %b %Y").to_string()],
    );
    wrap_report_layout(name, &t("profit-loss-title"), &*report_qual, typst_content.as_str())
}
