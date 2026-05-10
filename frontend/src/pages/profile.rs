/*
 * Copyright (c) 2026-2026. Trevor Campbell and others.
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

use crate::auth::UserContextHandle;
use crate::components::layout::Layout;
use gloo_net::http::Request;
use serde::Serialize;
use shared_core::dtos::user_detail::UserDetail;
use yew::prelude::*;

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
    let user_ctx = use_context::<UserContextHandle>().expect("User context not found");
    let user_update = use_state(UserUpdate::default);
    let details_error = use_state(|| None::<String>);
    let details_success = use_state(|| false);

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
        let user_update = user_update.clone();
        let error_state = details_error.clone();
        let success_state = details_success.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let user_ctx = user_ctx.clone();
            let user_update = user_update.clone();
            let error_state = error_state.clone();
            let success_state = success_state.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let resp = Request::put("/api/users/me")
                    .json(&(*user_update))
                    .unwrap()
                    .send()
                    .await;

                match resp {
                    Ok(r) if r.ok() => {
                        if let Ok(updated_user) = r.json::<UserDetail>().await {
                            user_ctx.dispatch(Some(updated_user));
                            success_state.set(true);
                            error_state.set(None);
                        } else {
                            error_state.set(Some("Failed to parse server response.".to_string()));
                        }
                    }
                    Ok(r) => {
                        error_state.set(Some(format!("Error saving profile: {}", r.status())));
                    }
                    Err(e) => {
                        error_state.set(Some(format!("Network error: {}", e)));
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
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if can_submit_password {
                let password_update = password_update.clone();
                let password_error = password_error.clone();
                let password_success = password_success.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let resp = Request::put("/api/users/me/password")
                        .json(&(*password_update))
                        .unwrap()
                        .send()
                        .await;

                    match resp {
                        Ok(r) if r.ok() => {
                            password_success.set(true);
                            password_error.set(None);
                        }
                        Ok(r) => {
                            password_error
                                .set(Some(format!("Error changing password: {}", r.status())));
                        }
                        Err(e) => {
                            password_error.set(Some(format!("Network error: {}", e)));
                        }
                    }
                });
            }
        })
    };

    html! {
        <Layout>
            <h1>{ "Edit Profile" }</h1>
            <div class="profile__forms-container">
                <form onsubmit={on_submit_details} class="profile__form">
                    <h2>{ "Your Details" }</h2>
                    <label>{"Email:"}</label>
                    <input type="email" value={user_update.email.clone()} oninput={on_email_input} required=true />

                    <label>{"Full Name:"}</label>
                    <input type="text" value={user_update.full_name.clone()} oninput={on_full_name_input} required=true />

                    <label>{"Display Name:"}</label>
                    <input type="text" value={user_update.display_name.clone().unwrap_or_default()} oninput={on_display_name_input} />

                    <div class="Method…-actions">
                        <button type="submit">{"Save Details"}</button>
                    </div>
                    if *details_success {
                        <div class="success-message">{"Profile saved successfully!"}</div>
                    }
                    if let Some(err) = (*details_error).clone() {
                        <div class="error">{err}</div>
                    }
                </form>

                <form onsubmit={on_submit_password} class="profile__form">
                    <h2>{ "Change Password" }</h2>
                    <label>{"Old Password:"}</label>
                    <input type="password" oninput={on_old_password_input} required=true />

                    <label>{"New Password:"}</label>
                    <input type="password" oninput={on_new_password_input} required=true />

                    <label>{"Confirm New Password:"}</label>
                    <input
                        type="password"
                        oninput={on_confirm_password_input}
                        required=true
                        class={if !confirm_password.is_empty() { if passwords_match { "input-success" } else { "input-error" } } else { "" }}
                    />

                    <div class="profile__form-actions">
                        <button type="submit" disabled={!can_submit_password}>{"Change Password"}</button>
                    </div>
                    if *password_success {
                        <div class="success-message">{"Password changed successfully!"}</div>
                    }
                    if let Some(err) = (*password_error).clone() {
                        <div class="error">{err}</div>
                    }
                </form>
            </div>
        </Layout>
    }
}
