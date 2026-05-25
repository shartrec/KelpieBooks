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
use crate::contexts::org_context::{OrgAction, OrgContextHandle, OrgState};
use crate::router::Route;
use fluent::fluent_args;
use shared_core::i18n::{t, t_args};
use shared_core::models::account::Account;
use shared_core::models::organization::Organization;
use shared_core::models::system_tag::SystemTag;
use shared_core::requests::configuration::UpdateConfigurationRequest;
use std::collections::HashMap;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[function_component(ConfigurationPage)]
pub fn configuration_page() -> Html {
    let user_ctx = use_user_context();
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
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        let org_ctx = org_ctx.clone();

        use_effect_with((), move |_| {
            let accounts = accounts.clone();
            let system_accounts = system_accounts.clone();
            let loading = loading.clone();
            let details_error = details_error.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            let org_ctx = org_ctx.clone();

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                let accounts_req =
                    Api::get("/api/accounts", user_ctx.clone(), navigator.clone()).await;
                let system_accounts_req = Api::get(
                    "/api/configurations/system-accounts",
                    user_ctx.clone(),
                    navigator.clone(),
                )
                .await;
                let org_req = Api::get("/api/organization", user_ctx, navigator).await;

                match (accounts_req, system_accounts_req, org_req) {
                    (Ok(acc_resp), Ok(sys_resp), Ok(org_resp)) => {
                        if acc_resp.ok() && sys_resp.ok() && org_resp.ok() {
                            let acc_data = acc_resp.json::<Vec<Account>>().await;
                            let sys_data = sys_resp.json::<HashMap<SystemTag, Uuid>>().await;
                            let org_data = org_resp.json::<Organization>().await;

                            match (acc_data, sys_data, org_data) {
                                (Ok(acc), Ok(sys), Ok(org)) => {
                                    accounts.set(acc);
                                    system_accounts.set(sys);
                                    org_ctx.dispatch(OrgAction::SetOrg(OrgState {
                                        id: org.id,
                                        name: org.name,
                                        strict_audit_mode: org.strict_audit_mode,
                                        locked_until: org.locked_until,
                                    }));
                                    details_error.set(None);
                                }
                                _ => details_error.set(Some(t("configuration-error-parse"))),
                            }
                        } else {
                            details_error.set(Some(t("configuration-error-fetch")));
                        }
                    }
                    _ => details_error.set(Some(t("common-network-error"))),
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
        let navigator = navigator.clone();
        let user_ctx = user_ctx.clone();

        Callback::from(move |_| {
            let system_accounts = (*system_accounts).clone();
            let org_ctx = org_ctx.clone();
            let strict_audit_mode = *strict_audit_mode;
            let error_state = error_state.clone();
            let success_state = success_state.clone();
            let navigator = navigator.clone();
            let user_ctx = user_ctx.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let req = UpdateConfigurationRequest {
                    system_accounts,
                    strict_audit_mode,
                };

                let resp = Api::put("/api/configurations", &req, user_ctx, navigator.clone()).await;

                match resp {
                    Ok(r) if r.ok() => {
                        org_ctx.dispatch(OrgAction::UpdateAuditMode(strict_audit_mode));
                        success_state.set(true);
                        error_state.set(None);
                        navigator.push(&Route::Dashboard);
                    }
                    Ok(r) => {
                        error_state.set(Some(t_args(
                            "configuration-error-save",
                            &fluent_args!["status" => r.status()],
                        )));
                    }
                    Err(e) => {
                        error_state.set(Some(t_args(
                            "common-network-error",
                            &fluent_args!["error" => e.to_string()],
                        )));
                    }
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
                <h3>{ t("configuration-title") }</h3>
                if *loading {
                    <p>{ t("common-loading") }</p>
                } else {
                    <div class="data-form">
                        <h4 class="data-form__full-width">{ t("configuration-org-settings-title") }</h4>
                        <label for="strict_audit_mode">{ t("configuration-strict-audit-label") }</label>
                        <input
                            type="checkbox"
                            id="strict_audit_mode"
                            checked={*strict_audit_mode}
                            onchange={on_audit_change}
                        />
                        <small class="data-form__full-width">{ t("configuration-strict-audit-description") }</small>
                        <hr class="data-form__full-width"/>

                        <h4>{ t("configuration-system-accounts-title") }</h4>
                        <p> { t("configuration-system-accounts-description") } </p>
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
                                        <option value="" disabled=true>{ t("configuration-select-account") }</option>
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
                            <button onclick={on_save}>{ t("configuration-save-button") }</button>
                        </div>
                        if *details_success {
                            <div class="message message__success">{t("configuration-save-success")}</div>
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
