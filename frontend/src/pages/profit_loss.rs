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
use crate::utils::csv::download_csv;
use crate::utils::typst::download_typst;
use shared_core::dtos::account_with_balance::AccountWithBalance;
use shared_core::models::AccountCategory;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use yew::prelude::*;
use gloo_net::http::Request;
use yew_router::prelude::*;
use crate::Route;

#[derive(Clone, Debug, PartialEq)]
pub struct AccountNode {
    pub account: AccountWithBalance,
    pub children: Vec<AccountNode>,
}

fn build_account_nodes(accounts: &[AccountWithBalance]) -> (Vec<AccountNode>, Vec<AccountNode>, i64) {
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
    let report_ctx = use_report_context();
    let accounts = use_state(Vec::<AccountWithBalance>::new);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let collapsed_nodes = use_state(HashSet::<Uuid>::new);

    let (revenue_nodes, expense_nodes, net_income) = build_account_nodes(&accounts);

    {
        let report_ctx = report_ctx.clone();
        let accounts = accounts.clone();
        use_effect_with(accounts.clone(), move |_| {
            let accounts_csv = accounts.clone();
            let on_export_csv = Callback::from(move |_| {
                let (revenue_nodes, expense_nodes, net_income) = build_account_nodes(&accounts_csv);
                let mut csv_content = String::new();
                csv_content.push_str("Account,Balance\n");

                fn build_csv_rows(node: &AccountNode, depth: usize, content: &mut String) {
                    let indent = " ".repeat(depth * 2);
                    let display_balance = if node.account.category == AccountCategory::Revenue {
                        -node.account.balance
                    } else {
                        node.account.balance
                    };
                    content.push_str(&format!("\"{}{}\",\"{}\"\n", indent, node.account.name, (display_balance as f64) / 100.0));
                    for child in &node.children {
                        build_csv_rows(child, depth + 1, content);
                    }
                }

                csv_content.push_str("Revenue,\n");
                for node in &revenue_nodes {
                    build_csv_rows(node, 0, &mut csv_content);
                }
                csv_content.push_str("Expenses,\n");
                for node in &expense_nodes {
                    build_csv_rows(node, 0, &mut csv_content);
                }
                csv_content.push_str(&format!("\"Net Income\",\"{}\"\n", (net_income as f64) / 100.0));

                if let Err(e) = download_csv("profit_and_loss.csv", &csv_content) {
                    gloo_console::error!("Failed to download CSV:", e);
                }
            });

            let accounts_typst = accounts.clone();
            let on_export_typst = Callback::from(move |_| {
                let (revenue_nodes, expense_nodes, net_income) = build_account_nodes(&accounts_typst);
                let mut typst_content = String::new();
                typst_content.push_str("#set text(size: 10pt)\n");
                typst_content.push_str("#set page(margin: (top: 2cm, bottom: 2cm, left: 1.5cm, right: 1.5cm))\n\n");
                typst_content.push_str("= Profit & Loss\n\n");

                fn build_typst_rows(node: &AccountNode, depth: usize, content: &mut String) {
                    let indent = " ".repeat(depth * 2);
                    let display_balance = if node.account.category == AccountCategory::Revenue {
                        -node.account.balance
                    } else {
                        node.account.balance
                    };
                    content.push_str(&format!("{}{}[{:.2}]\n", indent, node.account.name, (display_balance as f64) / 100.0));
                    for child in &node.children {
                        build_typst_rows(child, depth + 1, content);
                    }
                }

                typst_content.push_str("== Revenue\n\n");
                for node in &revenue_nodes {
                    build_typst_rows(node, 0, &mut typst_content);
                }
                typst_content.push_str("\n== Expenses\n\n");
                for node in &expense_nodes {
                    build_typst_rows(node, 0, &mut typst_content);
                }
                typst_content.push_str(&format!("\n*Net Income:* {:.2}\n", (net_income as f64) / 100.0));

                if let Err(e) = download_typst("profit_and_loss.typ", &typst_content) {
                    gloo_console::error!("Failed to download Typst file:", e);
                }
            });

            report_ctx.dispatch(ReportAction::SetOnExportCsv(Some(on_export_csv)));
            report_ctx.dispatch(ReportAction::SetOnExportTypst(Some(on_export_typst)));
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

        use_effect_with((start_date, end_date), move |(start, end)| {
            let accounts = accounts.clone();
            let loading = loading.clone();
            let error = error.clone();
            let start = *start;
            let end = *end;

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                let url = format!("/api/reports/profit-loss?start={}&end={}", start, end);
                match Request::get(&url).send().await {
                    Ok(resp) => {
                        if resp.ok() {
                            match resp.json::<Vec<AccountWithBalance>>().await {
                                Ok(data) => {
                                    accounts.set(data);
                                    error.set(None);
                                }
                                Err(e) => error.set(Some(format!("Failed to parse P&L data: {}", e))),
                            }
                        } else {
                            error.set(Some(format!("Error fetching P&L: {}", resp.status())));
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
                    if is_parent {
                        <td />
                    }
                    <td style="text-align: right;">
                        { format!("{:.2}", (display_balance as f64) / 100.0) }
                    </td>
                    if !is_parent {
                        <td />
                    }
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
                <h3>{ "Profit & Loss" }</h3>
                <p class="report-period">
                    { format!("Period: {} to {}", report_ctx.date_range.start_date, report_ctx.date_range.end_date) }
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
                                <th></th>
                                <th></th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr class="report-section-header"><td colspan="2">{ "Revenue" }</td><td></td></tr>
                            { for revenue_nodes.iter().map(|node| render_report_row(node, 0, &collapsed_nodes)) }
                            <tr class="report-section-header"><td colspan="2">{ "Expenses" }</td><td></td></tr>
                            { for expense_nodes.iter().map(|node| render_report_row(node, 0, &collapsed_nodes)) }

                            <tr class="report-total-row">
                                <td><strong>{ "Net Income" }</strong></td>
                                <td />
                                <td style="text-align: right;">
                                    <strong>{ format!("{:.2}", (net_income as f64) / 100.0) }</strong>
                                </td>
                            </tr>
                        </tbody>
                    </table>
                }
            </div>
        </Layout>
    }
}
