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


use yew::prelude::*;
use chrono::NaiveDate;
use crate::contexts::report_context::{ReportContext, ReportAction};
use uuid::Uuid;
use crate::api::Api;
use crate::contexts::auth_context::use_user_context;
use yew_router::prelude::use_navigator;
use shared_core::models::account::Account;
use shared_core::models::account_category::AccountCategory;
use crate::components::currency_input::CurrencyInput;

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
                            accounts.set(data.into_iter().filter(|acc| {
                                acc.category == AccountCategory::Expense || acc.category == AccountCategory::Revenue
                            }).collect());
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
        Callback::from(move |cents: i64| {
            if let Some(ctx) = &report_ctx {
                ctx.dispatch(ReportAction::SetMinAmount(Some(cents)));
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
                            <label>{ "From: " }</label>
                            <input type="date" value={ctx.date_range.start_date.to_string()} onchange={on_start_change} />
                        }
                        if props.show_end_date {
                            <label>{ "To: " }</label>
                            <input type="date" value={ctx.date_range.end_date.to_string()} onchange={on_end_change} />
                        }
                    </div>
                    <div class="report__export-buttons">
                        if ctx.on_export_csv.is_some() {
                            <button class="icon-button" onclick={on_export_csv_click} title="Export to CSV">
                                <img src="/images/download.svg" alt="Export CSV" />
                            </button>
                        }
                        if ctx.on_export_pdf.is_some() {
                            <button class="icon-button" onclick={on_export_pdf_click} title="Export to PDF">
                                <img src="/images/export-pdf.svg" alt="Export PDF" />
                            </button>
                        }
                    </div>
                </div>
                if props.show_advanced_filters {
                    <div class="report__advanced-filters">
                        <div class="report__filter-group">
                            <label>{ "Accounts:" }</label>
                            <AccountFilter accounts={(*accounts).clone()}/>
                        </div>
                        <div class="report__filter-group">
                            <label>{ "Min Amount:" }</label>
                            <CurrencyInput
                                value={ctx.min_amount.unwrap_or(0)}
                                on_change={on_min_amount_change}
                                placeholder="0.00"
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
                <span class="filter-label">{ "Account:" }</span>
                <div class="filter-trigger" onclick={on_toggle_dropdown}>
                    { if ctx.selected_accounts.is_empty() {
                        "All Accounts".to_string()
                    } else {
                        format!("{} Selected", ctx.selected_accounts.len())
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