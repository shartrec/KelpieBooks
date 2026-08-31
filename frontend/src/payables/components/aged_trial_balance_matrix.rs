/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::Local;
use fluent::fluent_args;
use rust_decimal::dec;
use shared_core::{
    payables::dtos::aged_payable_summary::AgedPayableSummary,
    PartnerId,
};
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{
    api::Api,
    contexts::{
        auth_context::use_user_context,
        locale_context::use_locale,
    },
};

#[function_component(AgedTrialBalanceMatrix)]
pub fn aged_trial_balance_matrix() -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
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
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        let i18n = i18n.clone();
        Callback::from(move |_: ()| {
            let summary = summary.clone();
            let error = error.clone();
            let loading = loading.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
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
                            Err(e) => error.set(Some(i18n.t_args(
                                "aged-trial-balance-error-parse",
                                &fluent_args!["error" => e.to_string()],
                            ))),
                        }
                    }
                    Ok(response) => error.set(Some(i18n.t_args(
                        "aged-trial-balance-error-fetch",
                        &fluent_args!["status" => response.status()],
                    ))),
                    Err(e) => error.set(Some(i18n.t_args(
                        "common-network-error",
                        &fluent_args!["error" => e.to_string()],
                    ))),
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
        Callback::from(move |partner_id: PartnerId| {
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
        return html! { <p>{ i18n.t("common-loading") }</p> };
    }
    if let Some(err) = &*error {
        return html! { <div class="message__error">{ err }</div> };
    }

    html! {
        <table class="table">
            <thead>
                <tr>
                    <th>{ i18n.t("common-vendor") }</th>
                    <th class="table__value-col">{ i18n.t("aged-trial-balance-current") }</th>
                    <th class="table__value-col">{ i18n.t("aged-trial-balance-1-30-days") }</th>
                    <th class="table__value-col">{ i18n.t("aged-trial-balance-31-60-days") }</th>
                    <th class="table__value-col">{ i18n.t("aged-trial-balance-61-90-days") }</th>
                    <th class="table__value-col">{ i18n.t("aged-trial-balance-90-plus-days") }</th>
                    <th class="table__value-col">{ i18n.t("common-total") }</th>
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
                                <td>
                                    <button class="collapse-toggle">
                                        if !is_expanded {
                                            <img src="/images/chevron-right.svg" alt={i18n.t("common-expand")} />
                                        } else {
                                            <img src="/images/chevron-down.svg" alt={i18n.t("common-collapse")} />
                                        }
                                    </button>
                                    { &summary.partner_name }
                                </td>
                                <td class="table__value-col">{ i18n.format_currency(summary.current) }</td>
                                <td class="table__value-col">{ i18n.format_currency(summary.days_30) }</td>
                                <td class={classes!("table__value-col", if summary.days_60 > dec!(0.00) { "col-severe" } else { "" })}>{ i18n.format_currency(summary.days_60) }</td>
                                <td class={classes!("table__value-col", if summary.days_90 > dec!(0.00) { "col-severe" } else { "" })}>{ i18n.format_currency(summary.days_90) }</td>
                                <td class={classes!("table__value-col", if summary.days_90_plus > dec!(0.00) { "col-severe" } else { "" })}>{ i18n.format_currency(summary.days_90_plus) }</td>
                                <td class="table__value-col">{ i18n.format_currency(summary.total) }</td>
                            </tr>
                            if is_expanded {
                                { for summary.invoices.iter().map(|invoice| {
                                    html! {
                                        <tr class="sub-row">
                                            <td class="table__text-col" colspan="2">{ &invoice.invoice_number }</td>
                                            <td class="table__value-col">{ i18n.format_date(invoice.issue_date) }</td>
                                            <td class="table__value-col">{ i18n.format_date(invoice.due_date) }</td>
                                            <td class="table__value-col" colspan="2">{ i18n.format_currency(invoice.amount_remaining) }</td>
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
