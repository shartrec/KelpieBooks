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

use yew::prelude::*;
use crate::auth::UserContextHandle;
use crate::Route;
use gloo_net::http::Request;
use yew_router::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    let user_ctx = use_context::<UserContextHandle>();
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

    // SVG Icons
    let profile_icon = html! { <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path><circle cx="12" cy="7" r="4"></circle></svg> };
    let logout_icon = html! { <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"></path><polyline points="16 17 21 12 16 7"></polyline><line x1="21" y1="12" x2="9" y2="12"></line></svg> };

    html! {
        <header class="header">
            <div class="header-content">
                <div class="user-menu">
                    <button onclick={toggle_dropdown} class="user-menu-trigger">
                        { user_display }
                        // A little chevron icon to indicate a dropdown
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="6 9 12 15 18 9"></polyline></svg>
                    </button>
                    if *dropdown_open {
                        <div class="user-menu-dropdown">
                            <Link<Route> to={Route::Profile} classes="dropdown-item">
                                { profile_icon }
                                <span>{ "Edit Profile" }</span>
                            </Link<Route>>
                            <button onclick={on_logout_click} class="dropdown-item">
                                { logout_icon }
                                <span>{ "Logout" }</span>
                            </button>
                        </div>
                    }
                </div>
            </div>
        </header>
    }
}
