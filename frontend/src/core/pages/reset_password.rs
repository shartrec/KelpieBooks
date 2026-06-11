/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use fluent::fluent_args;
use gloo_net::http::Request;
use serde::Deserialize;
use shared_core::core::requests::auth::ResetPasswordSubmit;
use yew::{
    function_component,
    html,
    prelude::*,
};
use yew_router::{
    hooks::use_location,
    prelude::*,
};

use crate::{
    contexts::locale_context::use_locale,
    router::Route,
};

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ResetPasswordQuery {
    pub id: i32,
    pub token: String,
}

#[function_component(ResetPasswordPage)]
pub fn reset_password_page() -> Html {
    let i18n = use_locale();
    let location = use_location().unwrap();
    let query = location.query::<ResetPasswordQuery>().unwrap();
    let token = query.token;
    let token_id = query.id;

    let success_state = use_state(|| false);
    let error_state = use_state(|| None::<String>);

    let on_submit = {
        let success_state = success_state.clone();
        let error_state = error_state.clone();
        let token = token.clone();
        let token_id = token_id.clone();
        let i18n = i18n.clone();

        Callback::from(move |new_password: String| {
            let success_state = success_state.clone();
            let error_state = error_state.clone();
            let token = token.clone();
            let i18n = i18n.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let request = ResetPasswordSubmit {
                    id: token_id,
                    raw_token: token,
                    new_password,
                };
                let resp = Request::post("/api/auth/reset-password")
                    .json(&request)
                    .unwrap()
                    .send()
                    .await;

                match resp {
                    Ok(r) if r.ok() => {
                        success_state.set(true);
                    }
                    Ok(r) => {
                        error_state.set(Some(i18n.t_args(
                            "reset-password-error-server",
                            &fluent_args!["status" => r.status()],
                        )));
                    }
                    Err(e) => {
                        error_state.set(Some(i18n.t_args(
                            "common-network-error",
                            &fluent_args!["error" => e.to_string()],
                        )));
                    }
                }
            });
        })
    };

    html! {
        <div class="login-wrapper">
            <div class="login-card">
                <div class="login-brand">
                    <img src="/images/kelpiedog_120x120_transparent.png" alt={i18n.t("login-logo-alt-text")} class="login-logo" />
                    <h1>{ i18n.t("branding-app-name") }</h1>
                    <p class="subtitle">{ i18n.t("reset-password-subtitle") }</p>
                </div>
                if *success_state {
                    <div class="message message__success">
                        {i18n.t("reset-password-success-message")}
                        <p><Link<Route> to={Route::Login}>{ i18n.t("reset-password-back-to-login") }</Link<Route>></p>
                    </div>
                } else {
                    <ResetPasswordForm on_submit={on_submit} error={(*error_state).clone()} />
                }
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ResetPasswordFormProps {
    pub on_submit: Callback<String>,
    pub error: Option<String>,
}

#[function_component(ResetPasswordForm)]
pub fn reset_password_form(props: &ResetPasswordFormProps) -> Html {
    let i18n = use_locale();
    let new_password = use_state(String::new);
    let confirm_password = use_state(String::new);
    let show_new_password = use_state(|| false);
    let show_confirm_password = use_state(|| false);

    let on_new_password_input = {
        let state = new_password.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            state.set(input.value());
        })
    };

    let on_confirm_password_input = {
        let state = confirm_password.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            state.set(input.value());
        })
    };

    let on_submit = {
        let on_submit = props.on_submit.clone();
        let new_password = new_password.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            on_submit.emit((*new_password).clone());
        })
    };

    let toggle_password_visibility = |state: UseStateHandle<bool>| {
        Callback::from(move |_| {
            state.set(!*state);
        })
    };

    let passwords_match = !new_password.is_empty() && *new_password == *confirm_password;

    html! {
        <form onsubmit={on_submit} class="auth-form">
            <div class="input-field-group">
                <label>{i18n.t("reset-password-new-password-label")}</label>
                <div class="password-input-wrapper">
                    <input type={if *show_new_password { "text" } else { "password" }} value={(*new_password).clone()} oninput={on_new_password_input} required=true />
                    <button type="button" class="icon-button" onclick={toggle_password_visibility(show_new_password.clone())}>
                        { if *show_new_password { "⊘" } else { "👁" } }
                    </button>
                </div>
            </div>
            <div class="input-field-group">
                <label>{i18n.t("reset-password-confirm-password-label")}</label>
                <div class="password-input-wrapper">
                    <input
                        type={if *show_confirm_password { "text" } else { "password" }}
                        value={(*confirm_password).clone()}
                        oninput={on_confirm_password_input}
                        required=true
                        class={if !confirm_password.is_empty() { if passwords_match { "input-success" } else { "input-error" } } else { "" }}
                    />
                    <button type="button" class="icon-button" onclick={toggle_password_visibility(show_confirm_password.clone())}>
                        { if *show_confirm_password { "⊘" } else { "👁" } }
                    </button>
                </div>
            </div>
            <button type="submit" class="button-primary login-btn" disabled={!passwords_match}>
                { i18n.t("reset-password-submit-button") }
            </button>
            if let Some(err) = &props.error {
                <div class="error">{err}</div>
            }
        </form>
    }
}
