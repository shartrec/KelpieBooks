/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::export::utils::{build_table_header, format_currency_typ, wrap_report_layout};
use shared_core::i18n::{t, t_args};
use chrono::NaiveDate;
use fluent::fluent_args;
use shared_core::dtos::account_with_balance::AccountWithBalance;
use shared_core::models::organization::Organization;
use shared_core::reports::balance_sheet::BalanceSheet;
use std::collections::HashMap;
use uuid::Uuid;

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
    csv_content.push_str(
        &format!(
            "{},{}\n",
            t("common-account"),
            t("common-balance"),
        ));

    fn build_csv_rows(node: &AccountNode, depth: usize, content: &mut String) {
        let indent = " ".repeat(depth * 2);
        content.push_str(&format!(
            "\"{}{}\",\"{}\"\n",
            indent,
            node.account.name,
            (node.account.balance as f64) / 100.0
        ));
        for child in &node.children {
            build_csv_rows(child, depth + 1, content);
        }
    }

    csv_content.push_str(&t("balance-sheet-export-assets-header"));
    for node in &asset_nodes {
        build_csv_rows(node, 0, &mut csv_content);
    }
    csv_content.push_str(&format!(
        "\"{}\",\"{}\"\n",
        t("balance-sheet-export-total-assets"),
        (balance_sheet.total_assets as f64) / 100.0
    ));

    csv_content.push_str(&t("balance-sheet-export-liabilities-header"));
    for node in &liability_nodes {
        build_csv_rows(node, 0, &mut csv_content);
    }
    csv_content.push_str(&format!(
        "\"{}\",\"{}\"\n",
        t("balance-sheet-export-total-liabilities"),
        (balance_sheet.total_liabilities as f64) / 100.0
    ));

    csv_content.push_str(&t("balance-sheet-export-equity-header"));
    for node in &equity_nodes {
        build_csv_rows(node, 0, &mut csv_content);
    }
    csv_content.push_str(&format!(
        "\"{}\",\"{}\"\n",
        t("balance-sheet-export-current-year-earnings"),
        (balance_sheet.net_income as f64) / 100.0
    ));
    csv_content.push_str(&format!(
        "\"{}\",\"{}\"\n",
        t("balance-sheet-export-total-equity"),
        (balance_sheet.total_equity as f64) / 100.0
    ));
    csv_content.push_str(&format!(
        "\"{}\",\"{}\"\n",
        t("balance-sheet-export-total-liabilities-equity"),
        ((balance_sheet.total_liabilities + balance_sheet.total_equity) as f64) / 100.0
    ));
    csv_content
}

pub fn generate_balance_sheet_typst(
    balance_sheet: &BalanceSheet,
    report_date: &NaiveDate,
    org: &Option<Organization>,
) -> String {
    let asset_nodes = build_account_nodes(&balance_sheet.assets);
    let liability_nodes = build_account_nodes(&balance_sheet.liabilities);
    let equity_nodes = build_account_nodes(&balance_sheet.equity);

    let mut typst_content = String::new();

    typst_content.push_str(&*build_table_header(
        &[t("common-account"), "".to_string(), "".to_string()],
        &[false, true, true],
    ));

    fn build_typst_rows(node: &AccountNode, depth: usize, content: &mut String) {
        let indent = "#h(2.0em)".repeat(depth);
        content.push_str(&format!(
            "  [{} {}], align(right)[{}],[],\n",
            indent,
            node.account.name,
            format_currency_typ(node.account.balance)
        ));
        for child in &node.children {
            build_typst_rows(child, depth + 1, content);
        }
    }

    typst_content.push_str(&format!("[*{}*],[],[],\n", t("balance-sheet-assets-section")));
    for node in &asset_nodes {
        build_typst_rows(node, 0, &mut typst_content);
    }
    typst_content.push_str(&format!(
        "align(right)[*{}*],[],align(right)[*{}*],\n",
        t("balance-sheet-total-assets"),
        format_currency_typ(balance_sheet.total_assets)
    ));

    typst_content.push_str(&format!("[*{}*],[],[],\n", t("balance-sheet-liabilities-section")));
    for node in &liability_nodes {
        build_typst_rows(node, 0, &mut typst_content);
    }
    typst_content.push_str(&format!(
        "align(right)[*{}*],[],align(right)[*{}*],\n",
        t("balance-sheet-total-liabilities"),
        format_currency_typ(balance_sheet.total_liabilities)
    ));

    typst_content.push_str(&format!("[*{}*],[],[],\n", t("balance-sheet-equity-section")));
    for node in &equity_nodes {
        build_typst_rows(node, 0, &mut typst_content);
    }
    typst_content.push_str(&format!(
        "[{}],align(right)[{}],[],",
        t("balance-sheet-current-year-earnings"),
        format_currency_typ(balance_sheet.net_income)
    ));

    typst_content.push_str(&format!(
        "align(right)[*{}*],[],align(right)[*{}*],\n",
        t("balance-sheet-total-equity"),
        format_currency_typ(balance_sheet.total_equity)
    ));
    typst_content.push_str(&format!(
        "align(right)[*{}*],[],align(right)[*{}*],\n",
        t("balance-sheet-total-liabilities-equity"),
        format_currency_typ(balance_sheet.total_liabilities + balance_sheet.total_equity)
    ));

    typst_content.push_str(")\n");

    let name = org.as_ref().map(|o| o.name.as_str());
    let report_qual = t_args(
        "balance-sheet-export-as-at",
        &fluent_args!["date" => report_date.format("%d %b %Y").to_string()],
    );
    wrap_report_layout(name, &t("balance-sheet-title"), &*report_qual, typst_content.as_str())
}
