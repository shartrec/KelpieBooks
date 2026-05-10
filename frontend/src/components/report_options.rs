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

#[derive(Properties, PartialEq)]
pub struct ReportOptionsProps {
    #[prop_or_default]
    pub show_start_date: bool,
    #[prop_or_default]
    pub show_end_date: bool,
}

#[function_component(ReportOptions)]
pub fn report_options(props: &ReportOptionsProps) -> Html {
    let report_ctx = use_context::<ReportContext>();

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
        html! {
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
        }
    } else {
        html! {}
    }
}
