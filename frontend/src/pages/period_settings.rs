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
use crate::api::Api;
use crate::components::layout::Layout;
use crate::contexts::auth_context::use_user_context;
use crate::contexts::org_context::{OrgAction, OrgContextHandle};
use crate::router::Route;
use chrono::NaiveDate;
use serde_json::json;
use web_sys::{Event, HtmlInputElement};
use yew::{function_component, html, use_context, use_state, Callback, Html, TargetCast};
use yew_router::prelude::use_navigator;

#[function_component(PeriodSettings)]
pub fn period_settings() -> Html {
    let user_ctx = use_user_context();
    let org_ctx = use_context::<OrgContextHandle>().expect("OrgContext not found");

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
            <h1>{ "Accounting Period Settings" }</h1>
            <div class="settings-card">
                <p>{ "Prevent changes to transactions on or before this date:" }</p>
                <div class="input-group">
                        <input
                            type="date"
                            class="form-control"
                            value={local_date.map(|d| d.to_string()).unwrap_or_default()}
                            onchange={on_date_input}
                        />
                        <button
                            class="button-primary"
                            onclick={on_save_lock}
                        >
                            { "Update Lock Date" }
                        </button>
                    </div>

                    <p class="text-muted">
                        { "Current Lock: " }
                        <strong>
                            { org_ctx.locked_until.map(|d| d.format("%d %b %Y").to_string()).unwrap_or_else(|| "None".to_string()) }
                        </strong>
                    </p>
            </div>
        </Layout>
    }
}
