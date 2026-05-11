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
use chrono::{Datelike, NaiveDate, Utc};
use gloo_net::http::Request;
use yew::prelude::*;

#[function_component(CloseYearPage)]
pub fn close_year_page() -> Html {
    let year_end_date = use_state(|| {
        let today = Utc::now().date_naive();
        NaiveDate::from_ymd_opt(today.year(), 12, 31).unwrap_or(today)
    });
    let show_confirmation = use_state(|| false);
    let loading = use_state(|| false);
    let error = use_state(|| None::<String>);
    let success = use_state(|| None::<String>);

    let on_date_change = {
        let year_end_date = year_end_date.clone();
        Callback::from(move |e: Event| {
            let target: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(new_date) = NaiveDate::parse_from_str(&target.value(), "%Y-%m-%d") {
                year_end_date.set(new_date);
            }
        })
    };

    let on_initiate_close = {
        let show_confirmation = show_confirmation.clone();
        Callback::from(move |_| {
            show_confirmation.set(true);
        })
    };

    let on_confirm_close = {
        let year_end_date = year_end_date.clone();
        let loading = loading.clone();
        let error = error.clone();
        let success = success.clone();
        let show_confirmation = show_confirmation.clone();

        Callback::from(move |_| {
            let year_end_str = year_end_date.format("%Y-%m-%d").to_string();
            let loading = loading.clone();
            let error = error.clone();
            let success = success.clone();
            let show_confirmation = show_confirmation.clone();

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                show_confirmation.set(false);
                error.set(None);
                success.set(None);

                let url = format!("/api/period-end/close-year?year_end={}", year_end_str);
                let response = Request::post(&url).send().await;

                match response {
                    Ok(resp) if resp.ok() => {
                        success.set(Some("Financial year closed successfully.".to_string()));
                    }
                    Ok(resp) => {
                        let status = resp.status();
                        let err_msg = resp.text().await.unwrap_or_default();
                        error.set(Some(format!("Error {}: {}", status, err_msg)));
                    }
                    Err(e) => {
                        error.set(Some(format!("Network error: {}", e)));
                    }
                }
                loading.set(false);
            });
        })
    };

    let on_cancel_close = {
        let show_confirmation = show_confirmation.clone();
        Callback::from(move |_| {
            show_confirmation.set(false);
        })
    };

    html! {
        <Layout>
            <div class="page">
                <h3>{ "Close Financial Year" }</h3>
                <p class="page-description">
                    { "Closing the financial year is an irreversible process. It will summarize all revenue and expense accounts into Retained Earnings and lock all transactions on or before the selected date." }
                </p>

                <div class="data-form">
                    <div class="form-group">
                        <label for="year-end-date">{ "Select Year-End Date" }</label>
                        <input
                            id="year-end-date"
                            type="date"
                            value={year_end_date.format("%Y-%m-%d").to_string()}
                            onchange={on_date_change}
                        />
                    </div>
                    <div class="form-actions">
                        <button class="button button-danger" onclick={on_initiate_close} disabled={*loading}>
                            { "Close Financial Year" }
                        </button>
                    </div>
                </div>

                if *loading {
                    <p>{ "Closing year..." }</p>
                }

                if let Some(err) = &*error {
                    <div class="message message-error">{ err }</div>
                }

                if let Some(msg) = &*success {
                    <div class="message message-success">{ msg }</div>
                }

                if *show_confirmation {
                    <div class="modal-backdrop">
                        <div class="modal">
                            <h4>{ "Confirm Year-End Close" }</h4>
                            <p>
                                { "Are you sure you want to close the financial year ending on " }
                                <strong>{ year_end_date.format("%d %B %Y").to_string() }</strong>
                                { "? This action cannot be undone." }
                            </p>
                            <div class="modal-actions">
                                <button class="button" onclick={on_cancel_close}>{ "Cancel" }</button>
                                <button class="button button-danger" onclick={on_confirm_close}>{ "Yes, Close Year" }</button>
                            </div>
                        </div>
                    </div>
                }
            </div>
        </Layout>
    }
}
