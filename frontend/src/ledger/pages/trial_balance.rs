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
use rust_decimal::dec;
use shared_core::ledger::{
    dtos::account_with_balance::AccountWithBalance,
    models::account_category::AccountCategory,
};
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::{
    use_navigator,
    Link,
};

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

#[derive(Clone, Debug, PartialEq)]
pub struct AccountNode {
    pub account: AccountWithBalance,
    pub children: Vec<AccountNode>,
}

// Helper function to build the hierarchical account nodes
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

#[function_component(TrialBalancePage)]
pub fn trial_balance_page() -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();
    let report_ctx = use_report_context();
    let accounts = use_state(|| Rc::new(Vec::<AccountWithBalance>::new()));
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let collapsed_nodes = use_state(HashSet::<Uuid>::new);

    let memoized_data = use_memo(accounts.clone(), |accounts| {
        let account_nodes = build_account_nodes(accounts);
        let (total_debit, total_credit) = AccountWithBalance::calculate_totals(accounts);
        (account_nodes, total_debit, total_credit)
    });

    let (account_nodes, total_debit, total_credit) = &*memoized_data;

    {
        let report_ctx = report_ctx.clone();
        use_effect_with((), move |_| {
            let report_ctx1 = report_ctx.clone();
            let date = report_ctx1.date_range.end_date;
            report_ctx1.dispatch(ReportAction::SetOnExportCsv(Some(Callback::from(
                move |_| {
                    let url = format!("/api/reports/trial-balance/export/csv?date={}", date);
                    web_sys::window()
                        .unwrap()
                        .location()
                        .set_href(&url)
                        .unwrap();
                },
            ))));
            let report_ctx1 = report_ctx.clone();
            let date = report_ctx1.date_range.end_date;
            report_ctx1.dispatch(ReportAction::SetOnExportTypst(Some(Callback::from(
                move |_| {
                    let url = format!("/api/reports/trial-balance/export/pdf?date={}", date);
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
        let accounts = accounts.clone();
        let loading = loading.clone();
        let error = error.clone();
        let report_date = report_ctx.date_range.end_date;
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();

        use_effect_with(report_date, move |&report_date| {
            let accounts = accounts.clone();
            let loading = loading.clone();
            let error = error.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                let url = format!("/api/reports/trial-balance?date={}", report_date);
                match Api::get(&url, user_ctx, navigator).await {
                    Ok(resp) => {
                        if resp.ok() {
                            match resp.json::<Vec<AccountWithBalance>>().await {
                                Ok(data) => {
                                    accounts.set(Rc::new(data));
                                    error.set(None);
                                }
                                Err(e) => error.set(Some(i18n.t_args(
                                    "trial-balance-error-parse",
                                    &fluent_args!["error" => e.to_string()],
                                ))),
                            }
                        } else {
                            error.set(Some(i18n.t_args(
                                "trial-balance-error-fetch",
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
        collapsed: &UseStateHandle<HashSet<Uuid>>,
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

        let (debit_display, credit_display) = match node.account.category {
            AccountCategory::Asset | AccountCategory::Expense => {
                if node.account.balance >= dec!(0.00) {
                    (i18n.format_currency(node.account.balance), "".to_string())
                } else {
                    (
                        "".to_string(),
                        i18n.format_currency(node.account.balance.abs()),
                    )
                }
            }
            AccountCategory::Liability | AccountCategory::Equity | AccountCategory::Revenue => {
                if node.account.balance <= dec!(0.00) {
                    (
                        "".to_string(),
                        i18n.format_currency(node.account.balance.abs()),
                    )
                } else {
                    (i18n.format_currency(node.account.balance), "".to_string())
                }
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
                    <td style="text-align: right;">{ debit_display }</td>
                    <td style="text-align: right;">{ credit_display }</td>
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
                    <h3>{ i18n.t("trial-balance-title") }</h3>
                    <ReportOptions show_start_date={false} show_end_date={true} />
                </div>
                if *loading {
                    <p>{ i18n.t("common-loading") }</p>
                } else if let Some(err) = &*error {
                    <div class="error">{ err }</div>
                } else {
                    <table class="report-table">
                        <thead>
                            <tr>
                                <th>{ i18n.t("common-account") }</th>
                                <th style="text-align: right;">{ i18n.t("common-debit") }</th>
                                <th style="text-align: right;">{ i18n.t("common-credit") }</th>
                            </tr>
                        </thead>
                        <tbody>
                            { for account_nodes.iter().map(|node| render_report_row(i18n.clone(), node, 0, &collapsed_nodes)) }
                            <tr class="report-total-row">
                                <td><strong>{ i18n.t("common-total") }</strong></td>
                                <td style="text-align: right;"><strong>{ i18n.format_currency(*total_debit) }</strong></td>
                                <td style="text-align: right;"><strong>{ i18n.format_currency(*total_credit) }</strong></td>
                            </tr>
                        </tbody>
                    </table>
                }
            </div>
        </Layout>
    }
}
