/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::export::utils::{build_table_header, wrap_report_layout};
use crate::routes::security::AuthenticatedUser;
use crate::util::locale_context::LocaleContext;
use chrono::NaiveDate;
use fluent::fluent_args;
use shared_core::dtos::account_with_balance::AccountWithBalance;
use shared_core::models::{account_category::AccountCategory, organization::Organization};
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

pub fn generate_profit_loss_csv(user: &AuthenticatedUser, accounts: &[AccountWithBalance]) -> String {
    let i18n = LocaleContext::new(&user.locale);

    let (revenue_nodes, expense_nodes, net_income) = build_account_nodes(accounts);
    let mut csv_content = String::new();
    csv_content.push_str(
        &format!(
            "{},{}\n",
            i18n.t("common-account"),
            i18n.t("common-balance"),
        ));

    fn build_csv_rows(node: &AccountNode, depth: usize, content: &mut String, i18n: &LocaleContext) {
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
            i18n.format_money(display_balance)
        ));
        for child in &node.children {
            build_csv_rows(child, depth + 1, content, &i18n
            );
        }
    }

    csv_content.push_str(&i18n.t("profit-loss-export-revenue-header"));
    for node in &revenue_nodes {
        build_csv_rows(node, 0, &mut csv_content, &i18n);
    }
    csv_content.push_str(&i18n.t("profit-loss-export-expenses-header"));
    for node in &expense_nodes {
        build_csv_rows(node, 0, &mut csv_content, &i18n);
    }
    csv_content.push_str(&format!(
        "\"{}\",\"{}\"\n",
        i18n.t("profit-loss-net-income"),
        i18n.format_money(net_income)
    ));
    csv_content
}

pub fn generate_profit_loss_typst(
    user: &AuthenticatedUser,
    accounts: &[AccountWithBalance],
    start_date: &NaiveDate,
    end_date: &NaiveDate,
    org: &Option<Organization>,
) -> String {
    let i18n = LocaleContext::new(&user.locale);

    let (revenue_nodes, expense_nodes, net_income) = build_account_nodes(accounts);
    let mut typst_content = String::new();
    typst_content.push_str(&*build_table_header(
        &[i18n.t("common-account"), "".to_string(), "".to_string()],
        &vec![false, true, true],
    ));

    fn build_typst_rows(node: &AccountNode, depth: usize, content: &mut String, i18n: &LocaleContext) {
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
                i18n.format_money_typ(display_balance)
            ));
        } else {
            content.push_str(&format!(
                "  [{} {}], align(right)[{}], [],\n",
                indent,
                node.account.name,
                i18n.format_money_typ(display_balance)
            ));
        }
        for child in &node.children {
            build_typst_rows(child, depth + 1, content, &i18n);
        }
    }

    typst_content.push_str(&format!("[*{}*],[],[],\n", i18n.t("profit-loss-revenue-section")));
    for node in &revenue_nodes {
        build_typst_rows(node, 0, &mut typst_content, &i18n);
    }
    typst_content.push_str(&format!("[*{}*],[],[],\n", i18n.t("profit-loss-expenses-section")));
    for node in &expense_nodes {
        build_typst_rows(node, 0, &mut typst_content, &i18n);
    }
    typst_content.push_str(&format!(
        "[*{}*], [], align(right)[{}]\n",
        i18n.t("profit-loss-net-income"),
        i18n.format_money_typ(net_income)
    ));
    typst_content.push_str(")\n");

    let name = org.as_ref().map(|o| o.name.as_str());
    let start_date_str = i18n.format_date(*start_date);
    let end_date_str = i18n.format_date(*end_date);
    let report_qual = i18n.t_args(
        "general-ledger-export-period",
        &fluent_args!["start_date" => start_date_str, "end_date" => end_date_str],
    );
    wrap_report_layout(name, &i18n.t("profit-loss-title"), &*report_qual, typst_content.as_str())
}
