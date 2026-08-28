/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use std::{
    collections::{
        HashMap,
        HashSet,
    },
    rc::Rc,
};

use fluent::fluent_args;
use shared_core::ledger::dtos::{
    account_with_balance::AccountWithBalance,
    balance_sheet::BalanceSheet,
};
use yew::prelude::*;
use yew_router::prelude::{
    use_navigator,
    Link,
};
use shared_core::AccountId;
use crate::{
    api::Api,
    contexts::{
        auth_context::use_user_context,
        locale_context::{
            use_locale,
            LocaleContext,
        },
        report_context::{
            use_report_context,
            ReportAction,
        },
    },
    core::components::{
        layout::Layout,
        report_options::ReportOptions,
    },
    router::Route,
};

#[derive(Clone)]
pub struct BalanceSheetHolder {
    pub balance_sheet: BalanceSheet,
    pub timestamp: i64,
}

impl PartialEq for BalanceSheetHolder {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
    fn ne(&self, other: &Self) -> bool {
        self.timestamp != other.timestamp
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AccountNode {
    pub account: AccountWithBalance,
    pub children: Vec<AccountNode>,
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

#[function_component(BalanceSheetPage)]
pub fn balance_sheet_page() -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();
    let report_ctx = use_report_context();

    let balance_sheet_holder = use_state(|| {
        Rc::new(BalanceSheetHolder {
            balance_sheet: BalanceSheet::default(),
            timestamp: 0,
        })
    });
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let collapsed_nodes = use_state(HashSet::<AccountId>::new);

    let memoized_nodes = use_memo(balance_sheet_holder.clone(), |bs_holder| {
        (
            build_account_nodes(&bs_holder.balance_sheet.assets),
            build_account_nodes(&bs_holder.balance_sheet.liabilities),
            build_account_nodes(&bs_holder.balance_sheet.equity),
        )
    });

    let (asset_nodes, liability_nodes, equity_nodes) = (*memoized_nodes).clone();

    {
        let report_ctx = report_ctx.clone();
        use_effect_with((), move |_| {
            let date = report_ctx.date_range.end_date;
            report_ctx.dispatch(ReportAction::SetOnExportCsv(Some(Callback::from(
                move |_| {
                    let url = format!("/api/reports/balance-sheet/export/csv?date={}", date);
                    web_sys::window()
                        .unwrap()
                        .location()
                        .set_href(&url)
                        .unwrap();
                },
            ))));
            report_ctx.dispatch(ReportAction::SetOnExportTypst(Some(Callback::from(
                move |_| {
                    let url = format!("/api/reports/balance-sheet/export/pdf?date={}", date);
                    web_sys::window()
                        .unwrap()
                        .location()
                        .set_href(&url)
                        .unwrap();
                },
            ))));
            move || {
                report_ctx.dispatch(ReportAction::SetOnExportCsv(None));
                report_ctx.dispatch(ReportAction::SetOnExportTypst(None));
            }
        });
    }

    {
        let balance_sheet_holder = balance_sheet_holder.clone();
        let loading = loading.clone();
        let error = error.clone();
        let report_date = report_ctx.date_range.end_date;
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();

        use_effect_with(report_date, move |&report_date| {
            let balance_sheet_holder = balance_sheet_holder.clone();
            let loading = loading.clone();
            let error = error.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                let url = format!("/api/reports/balance-sheet?date={}", report_date);
                match Api::get(&url, user_ctx, navigator).await {
                    Ok(resp) => {
                        if resp.ok() {
                            match resp.json::<BalanceSheet>().await {
                                Ok(data) => {
                                    let data_holder = BalanceSheetHolder {
                                        balance_sheet: data,
                                        timestamp: chrono::Utc::now().timestamp(),
                                    };
                                    balance_sheet_holder.set(Rc::new(data_holder));
                                    error.set(None);
                                }
                                Err(e) => error.set(Some(i18n.t_args(
                                    "balance-sheet-error-parse",
                                    &fluent_args!["error" => e.to_string()],
                                ))),
                            }
                        } else {
                            error.set(Some(i18n.t_args(
                                "balance-sheet-error-fetch",
                                &fluent_args!["status" => resp.status()],
                            )));
                        }
                    }
                    Err(e) => error.set(Some(i18n.t_args(
                        "common-network-error",
                        &fluent_args!["error" => e.to_string()],
                    ))),
                }
                loading.set(false);
            });
            || ()
        });
    }

