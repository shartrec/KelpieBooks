/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::router::Route;
use fluent::fluent_args;
use gloo_net::http::Request;
use shared_core::i18n::{t, t_args};
use shared_core::requests::onboard::OnboardingRequest;
use yew::function_component;
use yew::html;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

#[function_component(RegisterPage)]
pub fn register_page() -> Html {
    let error_state = use_state(|| None::<String>);
    let navigator = use_navigator().unwrap();

    let on_register_submit = {
        let error_state = error_state.clone();
        let navigator = navigator.clone();

        Callback::from(move |reg_data: OnboardingRequest| {
            let error_state = error_state.clone();
            let navigator = navigator.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let resp = Request::post("/api/register")
                    .json(&reg_data)
                    .unwrap()
                    .send()
                    .await;

                match resp {
                    Ok(r) if r.status() == 200 => {
                        navigator.push(&Route::Login);
                    }
                    Ok(r) => {
                        error_state.set(Some(t_args(
                            "register-error-server",
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

    html! {
        <div class="page-container">
            <h1>{t("register-title")}</h1>
            <RegisterForm
                on_register={on_register_submit}
                error={(*error_state).clone()}
            />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct RegisterFormProps {
    pub on_register: Callback<OnboardingRequest>,
    pub error: Option<String>,
}

#[function_component(RegisterForm)]
pub fn register_form(props: &RegisterFormProps) -> Html {
    let user_email = use_state(String::new);
    let password = use_state(String::new);
    let full_name = use_state(String::new);
    let display_name = use_state(String::new);
    let organisation = use_state(String::new);
    let coa_template_id = use_state(|| "service".to_string());
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
    let on_full_name_input = {
        let state = full_name.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            state.set(input.value());
        })
    };
    let on_display_name_input = {
        let state = display_name.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            state.set(input.value());
        })
    };
    let on_organisation_input = {
        let state = organisation.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            state.set(input.value());
        })
    };
    let on_coa_template_id_input = {
        let state = coa_template_id.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            state.set(input.value());
        })
    };

    let on_submit = {
        // Clone all the state handles before the `move` closure
        let user_email = user_email.clone();
        let password = password.clone();
        let full_name = full_name.clone();
        let display_name = display_name.clone();
        let organisation = organisation.clone();
        let coa_template_id = coa_template_id.clone();
        let on_register = props.on_register.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            // The closure now owns the clones, leaving the originals for the html! macro
            let register_req = OnboardingRequest {
                user_email: (*user_email).clone(),
                user_password: (*password).clone(),
                user_full_name: (*full_name).clone(),
                user_display_name: if display_name.is_empty() {
                    None
                } else {
                    Some((*display_name).clone())
                },
                organization_name: (*organisation).clone(),
                coa_template_id: (*coa_template_id).clone(),
            };
            on_register.emit(register_req);
        })
    };

    html! {
        <form onsubmit={on_submit} class="auth-form">
            <label>{t("register-org-name-label")}</label>
            <input type="text" value={(*organisation).clone()} oninput={on_organisation_input} required=true />

            <label>{t("register-full-name-label")}</label>
            <input type="text" value={(*full_name).clone()} oninput={on_full_name_input} required=true />

            <label>{t("register-display-name-label")}</label>
            <input type="text" value={(*display_name).clone()} oninput={on_display_name_input} />

            <label>{t("register-email-label")}</label>
            <input type="email" value={(*user_email).clone()} oninput={on_user_email_input} required=true autocomplete="email" />

            <label>{t("register-password-label")}</label>
            <input type="password" value={(*password).clone()} oninput={on_password_input} required=true autocomplete="new-password" />

            <label>{t("register-coa-template-label")}</label>
            <input type="text" value={(*coa_template_id).clone()} oninput={on_coa_template_id_input} required=true />

            <div class="form-actions">
                <button type="submit">{t("register-submit-button")}</button>
            </div>
            if let Some(err) = error {
                <div class="error">{err}</div>
            }
        </form>
    }
}
