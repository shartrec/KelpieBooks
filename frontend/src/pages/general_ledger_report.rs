/*
 * Copyright (c) 2026-2026. Trevor Campbell and others.
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
use crate::components::report_options::ReportOptions;
use crate::contexts::report_context::use_report_context;
use shared_core::dtos::general_ledger_line::GeneralLedgerLine;
use yew::prelude::*;
use crate::api::Api;
use crate::contexts::auth_context::use_user_context;
use yew_router::prelude::*;
use shared_core::util::format_currency;
use crate::router::Route;

#[function_component(GeneralLedgerReportPage)]
pub fn general_ledger_report_page() -> Html {
    let user_ctx = use_user_context();
    let navigator = use_navigator().unwrap();
    let report_ctx = use_report_context();
    let report_data = use_state(|| Vec::<GeneralLedgerLine>::new());
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let report_data = report_data.clone();
        let loading = loading.clone();
        let error = error.clone();
        let start_date = report_ctx.date_range.start_date;
        let end_date = report_ctx.date_range.end_date;
        let selected_accounts = report_ctx.selected_accounts.clone();
        let min_amount = report_ctx.min_amount;
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();

        use_effect_with((start_date, end_date, selected_accounts.clone(), min_amount), move |(start, end, accounts, min_amount)| {
            let report_data = report_data.clone();
            let loading = loading.clone();
            let error = error.clone();
            let start = *start;
            let end = *end;
            let accounts = accounts.clone();
            let min_amount = *min_amount;
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                let mut url = format!("/api/reports/general-ledger?start={}&end={}", start, end);
                if !accounts.is_empty() {
                    let account_ids = accounts.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(",");
                    url.push_str(&format!("&accounts={}", account_ids));
                }
                if let Some(amount) = min_amount {
                    url.push_str(&format!("&min_amount={}", amount));
                }

                match Api::get(&url, user_ctx, navigator).await {
                    Ok(resp) => {
                        if resp.ok() {
                            match resp.json::<Vec<GeneralLedgerLine>>().await {
                                Ok(data) => {
                                    report_data.set(data);
                                    error.set(None);
                                }
                                Err(e) => error.set(Some(format!("Failed to parse report data: {}", e))),
                            }
                        } else {
                            error.set(Some(format!("Error fetching report: {}", resp.status())));
                        }
                    }
                    Err(e) => error.set(Some(format!("Network error: {}", e))),
                }
                loading.set(false);
            });
            || ()
        });
    }

    let grouped_data = use_memo(report_data.clone(), |data| {
        let mut grouped = std::collections::BTreeMap::new();
        for line in data.iter() {
            grouped.entry(line.account_name.clone()).or_insert_with(Vec::new).push(line.clone());
        }
        grouped
    });

    html! {
        <Layout>
            <div class="report-page">
                <div class="report-header">
                    <h3>{ "General Ledger Detail" }</h3>
                    <ReportOptions show_start_date={true} show_end_date={true} show_advanced_filters={true} />
                </div>
                if *loading {
                    <p>{ "Loading..." }</p>
                } else if let Some(err) = &*error {
                    <div class="error">{ err }</div>
                } else {
                    <table class="report-table">
                        <thead>
                            <tr>
                                <th>{ "Date" }</th>
                                <th>{ "Description" }</th>
                                <th class="text-amount">{ "Debit" }</th>
                                <th class="text-amount">{ "Credit" }</th>
                                <th class="text-amount">{ "Balance" }</th>
                            </tr>
                        </thead>
                        <tbody>
                            { for grouped_data.iter().map(|(account_name, lines)| html! {
                                <>
                                    <tr class="report__section-header">
                                        <td colspan="5">{ account_name }</td>
                                    </tr>
                                    { for lines.iter().map(|line| html! {
                                        <tr>
                                            <td>
                                                <Link<Route> to={Route::AccountLedger { id: line.account_id }}>
                                                    { &line.date.to_string() }
                                                </Link<Route>>
                                            </td>
                                            <td>{ line.description.clone().unwrap_or_default() }</td>
                                            <td class="text-amount">{ format_currency(&line.debit) }</td>
                                            <td class="text-amount">{ format_currency(&line.credit) }</td>
                                            <td class="text-amount">{ format_currency(&line.balance) }</td>
                                        </tr>
                                    })}
                                </>
                            })}
                        </tbody>
                    </table>
                }
            </div>
        </Layout>
    }
}
