/*
 * Copyright (c) 2025-2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use fluent::fluent_args;
use gloo_net::http::Request;
use shared_core::core::{
    dtos::user_detail::AuthUserDetail,
    requests::auth::LoginRequest,
};
use yew::{
    function_component,
    html,
    prelude::*,
};
use yew_router::{
    hooks::use_navigator,
    prelude::Link,
};

use crate::{
    contexts::{
        auth_context::UserContextHandle,
        locale_context::use_locale,
    },
    router::Route,
    services::web::detect_browser_locale,
};

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let i18n = use_locale();
    let error_state = use_state(|| None::<String>);
    let is_login_success = use_state(|| false);
    let navigator = use_navigator().unwrap();
    let user_ctx = use_context::<UserContextHandle>().expect("UserContext not found");

    {
        let navigator = navigator.clone();
        use_effect_with(*is_login_success, move |success| {
            if *success {
                navigator.push(&Route::Dashboard);
            }
            || ()
        });
    }

    let on_login_submit =
        {
            let error_state = error_state.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let is_login_success = is_login_success.clone();

            Callback::from(move |login_data: LoginRequest| {
                let error_state = error_state.clone();
                let user_ctx = user_ctx.clone();
                let i18n = i18n.clone();
                let is_login_success = is_login_success.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let resp = Request::post("/api/login")
                        .json(&login_data)
                        .unwrap()
                        .send()
                        .await;

                    match resp {
                        Ok(r) if r.ok() => {
                            if let Ok(user) = r.json::<AuthUserDetail>().await {
                                user_ctx.dispatch(Some(user));
                                is_login_success.set(true);
                            } else {
                                error_state.set(Some(i18n.t("login-error-parse-response")));
                            }
                        }
                        Ok(r) => {
                            error_state.set(Some(i18n.t_args(
                                "login-error-failed",
                                &fluent_args!["status" => r.status()],
                            )));
                        }
                        Err(e) => {
                            error_state.set(Some(i18n.t_args(
                                "login-error-network",
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
                    // Pulling in your brand asset to establish identity
                    <img src="/images/kelpiedog_120x120_transparent.png" alt={i18n.t("login-logo-alt-text")} class="login-logo" />
                    <h1>{ i18n.t("branding-app-name") }</h1>
                    <p class="subtitle">{ i18n.t("branding-app-subtitle") }</p>
                </div>
                <LoginForm
                        on_login={on_login_submit}
                        error={(*error_state).clone()}
                    />
                if cfg!(feature = "password-reset") {
                    <div class="login-footer">
                        <p><Link<Route> to={Route::ForgotPassword}>{ i18n.t("login-forgot-password") }</Link<Route>></p>
                        <p>{ i18n.t("login-help-text") }</p>
                    </div>
                }
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct LoginFormProps {
    pub on_login: Callback<LoginRequest>,
    pub error: Option<String>,
}

#[function_component(LoginForm)]
pub fn login_form(props: &LoginFormProps) -> Html {
    let i18n = use_locale();

    let user_email = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let show_password = use_state(|| false);
    let error = props.error.clone();

    let on_user_email_input = {
        let state = user_email.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            state.set(input.value());
        })
    };
    let on_password_input = {
        let state = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            state.set(input.value());
        })
    };

    let on_submit = {
        let user_email = user_email.clone();
        let password = password.clone();
        let on_login = props.on_login.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let login_req = LoginRequest {
                email: (*user_email).clone(),
                password_raw: (*password).clone(),
                locale: Some(detect_browser_locale()),
            };
            on_login.emit(login_req);
        })
    };

    let toggle_password_visibility = {
        let show_password = show_password.clone();
        Callback::from(move |_| {
            show_password.set(!*show_password);
        })
    };

    let password_input_type = if *show_password { "text" } else { "password" };

    html! {
        <form onsubmit={on_submit} class="auth-form">
            <div class="input-field-group">
                <label>{i18n.t("login-form-email-label")}</label>
                <input type="text" value={(*user_email).clone()} oninput={on_user_email_input} required=true autocomplete="username" />
            </div>
            <div class="input-field-group">
                <label>{i18n.t("login-form-password-label")}</label>
                <div class="password-input-wrapper">
                    <input type={password_input_type} value={(*password).clone()} oninput={on_password_input} required=true autocomplete="current-password" />
                    <button type="button" class="icon-button" onclick={toggle_password_visibility}>
                        { if *show_password { "⊘" } else { "👁" } }
                    </button>
                </div>
            </div>
            <button type="submit" class="button-primary login-btn">
                    { i18n.t("login-form-submit-button") }
            </button>
            if let Some(err) = error {
                <div class="login__form__error">{err}</div>
            }
        </form>
    }
}
