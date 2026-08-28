/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use std::collections::HashMap;

use rust_decimal::dec;
use shared_core::ledger::{
    dtos::account_with_balance::AccountWithBalance,
    models::account_category::AccountCategory,
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

pub(crate) fn generate_trial_balance_csv(
    user: &AuthenticatedUser,
    accounts: &[AccountWithBalance],
) -> String {
    let i18n = LocaleContext::new(&user.locale);

    let account_nodes = build_account_nodes(accounts);
    let (total_debit, total_credit) = AccountWithBalance::calculate_totals(accounts);

    let mut csv_content = String::new();
    csv_content.push_str(&format!(
        "{},{},{}\n",
        i18n.t("common-account"),
        i18n.t("common-debit"),
        i18n.t("common-credit"),
    ));

    fn build_csv_rows(
        node: &AccountNode,
        depth: usize,
        content: &mut String,
        i18n: &LocaleContext,
    ) {
        let indent = " ".repeat(depth * 2);
        let (debit_display, credit_display) = match node.account.category {
            AccountCategory::Asset | AccountCategory::Expense => {
                if node.account.balance >= dec!(0.00) {
                    (i18n.format_money(node.account.balance), "".to_string())
                } else {
                    (
                        "".to_string(),
                        i18n.format_money(node.account.balance.abs()),
                    )
                }
            }
            AccountCategory::Liability | AccountCategory::Equity | AccountCategory::Revenue => {
                if node.account.balance <= dec!(0.00) {
                    (
                        "".to_string(),
                        i18n.format_money(node.account.balance.abs()),
                    )
                } else {
                    (i18n.format_money(node.account.balance), "".to_string())
                }
            }
        };
        content.push_str(&format!(
            "\"{}{}\",\"{}\",\"{}\"\n",
            indent, node.account.name, debit_display, credit_display
        ));
        for child in &node.children {
            build_csv_rows(child, depth + 1, content, &i18n);
        }
    }

    for node in &account_nodes {
        build_csv_rows(node, 0, &mut csv_content, &i18n);
    }
    let debit = i18n.format_money(total_debit);
    let credit = i18n.format_money(total_credit);
    csv_content.push_str(&format!(
        "\"{}\",\"{}\",\"{}\"\n",
        i18n.t("trial-balance-export-total"),
        debit,
        credit
    ));
    csv_content
}

pub(crate) fn generate_trial_balance_typst(
    user: &AuthenticatedUser,
    accounts: &[AccountWithBalance],
) -> String {
    let i18n = LocaleContext::new(&user.locale);

    let account_nodes = build_account_nodes(accounts);
    let (total_debit, total_credit) = AccountWithBalance::calculate_totals(accounts);

    let mut typst_content = String::new();

    typst_content.push_str(&*build_table_header(
        &[
            i18n.t("common-account"),
            i18n.t("common-debit"),
            i18n.t("common-credit"),
        ],
        &[false, true, true],
    ));

    fn build_typst_rows(
        node: &AccountNode,
        depth: usize,
        content: &mut String,
        i18n: &LocaleContext,
    ) {
        let indent = "#h(2.0em)".repeat(depth);
        let (debit_display, credit_display) = match node.account.category {
            AccountCategory::Asset | AccountCategory::Expense => {
                if node.account.balance >= dec!(0.00) {
                    (i18n.format_money_typ(node.account.balance), "".to_string())
                } else {
                    (
                        "".to_string(),
                        i18n.format_money_typ(node.account.balance.abs()),
                    )
                }
            }
            AccountCategory::Liability | AccountCategory::Equity | AccountCategory::Revenue => {
                if node.account.balance <= dec!(0.00) {
                    (
                        "".to_string(),
                        i18n.format_money_typ(node.account.balance.abs()),
                    )
                } else {
                    (i18n.format_money_typ(node.account.balance), "".to_string())
                }
            }
        };
        content.push_str(&format!(
            "  [{} {}], align(right)[{}], align(right)[{}],\n",
            indent, node.account.name, debit_display, credit_display
        ));
        for child in &node.children {
            build_typst_rows(child, depth + 1, content, &i18n);
        }
    }

    for node in &account_nodes {
        build_typst_rows(node, 0, &mut typst_content, &i18n);
    }
    typst_content.push_str(&format!(
        "  [*{}*], align(right)[*{}*], align(right)[*{}*],\n",
        i18n.t("common-total"),
        i18n.format_money_typ(total_debit),
        i18n.format_money_typ(total_credit)
    ));
    typst_content.push_str(")\n");

    typst_content
}
