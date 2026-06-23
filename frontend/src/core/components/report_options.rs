/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::NaiveDate;
use fluent::fluent_args;
use rust_decimal::{
    dec,
    Decimal,
};
use shared_core::ledger::models::{
    account::Account,
    account_category::AccountCategory,
};
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{
    api::Api,
    contexts::{
        auth_context::use_user_context,
        locale_context::use_locale,
        report_context::{
            ReportAction,
            ReportContext,
        },
    },
    core::components::currency_input::DecimalInput,
};

#[derive(Properties, PartialEq)]
pub struct ReportOptionsProps {
    #[prop_or_default]
    pub show_start_date: bool,
    #[prop_or_default]
    pub show_end_date: bool,
    #[prop_or_default]
    pub show_advanced_filters: bool,
}

#[function_component(ReportOptions)]
pub fn report_options(props: &ReportOptionsProps) -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();
    let report_ctx = use_context::<ReportContext>();
    let accounts = use_state(Vec::new);

    {
        let accounts = accounts.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        let show_advanced = props.show_advanced_filters;
        use_effect_with((), move |_| {
            if show_advanced {
                let accounts = accounts.clone();
                let user_ctx = user_ctx.clone();
                let navigator = navigator.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok(response) = Api::get("/api/accounts", user_ctx, navigator).await {
                        if let Ok(data) = response.json::<Vec<Account>>().await {
                            accounts.set(
                                data.into_iter()
                                    .filter(|acc| {
                                        acc.category == AccountCategory::Expense
                                            || acc.category == AccountCategory::Revenue
                                    })
                                    .collect(),
                            );
                        }
                    }
                });
            }
            || ()
        });
    }

    let on_start_change = {
        let report_ctx = report_ctx.clone();
        Callback::from(move |e: Event| {
            if let Some(ctx) = &report_ctx {
                let target: web_sys::HtmlInputElement = e.target_unchecked_into();
                if let Ok(new_date) = NaiveDate::parse_from_str(&target.value(), "%Y-%m-%d") {
                    let mut current = ctx.date_range.clone();
                    current.start_date = new_date;
                    ctx.dispatch(ReportAction::SetDateRange(current));
                }
            }
        })
    };

    let on_end_change = {
        let report_ctx = report_ctx.clone();
        Callback::from(move |e: Event| {
            if let Some(ctx) = &report_ctx {
                let target: web_sys::HtmlInputElement = e.target_unchecked_into();
                if let Ok(new_date) = NaiveDate::parse_from_str(&target.value(), "%Y-%m-%d") {
                    let mut current = ctx.date_range.clone();
                    current.end_date = new_date;
                    ctx.dispatch(ReportAction::SetDateRange(current));
                }
            }
        })
    };

    let on_min_amount_change = {
        let report_ctx = report_ctx.clone();
        Callback::from(move |amount: Decimal| {
            if let Some(ctx) = &report_ctx {
                ctx.dispatch(ReportAction::SetMinAmount(Some(amount)));
            }
        })
    };

    let on_export_csv_click = {
        let report_ctx = report_ctx.clone();
        Callback::from(move |_| {
            if let Some(ctx) = &report_ctx {
                if let Some(on_export_csv) = &ctx.on_export_csv {
                    on_export_csv.emit(());
                }
            }
        })
    };

    let on_export_pdf_click = {
        let report_ctx = report_ctx.clone();
        Callback::from(move |_| {
            if let Some(ctx) = &report_ctx {
                if let Some(on_export_pdf) = &ctx.on_export_pdf {
                    on_export_pdf.emit(());
                }
            }
        })
    };

    if let Some(ctx) = report_ctx {
        let accounts = accounts.clone();
        html! {
            <div class="report__options">
                <div class="report__action-bar">
                    <div class="report__date-range-selector">
                        if props.show_start_date {
                            <label>{ i18n.t("report-options-from-label") }</label>
                            <input type="date" value={ctx.date_range.start_date.format("%Y-%m-%d").to_string()} onchange={on_start_change} />
                        }
                        if props.show_end_date {
                            <label>{ i18n.t("report-options-to-label") }</label>
                            <input type="date" value={ctx.date_range.end_date.format("%Y-%m-%d").to_string()} onchange={on_end_change} />
                        }
                    </div>
                    <div class="report__export-buttons">
                        if ctx.on_export_csv.is_some() {
                            <button class="icon-button" onclick={on_export_csv_click} title={i18n.t("report-options-export-csv-tooltip")}>
                                <img src="/images/download.svg" alt={i18n.t("report-options-export-csv-tooltip")} />
                            </button>
                        }
                        if ctx.on_export_pdf.is_some() {
                            <button class="icon-button" onclick={on_export_pdf_click} title={i18n.t("report-options-export-pdf-tooltip")}>
                                <img src="/images/export-pdf.svg" alt={i18n.t("report-options-export-pdf-tooltip")} />
                            </button>
                        }
                    </div>
                </div>
                if props.show_advanced_filters {
                    <div class="report__advanced-filters">
                        <div class="report__filter-group">
                            <label>{ i18n.t("report-options-accounts-label") }</label>
                            <AccountFilter accounts={(*accounts).clone()}/>
                        </div>
                        <div class="report__filter-group">
                            <label>{ i18n.t("report-options-min-amount-label") }</label>
                            <DecimalInput
                                value={ctx.min_amount.unwrap_or(dec!(0.00))}
                                on_change={on_min_amount_change}
                                placeholder={i18n.t("journal-entry-currency-placeholder")}
                            />
                        </div>
                    </div>
                }
            </div>
        }
    } else {
        html! {}
    }
}

#[derive(Properties, PartialEq)]
pub struct AccountFilterProps {
    pub accounts: Vec<Account>,
}

#[function_component(AccountFilter)]
pub fn account_filter(props: &AccountFilterProps) -> Html {
    let i18n = use_locale();
    let report_ctx = use_context::<ReportContext>();
    let is_open = use_state(|| false);

    let on_toggle_dropdown = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };

    // Callback to update the context when a checkbox is clicked
    let on_account_select = {
        let ctx = report_ctx.clone().unwrap();
        Callback::from(move |id: Uuid| {
            ctx.dispatch(ReportAction::ToggleAccount(id));
        })
    };
    if let Some(ctx) = report_ctx {
        html! {
            <div class="report__filter-group">
                <span class="filter-label">{ i18n.t("report-options-accounts-label") }</span>
                <div class="filter-trigger" onclick={on_toggle_dropdown}>
                    { if ctx.selected_accounts.is_empty() {
                        i18n.t("report-options-all-accounts")
                    } else {
                        i18n.t_args("report-options-selected-accounts", &fluent_args!["count" => ctx.selected_accounts.len()])
                    }}
                </div>

                if *is_open {
                    <div class="filter-popover">
                        { for props.accounts.iter().map(|acc| {
                            let id = acc.id;
                            let is_selected = ctx.selected_accounts.contains(&id);
                            let on_click = on_account_select.clone();

                            html! {
                                <label class="filter-item">
                                    <input
                                        type="checkbox"
                                        checked={is_selected}
                                        onclick={move |_| on_click.emit(id)}
                                    />
                                    <span>{ format!("{} - {}", acc.code, acc.name) }</span>
                                </label>
                            }
                        })}
                    </div>
                }
            </div>
        }
    } else {
        html! {}
    }
}
