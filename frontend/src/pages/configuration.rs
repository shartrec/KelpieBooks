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
use gloo_net::http::Request;
use shared_core::models::{Account, SystemTag};
use std::collections::HashMap;
use serde_json::json;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use crate::contexts::org_context::{OrgAction, OrgContextHandle};
use crate::router::Route;

#[function_component(ConfigurationPage)]
pub fn configuration_page() -> Html {
    let accounts = use_state(Vec::new);
    let system_accounts = use_state(HashMap::new);
    let org_ctx = use_context::<OrgContextHandle>().expect("OrgContext not found");
    let strict_audit_mode = use_state(|| org_ctx.strict_audit_mode);
    let loading = use_state(|| true);
    let details_error = use_state(|| None::<String>);
    let details_success = use_state(|| false);
    let navigator = use_navigator().unwrap();

    {
        let accounts = accounts.clone();
        let system_accounts = system_accounts.clone();
        let loading = loading.clone();
        let details_error = details_error.clone();

        use_effect_with((), move |_| {
            let accounts = accounts.clone();
            let system_accounts = system_accounts.clone();
            let loading = loading.clone();
            let details_error = details_error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                let accounts_req = Request::get("/api/accounts").send();
                let system_accounts_req = Request::get("/api/configurations/system-accounts").send();
                let org_req = Request::get("/api/organization").send();

                match futures::join!(accounts_req, system_accounts_req, org_req) {
                    (Ok(acc_resp), Ok(sys_resp), Ok(org_resp)) => {
                        if acc_resp.ok() && sys_resp.ok() && org_resp.ok() {
                            let acc_data = acc_resp.json::<Vec<Account>>().await;
                            let sys_data = sys_resp.json::<HashMap<SystemTag, Uuid>>().await;

                            match (acc_data, sys_data) {
                                (Ok(acc), Ok(sys)) => {
                                    accounts.set(acc);
                                    system_accounts.set(sys);
                                    details_error.set(None);
                                }
                                _ => details_error.set(Some("Failed to parse data".to_string())),
                            }
                        } else {
                            details_error.set(Some("Failed to fetch data".to_string()));
                        }
                    }
                    _ => details_error.set(Some("Network error".to_string())),
                }
                loading.set(false);
            });
            || ()
        });
    }

    let on_save = {
        let system_accounts = system_accounts.clone();
        let org_ctx = org_ctx.clone();
        let strict_audit_mode = strict_audit_mode.clone();
        let error_state = details_error.clone();
        let success_state = details_success.clone();
        let org_id = org_ctx.id;
        let navigator = navigator.clone();


        Callback::from(move |_| {
            let system_accounts = system_accounts.clone();
            let org_ctx = org_ctx.clone();
            let strict_audit_mode = strict_audit_mode.clone();
            let error_state = error_state.clone();
            let success_state = success_state.clone();
            let navigator = navigator.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let mut errors = vec![];

                // Save system accounts
                let sys_resp = Request::post("/api/configurations/system-accounts")
                    .json(&*system_accounts)
                    .unwrap()
                    .send()
                    .await;

                match sys_resp {
                    Ok(r) if r.ok() => {
                        if let Ok(sys) = r.json::<HashMap<SystemTag, Uuid>>().await {
                            system_accounts.set(sys);
                        } else {
                            errors.push("Failed to parse system accounts response.".to_string());
                        }
                    }
                    Ok(r) => {
                        errors.push(format!("Error saving system accounts: {}", r.status()));
                    }
                    Err(e) => {
                        errors.push(format!("Network error saving system accounts: {}", e));
                    }
                }

                // Construct the update request
                let res = Request::put(&format!("/api/organizations/{}/audit_mode", org_id))
                    .json(&json!({ "strict_audit_mode": *strict_audit_mode }))
                    .unwrap()
                    .send()
                    .await;

                if res.is_ok() {
                    // Update global state so the rest of the app reacts
                    org_ctx.dispatch(OrgAction::UpdateAuditMode(*strict_audit_mode));
                    navigator.push(&Route::Dashboard);
                    // Optional: Add a "Saved!" notification logic here
                }

                if errors.is_empty() {
                    success_state.set(true);
                    error_state.set(None);
                } else {
                    success_state.set(false);
                    error_state.set(Some(errors.join("\n")));
                }
            });
        })
    };

    let on_select_change = {
        let system_accounts = system_accounts.clone();
        Callback::from(move |(tag, id_str): (SystemTag, String)| {
            if let Ok(id) = id_str.parse::<Uuid>() {
                let mut new_map = (*system_accounts).clone();
                new_map.insert(tag, id);
                system_accounts.set(new_map);
            }
        })
    };

    let on_audit_change = {
        let strict_audit_mode = strict_audit_mode.clone();
        Callback::from(move |e: Event| {
            let target: web_sys::HtmlInputElement = e.target_unchecked_into();
            strict_audit_mode.set(target.checked());
        })
    };

    html! {
        <Layout>
            <div class="page">
                <h3>{ "Configuration" }</h3>
                if *loading {
                    <p>{ "Loading..." }</p>
                } else {
                    <div class="data-form">
                        <h4 class="data-form__full-width">{ "Organization Settings" }</h4>
                        <label for="strict_audit_mode">{ "Strict Audit Mode" }</label>
                        <input
                            type="checkbox"
                            id="strict_audit_mode"
                            checked={*strict_audit_mode}
                            onchange={on_audit_change}
                        />
                        <small class="data-form__full-width">{ "When enabled, forbids editing and deletion of Journal Entries for closed periods." }</small>
                        <hr class="data-form__full-width"/>

                        <h4>{ "System Accounts" }</h4>
                        <p> { "Map system-critical accounts to the correct accounts in your chart of accounts." } </p>
                        { for SystemTag::iterator().map(|tag| {
                            let selected_account_id = system_accounts.get(&tag).map(|id| id.to_string());
                            html! {
                                <>
                                    <label for={format!("select-{:?}", tag)}>{ format!("{}", tag.display_name()) }</label>
                                    <select
                                        id={format!("select-{:?}", tag)}
                                        onchange={
                                            let on_select_change = on_select_change.clone();
                                            let tag = tag.clone();
                                            Callback::from(move |e: Event| {
                                                let target: web_sys::HtmlSelectElement = e.target_unchecked_into();
                                                on_select_change.emit((tag, target.value()));
                                            })
                                        }
                                        value={selected_account_id}
                                    >
                                        <option value="" disabled=true>{ "Select Account" }</option>
                                        { for accounts.iter().map(|acc| html! {
                                            <option
                                                value={acc.id.to_string()}
                                                selected={system_accounts.get(&tag) == Some(&acc.id)}
                                            >
                                                { &acc.name }
                                            </option>
                                        })}
                                    </select>
                                </>
                            }
                        })}
                        <div class="data-form__actions">
                            <button onclick={on_save}>{ "Save Configuration" }</button>
                        </div>
                        if *details_success {
                            <div class="message message__success">{"Configuration saved successfully!"}</div>
                        }
                        if let Some(err) = (*details_error).clone() {
                            <div class="message message__error">{err}</div>
                        }
                    </div>
                }
            </div>
        </Layout>
    }
}
