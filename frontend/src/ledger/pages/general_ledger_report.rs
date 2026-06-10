/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use fluent::fluent_args;
use shared_core::ledger::dtos::general_ledger_line::GeneralLedgerLine;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    api::Api,
    core::components::{
        layout::Layout,
        report_options::ReportOptions,
    },
    contexts::{
        auth_context::use_user_context,
        locale_context::use_locale,
        report_context::{
            use_report_context,
            ReportAction,
        },
    },
    router::Route,
};

#[function_component(GeneralLedgerReportPage)]
pub fn general_ledger_report_page() -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();
    let report_ctx = use_report_context();
    let report_data = use_state(|| Vec::<GeneralLedgerLine>::new());
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let report_ctx = report_ctx.clone();
        use_effect_with((), move |_| {
            let start_date = report_ctx.date_range.start_date;
            let end_date = report_ctx.date_range.end_date;
            let selected_accounts = report_ctx.selected_accounts.clone();
            let min_amount = report_ctx.min_amount;

            let mut url_params = format!("start={}&end={}", start_date, end_date);
            if !selected_accounts.is_empty() {
                let account_ids = selected_accounts
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(",");
                url_params.push_str(&format!("&accounts={}", account_ids));
            }
            if let Some(amount) = min_amount {
                url_params.push_str(&format!("&min_amount={}", amount));
            }

            let csv_url = format!("/api/reports/general-ledger/export/csv?{}", url_params);
            report_ctx.dispatch(ReportAction::SetOnExportCsv(Some(Callback::from(
                move |_| {
                    web_sys::window()
                        .unwrap()
                        .location()
                        .set_href(&csv_url)
                        .unwrap();
                },
            ))));

            let pdf_url = format!("/api/reports/general-ledger/export/pdf?{}", url_params);
            report_ctx.dispatch(ReportAction::SetOnExportTypst(Some(Callback::from(
                move |_| {
                    web_sys::window()
                        .unwrap()
                        .location()
                        .set_href(&pdf_url)
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
        let report_data = report_data.clone();
        let loading = loading.clone();
        let error = error.clone();
        let start_date = report_ctx.date_range.start_date;
        let end_date = report_ctx.date_range.end_date;
        let selected_accounts = report_ctx.selected_accounts.clone();
        let min_amount = report_ctx.min_amount;
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();

        use_effect_with(
            (start_date, end_date, selected_accounts.clone(), min_amount),
            move |(start, end, accounts, min_amount)| {
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
                    let mut url =
                        format!("/api/reports/general-ledger?start={}&end={}", start, end);
                    if !accounts.is_empty() {
                        let account_ids = accounts
                            .iter()
                            .map(|id| id.to_string())
                            .collect::<Vec<String>>()
                            .join(",");
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
                                    Err(e) => error.set(Some(i18n.t_args(
                                        "general-ledger-error-parse",
                                        &fluent_args!["error" => e.to_string()],
                                    ))),
                                }
                            } else {
                                error.set(Some(i18n.t_args(
                                    "general-ledger-error-fetch",
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
            },
        );
    }

    let grouped_data = use_memo(report_data.clone(), |data| {
        let mut grouped = std::collections::BTreeMap::new();
        for line in data.iter() {
            grouped
                .entry(line.account_name.clone())
                .or_insert_with(Vec::new)
                .push(line.clone());
        }
        grouped
    });

    let i18n = use_locale();

    html! {
        <Layout>
            <div class="report-page">
                <div class="report-header">
                    <h3>{ i18n.t("general-ledger-title") }</h3>
                    <ReportOptions show_start_date={true} show_end_date={true} show_advanced_filters={true} />
                </div>
                if *loading {
                    <p>{ i18n.t("common-loading") }</p>
                } else if let Some(err) = &*error {
                    <div class="message__error">{ err }</div>
                } else {
                    <table class="report-table">
                        <thead>
                            <tr>
                                <th>{ i18n.t("common-date") }</th>
                                <th>{ i18n.t("common-description") }</th>
                                <th class="text-amount">{ i18n.t("common-debit") }</th>
                                <th class="text-amount">{ i18n.t("common-credit") }</th>
                                <th class="text-amount">{ i18n.t("common-balance") }</th>
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
                                                    { i18n.format_date(line.date) }
                                                </Link<Route>>
                                            </td>
                                            <td>{ line.description.clone().unwrap_or_default() }</td>
                                            <td class="text-amount">{ i18n.format_currency(line.debit) }</td>
                                            <td class="text-amount">{ i18n.format_currency(line.credit) }</td>
                                            <td class="text-amount">{ i18n.format_currency(line.balance) }</td>
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
