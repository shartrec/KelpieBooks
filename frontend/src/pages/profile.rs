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

use yew::prelude::*;
use crate::components::layout::Layout;
use crate::auth::{UserContextHandle, CurrentUser};
use serde::Serialize;
use gloo_net::http::Request;

#[derive(Clone, Serialize, Default, Debug)]
struct UserUpdate {
    full_name: String,
    display_name: Option<String>,
}

#[function_component(ProfilePage)]
pub fn profile_page() -> Html {
    let user_ctx = use_context::<UserContextHandle>().expect("User context not found");
    let user_update = use_state(UserUpdate::default);
    let error_state = use_state(|| None::<String>);
    let success_state = use_state(|| false);

    {
        let user_ctx = user_ctx.clone();
        let user_update = user_update.clone();
        use_effect_with(user_ctx, move |ctx| {
            if let Some(user) = &ctx.user {
                user_update.set(UserUpdate {
                    full_name: user.full_name.clone(),
                    display_name: user.display_name.clone(),
                });
            }
            || ()
        });
    }

    let on_full_name_input = { let state = user_update.clone(); Callback::from(move |e: InputEvent| { let mut info = (*state).clone(); info.full_name = e.target_unchecked_into::<web_sys::HtmlInputElement>().value(); state.set(info); }) };
    let on_display_name_input = { let state = user_update.clone(); Callback::from(move |e: InputEvent| { let mut info = (*state).clone(); let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value(); info.display_name = if value.is_empty() { None } else { Some(value) }; state.set(info); }) };

    let on_submit = {
        let user_ctx = user_ctx.clone();
        let user_update = user_update.clone();
        let error_state = error_state.clone();
        let success_state = success_state.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let user_ctx = user_ctx.clone();
            let user_update = user_update.clone();
            let error_state = error_state.clone();
            let success_state = success_state.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let resp = Request::put("/api/users/me")
                    .json(&(*user_update))
                    .unwrap().send()
                    .await;

                match resp {
                    Ok(r) if r.ok() => {
                        if let Ok(updated_user) = r.json::<CurrentUser>().await {
                            // Update the global context with the new user info
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

    html! {
        <Layout>
            <h1>{ "Edit Profile" }</h1>
            <form onsubmit={on_submit} class="auth-form">
                <label>{"Full Name:"}</label>
                <input type="text" value={user_update.full_name.clone()} oninput={on_full_name_input} required=true />

                <label>{"Display Name:"}</label>
                <input type="text" value={user_update.display_name.clone().unwrap_or_default()} oninput={on_display_name_input} />

                <div class="form-actions">
                    <button type="submit">{"Save Changes"}</button>
                </div>
                if *success_state {
                    <div class="success-message">{"Profile saved successfully!"}</div>
                }
                if let Some(err) = (*error_state).clone() {
                    <div class="error">{err}</div>
                }
            </form>
        </Layout>
    }
}
