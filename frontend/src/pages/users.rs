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
use shared_core::dtos::user_detail::UserDetail;
use std::rc::Rc;
use yew::prelude::*;
use yew_router::prelude::*;
use shared_core::dtos::ApiErrorMessage;

#[function_component(UsersPage)]
pub fn users_page() -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();
    let users = use_state(|| Rc::new(Vec::<UserDetail>::new()));
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);

    let fetch_users = {
        let users = users.clone();
        let error = error.clone();
        let loading = loading.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        Callback::from(move |()| {
            let users = users.clone();
            let error = error.clone();
            let loading = loading.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                let fetched_users = Api::get("/api/users", user_ctx, navigator).await;
                loading.set(false);
                match fetched_users {
                    Ok(response) if response.ok() => {
                        match response.json::<Vec<UserDetail>>().await {
                            Ok(data) => users.set(Rc::new(data)),
                            Err(e) => error.set(Some(i18n.t_args(
                                "users-error-parse",
                                &fluent_args!["error" => e.to_string()],
                            ))),
                        }
                    }
                    Ok(response) => error.set(Some(i18n.t_args(
                        "users-error-fetch",
                        &fluent_args!["status" => response.status()],
                    ))),
                    Err(e) => error.set(Some(
                        i18n.t_args("coa-error-network", &fluent_args!["error" => e.to_string()]),
                    )),
                }
            });
        })
    };

    {
        let fetch_users = fetch_users.clone();
        use_effect_with((), move |_| {
            fetch_users.emit(());
            || ()
        });
    }

    let on_delete_click = {
        let fetch_users = fetch_users.clone();
        let error = error.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        Callback::from(move |user_id| {
            let fetch_users = fetch_users.clone();
            let error = error.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("/api/users/{}", user_id);
                let resp = Api::delete(&url, user_ctx, navigator).await;
                match resp {
                    Ok(r) if r.ok() => {
                        fetch_users.emit(());
                    }
                    Ok(r) => {
                        // Clone hooks so they can cross the upcoming async deserialization boundary
                        let error = error.clone();
                        let i18n = i18n.clone();
                        let status = r.status();

                        wasm_bindgen_futures::spawn_local(async move {
                            // Attempt to parse the structured error body from the backend JSON payload
                            if let Ok(error_payload) = r.json::<ApiErrorMessage>().await {
                                error.set(Some(i18n.t_args(
                                    "users-error-delete",
                                    &fluent_args!["error" => error_payload.error]
                                )));
                            } else {
                                // Fallback: If the body isn't standard JSON, drop back to the HTTP code number
                                error.set(Some(i18n.t_args(
                                    "users-error-delete",
                                    &fluent_args ! ["error" => status],
                                )));
                            }
                        });
                    }
                    Err(e) => error.set(Some(i18n.t_args(
                        "common-network-error",
                        &fluent_args!["error" => e.to_string()],
                    ))),
                }
            });
        })
    };

    let on_add_user = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            // TODO: Implement Add User modal or page
        })
    };

    let on_edit_user = {
        let navigator = navigator.clone();
        Callback::from(move |user_id| {
            // TODO: Implement Edit User modal or page
        })
    };

    html! {
        <Layout>

            <h1>{ i18n.t("users-title") }</h1>
            <p>{ i18n.t("users-list-description") }</p>
            <div class="table-actions">
                <button class="button" onclick={on_add_user}>
                    { i18n.t("users-add-button") }
                </button>
            </div>
            if *loading {
                <p>{ i18n.t("common-loading") }</p>
            } else {
                if let Some(err) = &*error {
                    <div class="message__error">{ err }</div>
                }
                <table class="table">
                    <thead>
                        <tr>
                            <th class="table__text-col">{ i18n.t("users-header-email") }</th>
                            <th class="table__text-col">{ i18n.t("users-header-full-name") }</th>
                            <th class="table__text-col">{ i18n.t("users-header-display-name") }</th>
                            <th class="table__text-col">{ i18n.t("users-header-role") }</th>
                            <th class="table__col-actions">{ i18n.t("common-actions") }</th>
                        </tr>
                    </thead>
                    <tbody>
                        { for users.iter().map(|user| {
                            let user_id = user.id;
                            let on_delete = on_delete_click.clone();
                            let on_edit = on_edit_user.clone();
                            html! {
                                <tr key={user.id.to_string()}>
                                    <td>{ &user.email }</td>
                                    <td>{ &user.full_name }</td>
                                    <td>{ user.display_name.as_deref().unwrap_or("") }</td>
                                    <td>{ &user.role }</td>
                                    <td class="table__col-actions">
                                        <div class="actions-wrapper">
                                            <button class="icon-button btn-action" onclick={move |_| on_edit.emit(user_id)}>
                                                <img src="/images/edit.svg" alt={i18n.t("common-edit")} />
                                            </button>
                                            <button class="icon-button btn-action" onclick={move |_| on_delete.emit(user_id)}>
                                                <img src="/images/delete.svg" alt={i18n.t("common-delete")} />
                                            </button>
                                        </div>
                                    </td>
                                </tr>
                            }
                        })}
                    </tbody>
                </table>
            }
        </Layout>
    }
}
