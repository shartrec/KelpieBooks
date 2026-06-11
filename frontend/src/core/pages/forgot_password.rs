/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use gloo_net::http::Request;
use shared_core::core::requests::auth::ForgotPasswordRequest;
use yew::{
    function_component,
    html,
    prelude::*,
};
use yew_router::prelude::Link;
use crate::{
    contexts::locale_context::use_locale,
    router::Route,
};

#[function_component(ForgotPasswordPage)]
pub fn forgot_password_page() -> Html {
    let i18n = use_locale();
    let success_state = use_state(|| false);

    let on_submit = {
        let success_state = success_state.clone();
        Callback::from(move |email: String| {
            let success_state = success_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let request = ForgotPasswordRequest { email };
                let resp = Request::post("/api/auth/forgot-password")
                    .json(&request)
                    .unwrap()
                    .send()
                    .await;

                if resp.is_ok() {
                    success_state.set(true);
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
                    <p class="subtitle">{ i18n.t("forgot-password-subtitle") }</p>
                </div>
                if *success_state {
                    <div class="message message__success">{i18n.t("forgot-password-success-message")}</div>
                } else {
                    <ForgotPasswordForm on_submit={on_submit} />
                }
                <div class="login-footer">
                    <p><Link<Route> to={Route::Login}>{ i18n.t("forgot-password-back-to-login") }</Link<Route>></p>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ForgotPasswordFormProps {
    pub on_submit: Callback<String>,
}

#[function_component(ForgotPasswordForm)]
pub fn forgot_password_form(props: &ForgotPasswordFormProps) -> Html {
    let i18n = use_locale();
    let email = use_state(String::new);

    let on_email_input = {
        let state = email.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            state.set(input.value());
        })
    };

    let on_submit = {
        let on_submit = props.on_submit.clone();
        let email = email.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            on_submit.emit((*email).clone());
        })
    };

    html! {
        <form onsubmit={on_submit} class="auth-form">
            <div class="input-field-group">
                <label>{i18n.t("forgot-password-email-label")}</label>
                <input type="email" value={(*email).clone()} oninput={on_email_input} required=true autocomplete="email" />
            </div>
            <button type="submit" class="button-primary login-btn">
                { i18n.t("forgot-password-submit-button") }
            </button>
        </form>
    }
}
