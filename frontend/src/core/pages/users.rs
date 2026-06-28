/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use std::rc::Rc;

use fluent::fluent_args;
use shared_core::core::{
    dtos::user_detail::UserDetail,
    models::role::Role,
    requests::user::{
        CreateUserRequest,
        UpdateUserRequest,
    },
};
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    api::Api,
    contexts::{
        auth_context::use_user_context,
        locale_context::use_locale,
    },
    core::{
        components::{
            add_user_modal::AddUserModal,
            delete_confirmation_modal::DeleteConfirmationModal,
            edit_user_modal::EditUserModal,
            layout::Layout,
        },
        pages,
    },
};

#[function_component(UsersPage)]
pub fn users_page() -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();
    let users = use_state(|| Rc::new(Vec::<UserDetail>::new()));
    let roles = use_state(|| Vec::<Role>::new());
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);
    let show_add_modal = use_state(|| false);
    let show_edit_modal = use_state(|| None::<UserDetail>);
    let user_to_delete = use_state(|| None::<UserDetail>);

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
                        i18n.t_args("common-network-error", &fluent_args!["error" => e.to_string()]),
                    )),
                }
            });
        })
    };

    let fetch_roles = {
        let roles = roles.clone();
        let error = error.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        Callback::from(move |()| {
            let roles = roles.clone();
            let error = error.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_roles = Api::get("/api/roles", user_ctx, navigator).await;
                match fetched_roles {
                    Ok(response) if response.ok() => {
                        match response.json::<Vec<Role>>().await {
                            Ok(data) => roles.set(data),
                            Err(e) => error.set(Some(i18n.t_args(
                                "roles-error-parse",
                                &fluent_args!["error" => e.to_string()],
                            ))),
                        }
                    }
                    Ok(response) => error.set(Some(i18n.t_args(
                        "roles-error-fetch",
                        &fluent_args!["status" => response.status()],
                    ))),
                    Err(e) => error.set(Some(
                        i18n.t_args("common-network-error", &fluent_args!["error" => e.to_string()]),
                    )),
                }
            });
        })
    };

    {
        let fetch_users = fetch_users.clone();
        let fetch_roles = fetch_roles.clone();
        use_effect_with((), move |_| {
            fetch_users.emit(());
            fetch_roles.emit(());
            || ()
        });
    }

    let on_add_user_click = {
        let show_add_modal = show_add_modal.clone();
        Callback::from(move |_| show_add_modal.set(true))
    };

    let on_add_modal_close = {
        let show_add_modal = show_add_modal.clone();
        Callback::from(move |_| show_add_modal.set(false))
    };

    let on_add_modal_submit = {
        let fetch_users = fetch_users.clone();
        let error = error.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        let show_add_modal = show_add_modal.clone();
        Callback::from(move |req: CreateUserRequest| {
            let fetch_users = fetch_users.clone();
            let error = error.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            let show_add_modal = show_add_modal.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Api::post("/api/users", &req, user_ctx, navigator).await;
                match resp {
                    Ok(r) if r.ok() => {
                        show_add_modal.set(false);
                        fetch_users.emit(());
                    }
                    Ok(r) => {
                        pages::set_error(error, i18n, r, "users-error-add");
                    }
                    Err(e) => error.set(Some(i18n.t_args(
                        "common-network-error",
                        &fluent_args!["error" => e.to_string()],
                    ))),
                }
            });
        })
    };

    let on_edit_user_click = {
        let show_edit_modal = show_edit_modal.clone();
        Callback::from(move |user: UserDetail| show_edit_modal.set(Some(user)))
    };

    let on_edit_modal_close = {
        let show_edit_modal = show_edit_modal.clone();
        Callback::from(move |_| show_edit_modal.set(None))
    };

    let on_edit_modal_submit = {
        let fetch_users = fetch_users.clone();
        let error = error.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        let show_edit_modal = show_edit_modal.clone();
        Callback::from(move |(user_id, req): (Uuid, UpdateUserRequest)| {
            let fetch_users = fetch_users.clone();
            let error = error.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            let show_edit_modal = show_edit_modal.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("/api/users/{}", user_id);
                let resp = Api::put(&url, &req, user_ctx, navigator).await;
                show_edit_modal.set(None);
                match resp {
                    Ok(r) if r.ok() => {
                        fetch_users.emit(());
                    }
                    Ok(r) => {
                        pages::set_error(error, i18n, r, "users-error-update");
                    }
                    Err(e) => error.set(Some(i18n.t_args(
                        "common-network-error",
                        &fluent_args!["error" => e.to_string()],
                    ))),
                }
            });
        })
    };

    let on_delete_click = {
        let user_to_delete = user_to_delete.clone();
        Callback::from(move |user: UserDetail| {
            user_to_delete.set(Some(user));
        })
    };

    let on_delete_confirm = {
        let fetch_users = fetch_users.clone();
        let error = error.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        let user_to_delete = user_to_delete.clone();
        Callback::from(move |user_id| {
            let fetch_users = fetch_users.clone();
            let error = error.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            let user_to_delete = user_to_delete.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("/api/users/{}", user_id);
                let resp = Api::delete(&url, user_ctx, navigator).await;
                user_to_delete.set(None);
                match resp {
                    Ok(r) if r.ok() => {
                        fetch_users.emit(());
                    }
                    Ok(r) => {
                        pages::set_error(error, i18n, r, "users-error-delete");
                    }
                    Err(e) => error.set(Some(i18n.t_args(
                        "common-network-error",
                        &fluent_args!["error" => e.to_string()],
                    ))),
                }
            });
        })
    };

    let on_delete_confirm_click = {
        let user_to_delete = user_to_delete.clone();

        Callback::from(move |_| {
            let user_to_delete = user_to_delete.clone();
            let id = user_to_delete.as_ref().unwrap().id;
            on_delete_confirm.emit(id)
        })
    };

    let on_delete_cancel = {
        let user_to_delete = user_to_delete.clone();
        Callback::from(move |()| {
            user_to_delete.set(None);
        })
    };

    html! {
        <Layout>

            <h1>{ i18n.t("users-title") }</h1>
            <p>{ i18n.t("users-list-description") }</p>
            <div class="table-actions">
                <button class="button" onclick={on_add_user_click}>
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
                            let user_clone = user.clone();
                            let user_clone2 = user.clone();
                            let on_delete = on_delete_click.clone();
                            let on_edit = on_edit_user_click.clone();
                            let none = i18n.t("common-none");
                            let role_name = user.role.as_deref().unwrap_or_else(|| &none);
                            html! {
                                <tr key={user.id.to_string()}>
                                    <td>{ &user.email }</td>
                                    <td>{ &user.full_name }</td>
                                    <td>{ user.display_name.as_deref().unwrap_or("") }</td>
                                    <td>{ role_name }</td>
                                    <td class="table__col-actions">
                                        <div class="actions-wrapper">
                                            <button class="icon-button btn-action" onclick={move |_| on_edit.emit(user_clone.clone())}>
                                                <img src="/images/edit.svg" alt={i18n.t("common-edit")} />
                                            </button>
                                            <button class="icon-button btn-action" onclick={move |_| on_delete.emit(user_clone2.clone())}>
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
            if *show_add_modal {
                <AddUserModal roles={(*roles).clone()} on_close={on_add_modal_close} on_submit={on_add_modal_submit} />
            }
            if let Some(user) = &*show_edit_modal {
                <EditUserModal user={user.clone()} roles={(*roles).clone()} on_close={on_edit_modal_close} on_submit={on_edit_modal_submit.clone()} />
            }
            if let Some(user) = &*user_to_delete {
                <DeleteConfirmationModal
                    title={i18n.t("delete-user-confirm-title")}
                    message={i18n.t_args("delete-user-confirm-message", &fluent_args!["user" => user.full_name.clone()])}
                    on_confirm={on_delete_confirm_click.clone()}
                    on_cancel={on_delete_cancel}
                />
            }
        </Layout>
    }
}
