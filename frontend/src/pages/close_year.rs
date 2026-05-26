/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::api::Api;
use crate::components::layout::Layout;
use crate::contexts::auth_context::use_user_context;
use chrono::{Datelike, NaiveDate, Utc};
use fluent::fluent_args;
use shared_core::i18n::{t, t_args};
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use crate::contexts::locale_context::use_locale;

#[function_component(CloseYearPage)]
pub fn close_year_page() -> Html {
    let user_ctx = use_user_context();
    let navigator = use_navigator().unwrap();
    let year_end_date = use_state(|| {
        let today = Utc::now().date_naive();
        NaiveDate::from_ymd_opt(today.year(), 12, 31).unwrap_or(today)
    });
    let show_confirmation = use_state(|| false);
    let loading = use_state(|| false);
    let error = use_state(|| None::<String>);
    let success = use_state(|| None::<String>);
    let i18n = use_locale();

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
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();

        Callback::from(move |_| {
            let year_end_str = year_end_date.format("%Y-%m-%d").to_string();
            let loading = loading.clone();
            let error = error.clone();
            let success = success.clone();
            let show_confirmation = show_confirmation.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                show_confirmation.set(false);
                error.set(None);
                success.set(None);

                let url = format!("/api/period-end/close-year?year_end={}", year_end_str);
                let response = Api::post(&url, &(), user_ctx, navigator).await;

                match response {
                    Ok(resp) if resp.ok() => {
                        success.set(Some(t("close-year-success-message")));
                    }
                    Ok(resp) => {
                        let status = resp.status();
                        let err_msg = resp.text().await.unwrap_or_default();
                        error.set(Some(t_args(
                            "close-year-error",
                            &fluent_args!["status" => status, "error" => err_msg],
                        )));
                    }
                    Err(e) => {
                        error.set(Some(t_args(
                            "common-network-error",
                            &fluent_args!["error" => e.to_string()],
                        )));
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
                <h3>{ t("close-year-title") }</h3>
                <p class="page-description">
                    { t("close-year-description") }
                </p>

                <div class="data-form">
                    <div class="form-group">
                        <label for="year-end-date">{ t("close-year-select-date-label") }</label>
                        <input
                            id="year-end-date"
                            type="date"
                            value={year_end_date.format("%Y-%m-%d").to_string()}
                            onchange={on_date_change}
                        />
                    </div>
                    <div class="form-actions">
                        <button class="button button-danger" onclick={on_initiate_close} disabled={*loading}>
                            { t("close-year-button") }
                        </button>
                    </div>
                </div>

                if *loading {
                    <p>{ t("close-year-loading-message") }</p>
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
                            <h4>{ t("close-year-confirm-title") }</h4>
                            <p>
                                { t_args("close-year-confirm-message", &fluent_args!["date" =>{ i18n.format_date(*year_end_date) }]) }
                            </p>
                            <div class="modal-actions">
                                <button class="button" onclick={on_cancel_close}>{ t("common-cancel") }</button>
                                <button class="button button-danger" onclick={on_confirm_close}>{ t("close-year-confirm-button") }</button>
                            </div>
                        </div>
                    </div>
                }
            </div>
        </Layout>
    }
}
