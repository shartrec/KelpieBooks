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
use crate::Route;
use shared_core::dtos::account_with_balance::AccountWithBalance;
use shared_core::reports::balance_sheet::BalanceSheet;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use yew::prelude::*;
use gloo_net::http::Request;
use yew_router::prelude::Link;

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
    let balance_sheet = use_state(BalanceSheet::default);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let collapsed_nodes = use_state(HashSet::<Uuid>::new);

    let asset_nodes = build_account_nodes(&balance_sheet.assets);
    let liability_nodes = build_account_nodes(&balance_sheet.liabilities);
    let equity_nodes = build_account_nodes(&balance_sheet.equity);

    {
        let report_ctx = report_ctx.clone();
        let balance_sheet = balance_sheet.clone();

        use_effect_with((), move |_| {
            let balance_sheet_csv = balance_sheet.clone();
            let on_export_csv = Callback::from(move |_| {
                let asset_nodes = build_account_nodes(&balance_sheet_csv.assets);
                let liability_nodes = build_account_nodes(&balance_sheet_csv.liabilities);
                let equity_nodes = build_account_nodes(&balance_sheet_csv.equity);
                let mut csv_content = String::new();
                csv_content.push_str("Account,Balance\n");

                fn build_csv_rows(node: &AccountNode, depth: usize, content: &mut String) {
                    let indent = " ".repeat(depth * 2);
                    content.push_str(&format!("\"{}{}\",\"{}\"\n", indent, node.account.name, (node.account.balance as f64) / 100.0));
                    for child in &node.children {
                        build_csv_rows(child, depth + 1, content);
                    }
                }

                csv_content.push_str("Assets,\n");
                for node in &asset_nodes {
                    build_csv_rows(node, 0, &mut csv_content);
                }
                csv_content.push_str(&format!("\"Total Assets\",\"{}\"\n", (balance_sheet_csv.total_assets as f64) / 100.0));

                csv_content.push_str("Liabilities,\n");
                for node in &liability_nodes {
                    build_csv_rows(node, 0, &mut csv_content);
                }
                csv_content.push_str(&format!("\"Total Liabilities\",\"{}\"\n", (balance_sheet_csv.total_liabilities as f64) / 100.0));

                csv_content.push_str("Equity,\n");
                for node in &equity_nodes {
                    build_csv_rows(node, 0, &mut csv_content);
                }
                csv_content.push_str(&format!("\"Current Year Earnings\",\"{}\"\n", (balance_sheet_csv.net_income as f64) / 100.0));
                csv_content.push_str(&format!("\"Total Equity\",\"{}\"\n", (balance_sheet_csv.total_equity as f64) / 100.0));
                csv_content.push_str(&format!("\"Total Liabilities & Equity\",\"{}\"\n", ((balance_sheet_csv.total_liabilities + balance_sheet_csv.total_equity) as f64) / 100.0));

                if let Err(e) = download_csv("balance_sheet.csv", &csv_content) {
                    gloo_console::error!("Failed to download CSV:", e);
                }
            });

            let balance_sheet_typst = balance_sheet.clone();
            let on_export_typst = Callback::from(move |_| {
                let asset_nodes = build_account_nodes(&balance_sheet_typst.assets);
                let liability_nodes = build_account_nodes(&balance_sheet_typst.liabilities);
                let equity_nodes = build_account_nodes(&balance_sheet_typst.equity);
                let mut typst_content = String::new();
                typst_content.push_str("#set text(size: 10pt)\n");
                typst_content.push_str("#set page(margin: (top: 2cm, bottom: 2cm, left: 1.5cm, right: 1.5cm))\n\n");
                typst_content.push_str("= Balance Sheet\n\n");

                fn build_typst_rows(node: &AccountNode, depth: usize, content: &mut String) {
                    let indent = " ".repeat(depth * 2);
                    content.push_str(&format!("{}{}[{:.2}]\n", indent, node.account.name, (node.account.balance as f64) / 100.0));
                    for child in &node.children {
                        build_typst_rows(child, depth + 1, content);
                    }
                }

                typst_content.push_str("== Assets\n\n");
                for node in &asset_nodes {
                    build_typst_rows(node, 0, &mut typst_content);
                }
                typst_content.push_str(&format!("\n*Total Assets:* {:.2}\n", (balance_sheet_typst.total_assets as f64) / 100.0));

                typst_content.push_str("\n== Liabilities\n\n");
                for node in &liability_nodes {
                    build_typst_rows(node, 0, &mut typst_content);
                }
                typst_content.push_str(&format!("\n*Total Liabilities:* {:.2}\n", (balance_sheet_typst.total_liabilities as f64) / 100.0));

                typst_content.push_str("\n== Equity\n\n");
                for node in &equity_nodes {
                    build_typst_rows(node, 0, &mut typst_content);
                }
                typst_content.push_str(&format!("\n  Current Year Earnings: {:.2}\n", (balance_sheet_typst.net_income as f64) / 100.0));
                typst_content.push_str(&format!("\n*Total Equity:* {:.2}\n", (balance_sheet_typst.total_equity as f64) / 100.0));
                typst_content.push_str(&format!("\n*Total Liabilities & Equity:* {:.2}\n", ((balance_sheet_typst.total_liabilities + balance_sheet_typst.total_equity) as f64) / 100.0));

                if let Err(e) = download_typst("balance_sheet.typ", &typst_content) {
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
        let balance_sheet = balance_sheet.clone();
        let loading = loading.clone();
        let error = error.clone();
        let report_date = report_ctx.date_range.end_date;

        use_effect_with(report_date, move |&report_date| {
            let balance_sheet = balance_sheet.clone();
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
                                    balance_sheet.set(data);
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
                        { format!("{:.2}", (node.account.balance as f64) / 100.0) }
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
                                <td style="text-align: right;"><strong>{ format!("{:.2}", (balance_sheet.total_assets as f64) / 100.0) }</strong></td>
                            </tr>

                            <tr class="report-section-header"><td colspan="2">{ "Liabilities" }</td></tr>
                            { for liability_nodes.iter().map(|node| render_report_row(node, 0, &collapsed_nodes)) }
                            <tr class="report-total-row">
                                <td><strong>{ "Total Liabilities" }</strong></td>
                                <td style="text-align: right;"><strong>{ format!("{:.2}", (balance_sheet.total_liabilities as f64) / 100.0) }</strong></td>
                            </tr>

                            <tr class="report-section-header"><td colspan="2">{ "Equity" }</td></tr>
                            { for equity_nodes.iter().map(|node| render_report_row(node, 0, &collapsed_nodes)) }
                            <tr>
                                <td style="padding-left: 1.5rem">{ "Current Year Earnings" }</td>
                                <td style="text-align: right;">{ format!("{:.2}", (balance_sheet.net_income as f64) / 100.0) }</td>
                            </tr>
                            <tr class="report-total-row">
                                <td><strong>{ "Total Equity" }</strong></td>
                                <td style="text-align: right;"><strong>{ format!("{:.2}", (balance_sheet.total_equity as f64) / 100.0) }</strong></td>
                            </tr>

                            <tr class="report-total-row">
                                <td><strong>{ "Total Liabilities & Equity" }</strong></td>
                                <td style="text-align: right;"><strong>{ format!("{:.2}", ((balance_sheet.total_liabilities + balance_sheet.total_equity) as f64) / 100.0) }</strong></td>
                            </tr>
                        </tbody>
                    </table>
                }
            </div>
        </Layout>
    }
}
