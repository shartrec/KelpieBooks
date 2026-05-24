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

use crate::api::Api;
use crate::contexts::auth_context::use_user_context;
use chrono::Local;
use shared_core::dtos::aged_payable_summary::AgedPayableSummary;
use shared_core::util::format_currency;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[function_component(AgedTrialBalanceMatrix)]
pub fn aged_trial_balance_matrix() -> Html {
    let user_ctx = use_user_context();
    let navigator = use_navigator().unwrap();
    let summary = use_state(Vec::new);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);
    let expanded_rows = use_state(Vec::new);

    let fetch_summary = {
        let summary = summary.clone();
        let error = error.clone();
        let loading = loading.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        Callback::from(move |_: ()| {
            let summary = summary.clone();
            let error = error.clone();
            let loading = loading.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            loading.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                let date = Local::now().date_naive();
                let url = format!("/api/reports/aged-payables?date={}", date);
                let fetched_summary = Api::get(&url, user_ctx, navigator).await;
                loading.set(false);
                match fetched_summary {
                    Ok(response) if response.ok() => {
                        match response.json::<Vec<AgedPayableSummary>>().await {
                            Ok(data) => {
                                summary.set(data);
                            }
                            Err(e) => error.set(Some(format!("Failed to parse summary: {}", e))),
                        }
                    }
                    Ok(response) => error.set(Some(format!(
                        "Failed to fetch summary: {}",
                        response.status()
                    ))),
                    Err(e) => error.set(Some(format!("Network error: {}", e))),
                }
            });
        })
    };

    use_effect_with((), move |()| {
        fetch_summary.emit(());
        || ()
    });

    let toggle_row = {
        let expanded_rows = expanded_rows.clone();
        Callback::from(move |partner_id: Uuid| {
            let mut new_expanded = (*expanded_rows).clone();
            if let Some(pos) = new_expanded.iter().position(|&id| id == partner_id) {
                new_expanded.remove(pos);
            } else {
                new_expanded.push(partner_id);
            }
            expanded_rows.set(new_expanded);
        })
    };

    if *loading {
        return html! { <p>{ "Loading..." }</p> };
    }
    if let Some(err) = &*error {
        return html! { <div class="error">{ err }</div> };
    }

    html! {
        <table class="table">
            <thead>
                <tr>
                    <th>{ "Vendor" }</th>
                    <th class="table__value-col">{ "Current" }</th>
                    <th class="table__value-col">{ "1-30 Days" }</th>
                    <th class="table__value-col">{ "31-60 Days" }</th>
                    <th class="table__value-col">{ "61-90 Days" }</th>
                    <th class="table__value-col">{ "90+ Days" }</th>
                    <th class="table__value-col">{ "Total" }</th>
                </tr>
            </thead>
            <tbody>
                { for (*summary).iter().map(|summary| {
                    let is_expanded = expanded_rows.contains(&summary.partner_id);
                    let on_toggle = {
                        let toggle_row = toggle_row.clone();
                        let partner_id = summary.partner_id;
                        Callback::from(move |_| {
                            toggle_row.emit(partner_id);
                        })
                    };
                    html! {
                        <>
                            <tr onclick={on_toggle}>
                                <td>{ &summary.partner_name }</td>
                                <td class="table__value-col">{ format_currency(&summary.current) }</td>
                                <td class="table__value-col">{ format_currency(&summary.days_30) }</td>
                                <td class={classes!("table__value-col", if summary.days_60 > 0 { "col-severe" } else { "" })}>{ format_currency(&summary.days_60) }</td>
                                <td class={classes!("table__value-col", if summary.days_90 > 0 { "col-severe" } else { "" })}>{ format_currency(&summary.days_90) }</td>
                                <td class={classes!("table__value-col", if summary.days_90_plus > 0 { "col-severe" } else { "" })}>{ format_currency(&summary.days_90_plus) }</td>
                                <td class="table__value-col">{ format_currency(&summary.total) }</td>
                            </tr>
                            if is_expanded {
                                { for summary.invoices.iter().map(|invoice| {
                                    html! {
                                        <tr class="sub-row">
                                            <td class="table__text-col" colspan="2">{ &invoice.invoice_number }</td>
                                            <td class="table__value-col">{ invoice.issue_date.format("%d %b %Y").to_string() }</td>
                                            <td class="table__value-col">{ invoice.due_date.format("%d %b %Y").to_string() }</td>
                                            <td class="table__value-col" colspan="2">{ format_currency(&invoice.amount_remaining) }</td>
                                            <td class="table__value-col">{ "" }</td>
                                        </tr>
                                    }
                                })}
                            }
                        </>
                    }
                })}
            </tbody>
        </table>
    }
}
