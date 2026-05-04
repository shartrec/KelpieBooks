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

use crate::components::layout::Layout;
use crate::contexts::report_context::{use_report_context, ReportAction};
use crate::Route;
use shared_core::dtos::account_with_balance::AccountWithBalance;
use shared_core::reports::balance_sheet::BalanceSheet;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use uuid::Uuid;
use yew::prelude::*;
use gloo_net::http::Request;
use yew_router::prelude::Link;
use shared_core::util::format_currency;

#[derive(Clone)]
pub struct BalanceSheetHolder {
    pub balance_sheet: BalanceSheet,
    pub timestamp: i64 ,
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

#[function_component(BalanceSheetPage)]
pub fn balance_sheet_page() -> Html {
    let report_ctx = use_report_context();
    let balance_sheet_holder = use_state(|| Rc::new(BalanceSheetHolder {
        balance_sheet: BalanceSheet::default(),
        timestamp: 0,
    }));
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let collapsed_nodes = use_state(HashSet::<Uuid>::new);

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
            report_ctx.dispatch(ReportAction::SetOnExportCsv(Some(Callback::from(move |_| {
                let url = format!("/api/reports/balance-sheet/export/csv?date={}", date);
                web_sys::window().unwrap().location().set_href(&url).unwrap();
            }))));
            report_ctx.dispatch(ReportAction::SetOnExportTypst(Some(Callback::from(move |_| {
                let url = format!("/api/reports/balance-sheet/export/typst?date={}", date);
                web_sys::window().unwrap().location().set_href(&url).unwrap();
            }))));
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

        use_effect_with(report_date, move |&report_date| {
            let balance_sheet_holder = balance_sheet_holder.clone();
            let loading = loading.clone();
            let error = error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                let url = format!("/api/reports/balance-sheet?date={}", report_date);
                match Request::get(&url).send().await {
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
                                Err(e) => error.set(Some(format!("Failed to parse Balance Sheet data: {}", e))),
                            }
                        } else {
                            error.set(Some(format!("Error fetching Balance Sheet: {}", resp.status())));
                        }
                    }
                    Err(e) => error.set(Some(format!("Network error: {}", e))),
                }
                loading.set(false);
            });
            || ()
        });
    }

    fn render_report_row(node: &AccountNode, depth: usize, collapsed: &UseStateHandle<HashSet<Uuid>>) -> Html {
        let is_parent = !node.children.is_empty();
        let is_collapsed = collapsed.contains(&node.account.id);
        let name_style = format!("padding-left: {}rem;", depth as f64 * 1.5);

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
                <tr class={if is_parent { "parent-account" } else { "" }}>
                    <td style={name_style}>
                        if is_parent {
                            <button onclick={on_toggle} class="collapse-toggle">
                                if is_collapsed {
                                    <img src="/images/chevron-right.svg" alt="Expand" />
                                } else {
                                    <img src="/images/chevron-down.svg" alt="Collapse" />
                                }
                            </button>
                        }
                        { account_name_display }
                    </td>
                    <td style="text-align: right;">
                        { format_currency(&node.account.balance) }
                    </td>
                </tr>
                if is_parent && !is_collapsed {
                    { for node.children.iter().map(|child| render_report_row(child, depth + 1, collapsed)) }
                }
            </>
        }
    }

    html! {
        <Layout>
            <div class="report-page">
                <h3>{ "Balance Sheet" }</h3>
                <p class="report-period">
                    { format!("As of {}", report_ctx.date_range.end_date) }
                </p>

                if *loading {
                    <p>{ "Loading..." }</p>
                } else if let Some(err) = &*error {
                    <div class="error">{ err }</div>
                } else {
                    <table class="table report-table">
                        <thead>
                            <tr>
                                <th>{ "Account" }</th>
                                <th style="text-align: right;">{ "Balance" }</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr class="report-section-header"><td colspan="2">{ "Assets" }</td></tr>
                            { for asset_nodes.iter().map(|node| render_report_row(node, 0, &collapsed_nodes)) }
                            <tr class="report-total-row">
                                <td><strong>{ "Total Assets" }</strong></td>
                                <td style="text-align: right;"><strong>{ format_currency(&balance_sheet_holder.balance_sheet.total_assets) }</strong></td>
                            </tr>

                            <tr class="report-section-header"><td colspan="2">{ "Liabilities" }</td></tr>
                            { for liability_nodes.iter().map(|node| render_report_row(node, 0, &collapsed_nodes)) }
                            <tr class="report-total-row">
                                <td><strong>{ "Total Liabilities" }</strong></td>
                                <td style="text-align: right;"><strong>{ format_currency(&balance_sheet_holder.balance_sheet.total_liabilities) }</strong></td>
                            </tr>

                            <tr class="report-section-header"><td colspan="2">{ "Equity" }</td></tr>
                            { for equity_nodes.iter().map(|node| render_report_row(node, 0, &collapsed_nodes)) }
                            <tr>
                                <td style="padding-left: 1.5rem">{ "Current Year Earnings" }</td>
                                <td style="text-align: right;">{ format_currency(&balance_sheet_holder.balance_sheet.net_income) }</td>
                            </tr>
                            <tr class="report-total-row">
                                <td><strong>{ "Total Equity" }</strong></td>
                                <td style="text-align: right;"><strong>{ format_currency(&balance_sheet_holder.balance_sheet.total_equity) }</strong></td>
                            </tr>

                            <tr class="report-total-row">
                                <td><strong>{ "Total Liabilities & Equity" }</strong></td>
                                <td style="text-align: right;"><strong>{ format_currency(&(balance_sheet_holder.balance_sheet.total_liabilities + balance_sheet_holder.balance_sheet.total_equity)) }</strong></td>
                            </tr>
                        </tbody>
                    </table>
                }
            </div>
        </Layout>
    }
}
