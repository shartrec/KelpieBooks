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

use crate::auth::UserContextHandle;
use crate::Route;
use gloo_net::http::Request;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::org::OrgContextHandle;

#[function_component(Header)]
pub fn header() -> Html {
    let user_ctx = use_context::<UserContextHandle>();
    let org_state = use_context::<OrgContextHandle>();
    let navigator = use_navigator().unwrap();
    let dropdown_open = use_state(|| false);

    let on_logout_click = {
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        Callback::from(move |_| {
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Request::post("/api/auth/logout").send().await;
                if let Ok(_) = resp {
                    if let Some(handle) = user_ctx {
                        handle.dispatch(None);
                    }
                    navigator.push(&Route::Login);
                }
            });
        })
    };

    let toggle_dropdown = {
        let dropdown_open = dropdown_open.clone();
        Callback::from(move |_| {
            dropdown_open.set(!*dropdown_open);
        })
    };

    let organisation_name = if let Some(org_state) = &org_state {
        org_state.name.clone()
    } else {
        "".to_string()
    };

    let user_display = if let Some(handle) = user_ctx {
        if let Some(user) = &handle.user {
            let name_to_display = user.display_name.as_ref().unwrap_or(&user.full_name);
            html! { <span class="username">{ name_to_display }</span> }
        } else {
            html! { <span class="username">{ "Loading..." }</span> }
        }
    } else {
        html! { <span></span> }
    };

    html! {
        <header class="header">
            <div class="header__content">
                <div class="header__left">
                    <h2>{ organisation_name }</h2>
                </div>
                <div class="header__right">
                    <div class="user-menu">
                        <button onclick={toggle_dropdown} class="user-menu-trigger">
                            { user_display }
                            <img src="/images/chevron-down.svg" alt="Toggle menu" />
                        </button>
                        if *dropdown_open {
                            <div class="user-menu-dropdown">
                                <Link<Route> to={Route::Profile} classes="user-menu__item">
                                    <img src="/images/user.svg" alt="Profile" />
                                    <span>{ "Edit Profile" }</span>
                                </Link<Route>>
                                <button onclick={on_logout_click} class="user-menu__item">
                                    <img src="/images/logout.svg" alt="Logout" />
                                    <span>{ "Logout" }</span>
                                </button>
                            </div>
                        }
                    </div>
                </div>
            </div>
        </header>
    }
}