    fn render_report_row(
        i18n: LocaleContext,
        node: &AccountNode,
        depth: usize,
        collapsed: &UseStateHandle<HashSet<AccountId>>,
    ) -> Html {
        let is_parent = !node.children.is_empty();
        let is_collapsed = collapsed.contains(&node.account.id);
        let indent_class = format!("report__indent__level_{}", depth);

        let on_toggle = {
            let collapsed = collapsed.clone();
            let id = node.account.id;
            Callback::from(move |_| {
                let mut new_set = (*collapsed).clone();
                if new_set.contains(&id) {
                    new_set.remove(&id);
                } else {
                    new_set.insert(id);
                }
                collapsed.set(new_set);
            })
        };

        let account_name_display = if node.account.is_group {
            html! { { &node.account.name } }
        } else {
            html! {
                <Link<Route> to={Route::AccountLedger { id: node.account.id }}>
                    { &node.account.name }
                </Link<Route>>
            }
        };

        html! {
            <>
                <tr class={format!{"{} {}", indent_class, if is_parent { "parent-account "} else { "" }}}>
                    <td>
                        if is_parent {
                            <button onclick={on_toggle} class="collapse-toggle">
                                if is_collapsed {
                                    <img src="/images/chevron-right.svg" alt={i18n.t("common-expand")} />
                                } else {
                                    <img src="/images/chevron-down.svg" alt={i18n.t("common-collapse")} />
                                }
                            </button>
                        }
                        { account_name_display }
                    </td>
                    <td style="text-align: right;">
                        { i18n.format_currency(node.account.balance) }
                    </td>
                </tr>
                if is_parent && !is_collapsed {
                    { for node.children.iter().map(|child| render_report_row(i18n.clone(), child, depth + 1, collapsed)) }
                }
            </>
        }
    }

    html! {
        <Layout>
            <div class="report-page">
                <div class="report-header">
                    <h3>{ i18n.t("balance-sheet-title") }</h3>
                    <ReportOptions show_start_date={false} show_end_date={true} />
                </div>

                if *loading {
                    <p>{ i18n.t("common-loading") }</p>
                } else if let Some(err) = &*error {
                    <div class="message__error">{ err }</div>
                } else {
                    <table class="report-table">
                        <thead>
                            <tr>
                                <th>{ i18n.t("common-account") }</th>
                                <th style="text-align: right;">{ i18n.t("common-balance") }</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr class="report__section-header"><td colspan="2">{ i18n.t("balance-sheet-assets-section") }</td></tr>
                            { for asset_nodes.iter().map(|node| render_report_row(i18n.clone(), node, 0, &collapsed_nodes)) }
                            <tr class="report-total-row">
                                <td><strong>{ i18n.t("balance-sheet-total-assets") }</strong></td>
                                <td style="text-align: right;"><strong>{ i18n.format_currency(balance_sheet_holder.balance_sheet.total_assets) }</strong></td>
                            </tr>

                            <tr class="report__section-header"><td colspan="2">{ i18n.t("balance-sheet-liabilities-section") }</td></tr>
                            { for liability_nodes.iter().map(|node| render_report_row(i18n.clone(), node, 0, &collapsed_nodes)) }
                            <tr class="report-total-row">
                                <td><strong>{ i18n.t("balance-sheet-total-liabilities") }</strong></td>
                                <td style="text-align: right;"><strong>{ i18n.format_currency(balance_sheet_holder.balance_sheet.total_liabilities) }</strong></td>
                            </tr>

                            <tr class="report__section-header"><td colspan="2">{ i18n.t("balance-sheet-equity-section") }</td></tr>
                            { for equity_nodes.iter().map(|node| render_report_row(i18n.clone(), node, 0, &collapsed_nodes)) }
                            <tr>
                                <td style="padding-left: 1.5rem">{ i18n.t("balance-sheet-current-year-earnings") }</td>
                                <td style="text-align: right;">{ i18n.format_currency(balance_sheet_holder.balance_sheet.net_income) }</td>
                            </tr>
                            <tr class="report-total-row">
                                <td><strong>{ i18n.t("balance-sheet-total-equity") }</strong></td>
                                <td style="text-align: right;"><strong>{ i18n.format_currency(balance_sheet_holder.balance_sheet.total_equity) }</strong></td>
                            </tr>

                            <tr class="report__total-row">
                                <td><strong>{ i18n.t("balance-sheet-total-liabilities-equity") }</strong></td>
                                <td style="text-align: right;"><strong>{ i18n.format_currency(balance_sheet_holder.balance_sheet.total_liabilities + balance_sheet_holder.balance_sheet.total_equity) }</strong></td>
                            </tr>
                        </tbody>
                    </table>
                }
            </div>
        </Layout>
    }
}
