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
use crate::contexts::locale_context::use_locale;
use crate::router::Route;
use fluent::fluent_args;
use serde::Serialize;
use shared_core::dtos::user_detail::AuthUserDetail;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[derive(Clone, Serialize, Default, Debug)]
struct UserUpdate {
    email: String,
    full_name: String,
    display_name: Option<String>,
}

#[derive(Clone, Serialize, Default, Debug)]
struct PasswordUpdate {
    old_password: String,
    new_password: String,
}

#[function_component(ProfilePage)]
pub fn profile_page() -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let user_update = use_state(UserUpdate::default);
    let details_error = use_state(|| None::<String>);
    let details_success = use_state(|| false);

    let navigator = use_navigator().unwrap();

    // Password form state
    let password_update = use_state(PasswordUpdate::default);
    let confirm_password = use_state(String::new);
    let password_error = use_state(|| None::<String>);
    let password_success = use_state(|| false);

    {
        let user_ctx = user_ctx.clone();
        let user_update = user_update.clone();
        use_effect_with(user_ctx, move |ctx| {
            if let Some(user) = &ctx.user {
                user_update.set(UserUpdate {
                    email: user.email.clone(),
                    full_name: user.full_name.clone(),
                    display_name: user.display_name.clone(),
                });
            }
            || ()
        });
    }

    let on_email_input = {
        let state = user_update.clone();
        Callback::from(move |e: InputEvent| {
            let mut info = (*state).clone();
            info.email = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            state.set(info);
        })
    };
    let on_full_name_input = {
        let state = user_update.clone();
        Callback::from(move |e: InputEvent| {
            let mut info = (*state).clone();
            info.full_name = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            state.set(info);
        })
    };
    let on_display_name_input = {
        let state = user_update.clone();
        Callback::from(move |e: InputEvent| {
            let mut info = (*state).clone();
            let value = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            info.display_name = if value.is_empty() { None } else { Some(value) };
            state.set(info);
        })
    };

    let on_submit_details = {
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let user_update = user_update.clone();
        let error_state = details_error.clone();
        let success_state = details_success.clone();
        let navigator = navigator.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let user_update = user_update.clone();
            let error_state = error_state.clone();
            let success_state = success_state.clone();
            let navigator = navigator.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let resp = Api::put(
                    "/api/users/me",
                    &*user_update,
                    user_ctx.clone(),
                    navigator.clone(),
                )
                .await;

                match resp {
                    Ok(r) if r.ok() => {
                        if let Ok(updated_user) = r.json::<AuthUserDetail>().await {
                            user_ctx.dispatch(Some(updated_user));
                            success_state.set(true);
                            error_state.set(None);
                            navigator.push(&Route::Dashboard);
                        } else {
                            error_state.set(Some(i18n.t("profile-error-parse-response")));
                        }
                    }
                    Ok(r) => {
                        error_state.set(Some(i18n.t_args(
                            "profile-error-save-profile",
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

    // --- Password Form Logic ---
    let on_old_password_input = {
        let state = password_update.clone();
        Callback::from(move |e: InputEvent| {
            let mut info = (*state).clone();
            info.old_password = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            state.set(info);
        })
    };
    let on_new_password_input = {
        let state = password_update.clone();
        Callback::from(move |e: InputEvent| {
            let mut info = (*state).clone();
            info.new_password = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            state.set(info);
        })
    };
    let on_confirm_password_input = {
        let state = confirm_password.clone();
        Callback::from(move |e: InputEvent| {
            state.set(
                e.target_unchecked_into::<web_sys::HtmlInputElement>()
                    .value(),
            );
        })
    };

    let passwords_match = !password_update.new_password.is_empty()
        && password_update.new_password == *confirm_password;
    let can_submit_password = !password_update.old_password.is_empty() && passwords_match;

    let on_submit_password = {
        let password_update = password_update.clone();
        let password_error = password_error.clone();
        let password_success = password_success.clone();
        let navigator = navigator.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if can_submit_password {
                let password_update = password_update.clone();
                let password_error = password_error.clone();
                let password_success = password_success.clone();
                let navigator = navigator.clone();
                let user_ctx = user_ctx.clone();
                let i18n = i18n.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let resp = Api::put(
                        "/api/users/me/password",
                        &*password_update,
                        user_ctx,
                        navigator.clone(),
                    )
                    .await;

                    match resp {
                        Ok(r) if r.ok() => {
                            password_success.set(true);
                            password_error.set(None);
                            navigator.push(&Route::Dashboard);
                        }
                        Ok(r) => {
                            password_error.set(Some(i18n.t_args(
                                "profile-error-change-password",
                                &fluent_args!["status" => r.status()],
                            )));
                        }
                        Err(e) => {
                            password_error.set(Some(i18n.t_args(
                                "common-network-error",
                                &fluent_args!["error" => e.to_string()],
                            )));
                        }
                    }
                });
            }
        })
    };

    html! {
        <Layout>
            <h1>{ i18n.t("profile-title") }</h1>
            <div class="profile__forms-container">
                <form onsubmit={on_submit_details} class="profile__form">
                    <h2>{ i18n.t("profile-details-title") }</h2>
                    <label>{i18n.t("profile-email-label")}</label>
                    <input type="email" value={user_update.email.clone()} oninput={on_email_input} required=true />

                    <label>{i18n.t("profile-full-name-label")}</label>
                    <input type="text" value={user_update.full_name.clone()} oninput={on_full_name_input} required=true />

                    <label>{i18n.t("profile-display-name-label")}</label>
                    <input type="text" value={user_update.display_name.clone().unwrap_or_default()} oninput={on_display_name_input} />

                    <div class="Method…-actions">
                        <button type="submit">{i18n.t("profile-save-details-button")}</button>
                    </div>
                    if *details_success {
                        <div class="message message__success">{i18n.t("profile-save-success-message")}</div>
                    }
                    if let Some(err) = (*details_error).clone() {
                        <div class="message message__error">{err}</div>
                    }
                </form>

                <form onsubmit={on_submit_password} class="profile__form">
                    <h2>{ i18n.t("profile-change-password-title") }</h2>
                    <label>{i18n.t("profile-old-password-label")}</label>
                    <input type="password" oninput={on_old_password_input} required=true />

                    <label>{i18n.t("profile-new-password-label")}</label>
                    <input type="password" oninput={on_new_password_input} required=true />

                    <label>{i18n.t("profile-confirm-password-label")}</label>
                    <input
                        type="password"
                        oninput={on_confirm_password_input}
                        required=true
                        class={if !confirm_password.is_empty() { if passwords_match { "input-success" } else { "input-error" } } else { "" }}
                    />

                    <div class="profile__form-actions">
                        <button type="submit" disabled={!can_submit_password}>{i18n.t("profile-change-password-button")}</button>
                    </div>
                    if *password_success {
                        <div class="message message__success">{i18n.t("profile-password-change-success")}</div>
                    }
                    if let Some(err) = (*password_error).clone() {
                        <div class="message message__error">{err}</div>
                    }
                </form>
            </div>
        </Layout>
    }
}
