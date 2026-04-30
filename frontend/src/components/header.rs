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

use crate::auth::UserContextHandle;
use crate::contexts::report_context::ReportContext;
use crate::Route;
use gloo_net::http::Request;
use yew::prelude::*;
use yew_router::prelude::*;
use chrono::NaiveDate;

#[function_component(Header)]
pub fn header() -> Html {
    let user_ctx = use_context::<UserContextHandle>();
    let report_ctx = use_context::<ReportContext>();
    let navigator = use_navigator().unwrap();
    let dropdown_open = use_state(|| false);

    let on_logout_click = {
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        Callback::from(move |_| {
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Request::post("/api/auth/logout").send().await;
                if let Ok(_) = resp {
                    if let Some(handle) = user_ctx {
                        handle.dispatch(None);
                    }
                    navigator.push(&Route::Login);
                }
            });
        })
    };

    let toggle_dropdown = {
        let dropdown_open = dropdown_open.clone();
        Callback::from(move |_| {
            dropdown_open.set(!*dropdown_open);
        })
    };

    let on_start_change = {
        let report_ctx = report_ctx.clone();
        Callback::from(move |e: Event| {
            if let Some(ctx) = &report_ctx {
                let target: web_sys::HtmlInputElement = e.target_unchecked_into();
                if let Ok(new_date) = NaiveDate::parse_from_str(&target.value(), "%Y-%m-%d") {
                    let mut current = ctx.date_range.clone();
                    current.start_date = new_date;
                    ctx.dispatch(crate::contexts::report_context::ReportAction::SetDateRange(current));
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
                    ctx.dispatch(crate::contexts::report_context::ReportAction::SetDateRange(current));
                }
            }
        })
    };

    let on_export_click = {
        let report_ctx = report_ctx.clone();
        Callback::from(move |_| {
            if let Some(ctx) = &report_ctx {
                if let Some(on_export) = &ctx.on_export {
                    on_export.emit(());
                }
            }
        })
    };

    let user_display = if let Some(handle) = user_ctx {
        if let Some(user) = &handle.user {
            let name_to_display = user.display_name.as_ref().unwrap_or(&user.full_name);
            html! { <span class="username">{ name_to_display }</span> }
        } else {
            html! { <span class="username">{ "Loading..." }</span> }
        }
    } else {
        html! { <span></span> }
    };

    let route = use_route::<Route>().unwrap_or(Route::Home);
    let is_report_route = matches!(route, Route::Ledger | Route::TrialBalance | Route::ProfitLoss | Route::BalanceSheet | Route::AccountLedger { .. });

    html! {
        <header class="header">
            <div class="header-content">
                <div class="header-left">
                    if is_report_route {
                        if let Some(ctx) = report_ctx {
                            <div class="header-action-bar">
                                <div class="date-range-selector">
                                    <label>{ "From: " }</label>
                                    <input type="date" value={ctx.date_range.start_date.to_string()} onchange={on_start_change} />
                                    <label>{ "To: " }</label>
                                    <input type="date" value={ctx.date_range.end_date.to_string()} onchange={on_end_change} />
                                </div>
                                if ctx.on_export.is_some() {
                                    <button class="icon-button" onclick={on_export_click} title="Export to CSV">
                                        <img src="/images/download.svg" alt="Export" />
                                    </button>
                                }
                            </div>
                        }
                    }
                </div>
                <div class="user-menu">
                    <button onclick={toggle_dropdown} class="user-menu-trigger">
                        { user_display }
                        <img src="/images/chevron-down.svg" alt="Toggle menu" />
                    </button>
                    if *dropdown_open {
                        <div class="user-menu-dropdown">
                            <Link<Route> to={Route::Profile} classes="dropdown-item">
                                <img src="/images/user.svg" alt="Profile" />
                                <span>{ "Edit Profile" }</span>
                            </Link<Route>>
                            <button onclick={on_logout_click} class="dropdown-item">
                                <img src="/images/logout.svg" alt="Logout" />
                                <span>{ "Logout" }</span>
                            </button>
                        </div>
                    }
                </div>
            </div>
        </header>
    }
}
