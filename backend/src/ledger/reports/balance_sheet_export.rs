/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use std::collections::HashMap;

use shared_core::ledger::dtos::{
    account_with_balance::AccountWithBalance,
    balance_sheet::BalanceSheet,
};
use shared_core::AccountId;
use crate::{
    core::routes::security::AuthenticatedUser,
    util::{
        locale_context::LocaleContext,
        reports::build_table_header,
    },
};

#[derive(Clone, Debug)]
pub(crate) struct AccountNode {
    pub(crate) account: AccountWithBalance,
    pub(crate) children: Vec<AccountNode>,
}

fn build_account_nodes(accounts: &[AccountWithBalance]) -> Vec<AccountNode> {
    let mut acc_map: HashMap<AccountId, AccountWithBalance> = HashMap::new();
    let mut pc_map: HashMap<AccountId, Vec<AccountId>> = HashMap::new();

    for acc in accounts.iter() {
        acc_map.insert(acc.id, acc.clone());
        if let Some(parent_id) = acc.parent_id {
            pc_map.entry(parent_id).or_default().push(acc.id);
        }
    }

    fn build_node(
        acc_id: AccountId,
        acc_map: &HashMap<AccountId, AccountWithBalance>,
        pc_map: &HashMap<AccountId, Vec<AccountId>>,
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

pub(crate) fn generate_balance_sheet_csv(
    user: &AuthenticatedUser,
    balance_sheet: &BalanceSheet,
) -> String {
    let i18n = LocaleContext::new(&user.locale);

    let asset_nodes = build_account_nodes(&balance_sheet.assets);
    let liability_nodes = build_account_nodes(&balance_sheet.liabilities);
    let equity_nodes = build_account_nodes(&balance_sheet.equity);
    let mut csv_content = String::new();
    csv_content.push_str(&format!(
        "{},{}\n",
        i18n.t("common-account"),
        i18n.t("common-balance"),
    ));

    fn build_csv_rows(
        node: &AccountNode,
        depth: usize,
        content: &mut String,
        i18n: &LocaleContext,
    ) {
        let indent = " ".repeat(depth * 2);
        content.push_str(&format!(
            "\"{}{}\",\"{}\"\n",
            indent,
            node.account.name,
            i18n.format_money(node.account.balance)
        ));
        for child in &node.children {
            build_csv_rows(child, depth + 1, content, &i18n);
        }
    }

    csv_content.push_str(&i18n.t("balance-sheet-export-assets-header"));
    for node in &asset_nodes {
        build_csv_rows(node, 0, &mut csv_content, &i18n);
    }
    csv_content.push_str(&format!(
        "\"{}\",\"{}\"\n",
        i18n.t("balance-sheet-export-total-assets"),
        i18n.format_money(balance_sheet.total_assets)
    ));

    csv_content.push_str(&i18n.t("balance-sheet-export-liabilities-header"));
    for node in &liability_nodes {
        build_csv_rows(node, 0, &mut csv_content, &i18n);
    }
    csv_content.push_str(&format!(
        "\"{}\",\"{}\"\n",
        i18n.t("balance-sheet-export-total-liabilities"),
        i18n.format_money(balance_sheet.total_liabilities)
    ));

    csv_content.push_str(&i18n.t("balance-sheet-export-equity-header"));
    for node in &equity_nodes {
        build_csv_rows(node, 0, &mut csv_content, &i18n);
    }
    csv_content.push_str(&format!(
        "\"{}\",\"{}\"\n",
        i18n.t("balance-sheet-export-current-year-earnings"),
        i18n.format_money(balance_sheet.net_income)
    ));
    csv_content.push_str(&format!(
        "\"{}\",\"{}\"\n",
        i18n.t("balance-sheet-export-total-equity"),
        i18n.format_money(balance_sheet.total_equity)
    ));
    csv_content.push_str(&format!(
        "\"{}\",\"{}\"\n",
        i18n.t("balance-sheet-export-total-liabilities-equity"),
        i18n.format_money(balance_sheet.total_liabilities + balance_sheet.total_equity)
    ));
    csv_content
}

pub(crate) fn generate_balance_sheet_typst(
    user: &AuthenticatedUser,
    balance_sheet: &BalanceSheet,
) -> String {
    let i18n = LocaleContext::new(&user.locale);

    let asset_nodes = build_account_nodes(&balance_sheet.assets);
    let liability_nodes = build_account_nodes(&balance_sheet.liabilities);
    let equity_nodes = build_account_nodes(&balance_sheet.equity);

    let mut typst_content = String::new();

    typst_content.push_str(&*build_table_header(
        &[i18n.t("common-account"), "".to_string(), "".to_string()],
        &[false, true, true],
    ));

    fn build_typst_rows(
        node: &AccountNode,
        depth: usize,
        content: &mut String,
        i18n: &LocaleContext,
    ) {
        let indent = "#h(2.0em)".repeat(depth);

        content.push_str(&format!(
            "  [{} {}], align(right)[{}],[],\n",
            indent,
            node.account.name,
            i18n.format_money_typ(node.account.balance)
        ));
        for child in &node.children {
            build_typst_rows(child, depth + 1, content, &i18n);
        }
    }

    typst_content.push_str(&format!(
        "[*{}*],[],[],\n",
        i18n.t("balance-sheet-assets-section")
    ));
    for node in &asset_nodes {
        build_typst_rows(node, 0, &mut typst_content, &i18n);
    }
    typst_content.push_str(&format!(
        "align(right)[*{}*],[],align(right)[*{}*],\n",
        i18n.t("balance-sheet-total-assets"),
        i18n.format_money_typ(balance_sheet.total_assets)
    ));

    typst_content.push_str(&format!(
        "[*{}*],[],[],\n",
        i18n.t("balance-sheet-liabilities-section")
    ));
    for node in &liability_nodes {
        build_typst_rows(node, 0, &mut typst_content, &i18n);
    }
    typst_content.push_str(&format!(
        "align(right)[*{}*],[],align(right)[*{}*],\n",
        i18n.t("balance-sheet-total-liabilities"),
        i18n.format_money_typ(balance_sheet.total_liabilities)
    ));

    typst_content.push_str(&format!(
        "[*{}*],[],[],\n",
        i18n.t("balance-sheet-equity-section")
    ));
    for node in &equity_nodes {
        build_typst_rows(node, 0, &mut typst_content, &i18n);
    }
    typst_content.push_str(&format!(
        "[{}],align(right)[{}],[],",
        i18n.t("balance-sheet-current-year-earnings"),
        i18n.format_money_typ(balance_sheet.net_income)
    ));

    typst_content.push_str(&format!(
        "align(right)[*{}*],[],align(right)[*{}*],\n",
        i18n.t("balance-sheet-total-equity"),
        i18n.format_money_typ(balance_sheet.total_equity)
    ));
    typst_content.push_str(&format!(
        "align(right)[*{}*],[],align(right)[*{}*],\n",
        i18n.t("balance-sheet-total-liabilities-equity"),
        i18n.format_money_typ(balance_sheet.total_liabilities + balance_sheet.total_equity)
    ));

    typst_content.push_str(")\n");

    typst_content
}
