/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use gloo_net::http::Request;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    contexts::{
        auth_context::UserContextHandle,
        locale_context::use_locale,
        org_context::OrgContextHandle,
    },
    router::Route,
};

#[function_component(Header)]
pub fn header() -> Html {
    let user_ctx = use_context::<UserContextHandle>();
    let i18n = use_locale();
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
            html! { <span class="username">{ i18n.t("common-loading") }</span> }
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
                            <img src="/images/chevron-down.svg" alt={i18n.t("header-toggle-menu-alt")} />
                        </button>
                        if *dropdown_open {
                            <div class="user-menu-dropdown">
                                <Link<Route> to={Route::Profile} classes="user-menu__item">
                                    <img src="/images/user.svg" alt={i18n.t("header-profile-alt")} />
                                    <span>{ i18n.t("header-edit-profile") }</span>
                                </Link<Route>>
                                <button onclick={on_logout_click} class="user-menu__item">
                                    <img src="/images/logout.svg" alt={i18n.t("header-logout-alt")} />
                                    <span>{ i18n.t("header-logout") }</span>
                                </button>
                            </div>
                        }
                    </div>
                </div>
            </div>
        </header>
    }
}
