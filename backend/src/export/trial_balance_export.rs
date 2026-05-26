/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::export::utils::{build_table_header, wrap_report_layout};
use shared_core::i18n::{t, t_args};
use chrono::NaiveDate;
use fluent::fluent_args;
use shared_core::dtos::account_with_balance::AccountWithBalance;
use shared_core::models::{account_category::AccountCategory, organization::Organization};
use std::collections::HashMap;
use uuid::Uuid;
use shared_core::util::format_currency_icu_typ;
use crate::routes::security::AuthenticatedUser;

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

pub fn generate_trial_balance_csv(user: &AuthenticatedUser, accounts: &[AccountWithBalance]) -> String {
    let account_nodes = build_account_nodes(accounts);
    let (total_debit, total_credit) = AccountWithBalance::calculate_totals(accounts);

    let mut csv_content = String::new();
    csv_content.push_str("Account,Debit,Credit\n");
    csv_content.push_str(
        &format!(
            "{},{},{}\n",
            t("common-account"),
            t("common-debit"),
            t("common-credit"),
        ));

    fn build_csv_rows(node: &AccountNode, depth: usize, content: &mut String, locale: Option<&str> ) {
        let indent = " ".repeat(depth * 2);
        let (debit_display, credit_display) = match node.account.category {
            AccountCategory::Asset | AccountCategory::Expense => {
                if node.account.balance >= 0 {
                    (format_currency_icu_typ(node.account.balance, locale), "".to_string())
                } else {
                    (
                        "".to_string(),
                        format_currency_icu_typ(node.account.balance.abs(), locale),
                    )
                }
            }
            AccountCategory::Liability | AccountCategory::Equity | AccountCategory::Revenue => {
                if node.account.balance <= 0 {
                    (
                        "".to_string(),
                        format_currency_icu_typ(node.account.balance.abs(), locale),
                    )
                } else {
                    (format_currency_icu_typ(node.account.balance, locale), "".to_string())
                }
            }
        };
        content.push_str(&format!(
            "\"{}{}\",\"{}\",\"{}\"\n",
            indent, node.account.name, debit_display, credit_display
        ));
        for child in &node.children {
            build_csv_rows(child, depth + 1, content, locale);
        }
    }

    for node in &account_nodes {
        build_csv_rows(node, 0, &mut csv_content, Some(&user.locale));
    }
    let debit = format_currency_icu_typ(total_debit, Some(&user.locale));
    let credit = format_currency_icu_typ(total_credit, Some(&user.locale));
    csv_content.push_str(&format!("\"{}\",\"{}\",\"{}\"\n", t("trial-balance-export-total"), debit, credit));
    csv_content
}

pub fn generate_trial_balance_typst(
    user: &AuthenticatedUser,
    accounts: &[AccountWithBalance],
    report_date: &NaiveDate,
    org: &Option<Organization>,
) -> String {
    let account_nodes = build_account_nodes(accounts);
    let (total_debit, total_credit) = AccountWithBalance::calculate_totals(accounts);

    let mut typst_content = String::new();
    typst_content.push_str(&*build_table_header(
        &[t("common-account"), t("common-debit"), t("common-credit")],
        &[false, true, true],
    ));

    fn build_typst_rows(node: &AccountNode, depth: usize, content: &mut String, locale: Option<&str>) {
        let indent = "#h(2.0em)".repeat(depth);
        let (debit_display, credit_display) = match node.account.category {
            AccountCategory::Asset | AccountCategory::Expense => {
                if node.account.balance >= 0 {
                    (format_currency_icu_typ(node.account.balance, locale), "".to_string())
                } else {
                    (
                        "".to_string(),
                        format_currency_icu_typ(node.account.balance.abs(), locale),
                    )
                }
            }
            AccountCategory::Liability | AccountCategory::Equity | AccountCategory::Revenue => {
                if node.account.balance <= 0 {
                    (
                        "".to_string(),
                        format_currency_icu_typ(node.account.balance.abs(), locale),
                    )
                } else {
                    (format_currency_icu_typ(node.account.balance, locale), "".to_string())
                }
            }
        };
        content.push_str(&format!(
            "  [{} {}], align(right)[{}], align(right)[{}],\n",
            indent, node.account.name, debit_display, credit_display
        ));
        for child in &node.children {
            build_typst_rows(child, depth + 1, content, locale);
        }
    }

    for node in &account_nodes {
        build_typst_rows(node, 0, &mut typst_content, Some(&user.locale));
    }
    typst_content.push_str(&format!(
        "  [*{}*], align(right)[*{}*], align(right)[*{}*],\n",
        t("common-total"),
        (total_debit as f64) / 100.0,
        (total_credit as f64) / 100.0
    ));
    typst_content.push_str(")\n");

    let name = org.as_ref().map(|o| o.name.as_str());
    let report_qual = t_args(
        "balance-sheet-export-as-at",
        &fluent_args!["date" => report_date.format("%d %b %Y").to_string()],
    );
    wrap_report_layout(name, &t("trial-balance-title"), &*report_qual, typst_content.as_str())
}
