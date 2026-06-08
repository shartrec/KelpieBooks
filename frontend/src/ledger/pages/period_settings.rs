/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use chrono::NaiveDate;
use serde_json::json;
use web_sys::{
    Event,
    HtmlInputElement,
};
use yew::{
    function_component,
    html,
    use_context,
    use_state,
    Callback,
    Html,
    TargetCast,
};
use yew_router::prelude::use_navigator;

use crate::{
    api::Api,
    components::layout::Layout,
    contexts::{
        auth_context::use_user_context,
        locale_context::use_locale,
        org_context::{
            OrgAction,
            OrgContextHandle,
        },
    },
    router::Route,
};

#[function_component(PeriodSettings)]
pub fn period_settings() -> Html {
    let user_ctx = use_user_context();
    let org_ctx = use_context::<OrgContextHandle>().expect("OrgContext not found");
    let i18n = use_locale();

    let navigator = use_navigator().unwrap();
    // Local state for the input field so we don't spam the global context
    // or the API until the user clicks "Update"
    let local_date = use_state(|| org_ctx.locked_until);

    // 1. Handle local input changes
    let on_date_input = {
        let local_date = local_date.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let val = input.value();
            if let Ok(date) = NaiveDate::parse_from_str(&val, "%Y-%m-%d") {
                local_date.set(Some(date));
            } else if val.is_empty() {
                local_date.set(None);
            }
        })
    };

    let navigator = navigator.clone();
    // 2. Handle persistent save
    let on_save_lock = {
        let org_ctx = org_ctx.clone();
        let local_date = *local_date;
        let navigator = navigator.clone();
        let user_ctx = user_ctx.clone();

        Callback::from(move |_| {
            let org_ctx = org_ctx.clone();
            let navigator = navigator.clone();
            let user_ctx = user_ctx.clone();
            wasm_bindgen_futures::spawn_local(async move {
                // Construct the update request
                let res = Api::put(
                    "/api/organization/lock",
                    &json!({ "locked_until": local_date }),
                    user_ctx,
                    navigator.clone(),
                )
                .await;

                if res.is_ok() {
                    // Update global state so the rest of the app reacts
                    org_ctx.dispatch(OrgAction::UpdateLockDate(local_date));
                    navigator.push(&Route::Dashboard);
                    // Optional: Add a "Saved!" notification logic here
                }
            });
        })
    };

    html! {
        <Layout>
            <h1>{ i18n.t("period-settings-title") }</h1>
            <div class="settings-card">
                <p>{ i18n.t("period-settings-description") }</p>
                <div class="input-group">
                        <input
                            type="date"
                            class="form-control"
                            value={local_date.map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or_default()}
                            onchange={on_date_input}
                        />
                        <button
                            class="button-primary"
                            onclick={on_save_lock}
                        >
                            { i18n.t("period-settings-update-button") }
                        </button>
                    </div>

                    <p class="text-muted">
                        { i18n.t("period-settings-current-lock") }
                        <strong>
                            { org_ctx.locked_until.map(|d|{ i18n.format_date(d) }).unwrap_or_else(|| i18n.t("common-none")) }
                        </strong>
                    </p>
            </div>
        </Layout>
    }
}
