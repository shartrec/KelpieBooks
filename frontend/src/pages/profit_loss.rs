/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::api::Api;
use crate::components::layout::Layout;
use crate::components::report_options::ReportOptions;
use crate::contexts::auth_context::use_user_context;
use crate::contexts::report_context::{use_report_context, ReportAction};
use crate::router::Route;
use fluent::fluent_args;
use shared_core::dtos::account_with_balance::AccountWithBalance;
use shared_core::i18n::{t, t_args};
use shared_core::models::account_category::AccountCategory;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::contexts::locale_context::{use_locale, LocaleContext};

#[derive(Clone, Debug, PartialEq)]
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

#[function_component(ProfitLossPage)]
pub fn profit_loss_page() -> Html {
    let user_ctx = use_user_context();
    let navigator = use_navigator().unwrap();
    let report_ctx = use_report_context();
    let accounts = use_state(|| Rc::new(Vec::<AccountWithBalance>::new()));
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let collapsed_nodes = use_state(HashSet::<Uuid>::new);

    let memoized_data = use_memo(accounts.clone(), |accounts| build_account_nodes(accounts));

    let (revenue_nodes, expense_nodes, net_income) = (*memoized_data).clone();

    {
        let report_ctx = report_ctx.clone();
        use_effect_with((), move |_| {
            let start_date = report_ctx.date_range.start_date;
            let end_date = report_ctx.date_range.end_date;
            report_ctx.dispatch(ReportAction::SetOnExportCsv(Some(Callback::from(
                move |_| {
                    let url = format!(
                        "/api/reports/profit-loss/export/csv?start={}&end={}",
                        start_date, end_date
                    );
                    web_sys::window()
                        .unwrap()
                        .location()
                        .set_href(&url)
                        .unwrap();
                },
            ))));
            report_ctx.dispatch(ReportAction::SetOnExportTypst(Some(Callback::from(
                move |_| {
                    let url = format!(
                        "/api/reports/profit-loss/export/pdf?start={}&end={}",
                        start_date, end_date
                    );
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
        let start_date = report_ctx.date_range.start_date;
        let end_date = report_ctx.date_range.end_date;
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();

        use_effect_with((start_date, end_date), move |(start, end)| {
            let accounts = accounts.clone();
            let loading = loading.clone();
            let error = error.clone();
            let start = *start;
            let end = *end;
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                let url = format!("/api/reports/profit-loss?start={}&end={}", start, end);
                match Api::get(&url, user_ctx, navigator).await {
                    Ok(resp) => {
                        if resp.ok() {
                            match resp.json::<Vec<AccountWithBalance>>().await {
                                Ok(data) => {
                                    accounts.set(Rc::new(data));
                                    error.set(None);
                                }
                                Err(e) => {
                                    error.set(Some(t_args(
                                        "profit-loss-error-parse",
                                        &fluent_args!["error" => e.to_string()],
                                    )))
                                }
                            }
                        } else {
                            error.set(Some(t_args(
                                "profit-loss-error-fetch",
                                &fluent_args!["status" => resp.status()],
                            )));
                        }
                    }
                    Err(e) => error.set(Some(t_args(
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

        let display_balance = if node.account.category == AccountCategory::Revenue {
            -node.account.balance
        } else {
            node.account.balance
        };

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
                                    <img src="/images/chevron-right.svg" alt={t("common-expand")} />
                                } else {
                                    <img src="/images/chevron-down.svg" alt={t("common-collapse")} />
                                }
                            </button>
                        }
                        { account_name_display }
                    </td>
                    if is_parent {
                        <td />
                    }
                    <td style="text-align: right;">
                        { i18n.format_currency(display_balance) }
                    </td>
                    if !is_parent {
                        <td />
                    }
                </tr>
                if is_parent && !is_collapsed {
                    { for node.children.iter().map(|child| render_report_row(i18n.clone(), child, depth + 1, collapsed)) }
                }
            </>
        }
    }

    let i18n = use_locale();
    html! {
        <Layout>
            <div class="report-page">
                <div class="report-header">
                    <h3>{ t("profit-loss-title") }</h3>
                    <ReportOptions show_start_date={true} show_end_date={true} />
                </div>
                if *loading {
                    <p>{ t("common-loading") }</p>
                } else if let Some(err) = &*error {
                    <div class="error">{ err }</div>
                } else {
                    <table class="report-table">
                        <thead>
                            <tr>
                                <th>{ t("common-account") }</th>
                                <th></th>
                                <th></th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr class="report__section-header"><td colspan="2">{ t("profit-loss-revenue-section") }</td><td></td></tr>
                            { for revenue_nodes.iter().map(|node| render_report_row(i18n.clone(), node, 0, &collapsed_nodes)) }
                            <tr class="report__section-header"><td colspan="2">{ t("profit-loss-expenses-section") }</td><td></td></tr>
                            { for expense_nodes.iter().map(|node| render_report_row(i18n.clone(), node, 0, &collapsed_nodes)) }

                            <tr class="report__total-row">
                                <td><strong>{ t("profit-loss-net-income") }</strong></td>
                                <td />
                                <td style="text-align: right;">
                                    <strong>{ i18n.format_currency(net_income) }</strong>
                                </td>
                            </tr>
                        </tbody>
                    </table>
                }
            </div>
        </Layout>
    }
}
