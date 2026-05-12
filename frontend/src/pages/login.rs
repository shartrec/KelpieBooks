/*
 * Copyright (c) 2025-2026. Trevor Campbell and others.
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

use crate::router::Route;
use gloo_net::http::Request;
use shared_core::dtos::user_detail::UserDetail;
use shared_core::requests::auth::LoginRequest;
use yew::function_component;
use yew::html;
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use crate::contexts::auth_context::UserContextHandle;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
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

    let on_login_submit = {
        let error_state = error_state.clone();
        let user_ctx = user_ctx.clone();
        let is_login_success = is_login_success.clone();

        Callback::from(move |login_data: LoginRequest| {
            let error_state = error_state.clone();
            let user_ctx = user_ctx.clone();
            let is_login_success = is_login_success.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let resp = Request::post("/api/login")
                    .json(&login_data)
                    .unwrap()
                    .send()
                    .await;

                match resp {
                    Ok(r) if r.ok() => {
                        if let Ok(user) = r.json::<UserDetail>().await {
                            user_ctx.dispatch(Some(user));
                            is_login_success.set(true);
                        } else {
                            error_state.set(Some("Failed to parse login response.".to_string()));
                        }
                    }
                    Ok(r) => {
                        error_state.set(Some(format!("Login failed: {}", r.status())));
                    }
                    Err(e) => {
                        error_state.set(Some(format!("Network error: {}", e)));
                    }
                }
            });
        })
    };

    html! {
        <div class="login__page-contianer">
            <h1>{"Please login"}</h1>
            if !*is_login_success {
                <LoginForm
                    on_login={on_login_submit}
                    error={(*error_state).clone()}
                />
            } else {
                <div class="card">
                    <p>{"Login successful, redirecting to your dashboard..."}</p>
                </div>
            }
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
    let user_email = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
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
            };
            on_login.emit(login_req);
        })
    };

    html! {
        <form onsubmit={on_submit} class="login__form">
            <label>{"User Email: "}</label>
            <input type="text" value={(*user_email).clone()} oninput={on_user_email_input} required=true autocomplete="username" />
            <label>{"Password: "}</label>
            <input type="password" value={(*password).clone()} oninput={on_password_input} required=true autocomplete="current-password" />
            <div class="login__form__form-actions">
                <button type="submit">{"Login"}</button>
            </div>
            if let Some(err) = error {
                <div class="login__form__error">{err}</div>
            }
        </form>
    }
}
