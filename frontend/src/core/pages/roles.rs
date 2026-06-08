/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use std::rc::Rc;

use fluent::fluent_args;
use shared_core::core::requests::role::{
    CreateRoleRequest,
    UpdateRoleRequest,
};
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;
use shared_core::core::models::role::Role;
use crate::{
    api::Api,
    contexts::{
        auth_context::use_user_context,
        locale_context::use_locale,
    },
    core::components::{
        add_role_modal::AddRoleModal,
        edit_role_modal::EditRoleModal,
        generic_delete_confirmation_modal::GenericDeleteConfirmationModal,
        layout::Layout,
    },
    core::pages,
};

#[function_component(RolesPage)]
pub fn roles_page() -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();
    let roles = use_state(|| Rc::new(Vec::<Role>::new()));
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);
    let show_add_modal = use_state(|| false);
    let show_edit_modal = use_state(|| None::<Role>);
    let role_to_delete = use_state(|| None::<Role>);

    let fetch_roles = {
        let roles = roles.clone();
        let error = error.clone();
        let loading = loading.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        Callback::from(move |()| {
            let roles = roles.clone();
            let error = error.clone();
            let loading = loading.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                let fetched_roles = Api::get("/api/roles", user_ctx, navigator).await;
                loading.set(false);
                match fetched_roles {
                    Ok(response) if response.ok() => {
                        match response.json::<Vec<Role>>().await {
                            Ok(data) => roles.set(Rc::new(data)),
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
                        i18n.t_args("coa-error-network", &fluent_args!["error" => e.to_string()]),
                    )),
                }
            });
        })
    };

    {
        let fetch_roles = fetch_roles.clone();
        use_effect_with((), move |_| {
            fetch_roles.emit(());
            || ()
        });
    }

    let on_add_role_click = {
        let show_add_modal = show_add_modal.clone();
        Callback::from(move |_| show_add_modal.set(true))
    };

    let on_add_modal_close = {
        let show_add_modal = show_add_modal.clone();
        Callback::from(move |_| show_add_modal.set(false))
    };

    let on_add_modal_submit = {
        let fetch_roles = fetch_roles.clone();
        let error = error.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        let show_add_modal = show_add_modal.clone();
        Callback::from(move |req: CreateRoleRequest| {
            let fetch_roles = fetch_roles.clone();
            let error = error.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            let show_add_modal = show_add_modal.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Api::post("/api/roles", &req, user_ctx, navigator).await;
                match resp {
                    Ok(r) if r.ok() => {
                        show_add_modal.set(false);
                        fetch_roles.emit(());
                    }
                    Ok(r) => {
                        pages::set_error(error, i18n, r, "roles-error-add");
                    }
                    Err(e) => error.set(Some(
                        i18n.t_args("coa-error-network", &fluent_args!["error" => e.to_string()]),
                    )),
                }
            });
        })
    };

    let on_edit_role_click = {
        let show_edit_modal = show_edit_modal.clone();
        Callback::from(move |role: Role| show_edit_modal.set(Some(role)))
    };

    let on_edit_modal_close = {
        let show_edit_modal = show_edit_modal.clone();
        Callback::from(move |_| show_edit_modal.set(None))
    };

    let on_edit_modal_submit = {
        let fetch_roles = fetch_roles.clone();
        let error = error.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        let show_edit_modal = show_edit_modal.clone();
        Callback::from(move |(role_id, req): (Uuid, UpdateRoleRequest)| {
            let fetch_roles = fetch_roles.clone();
            let error = error.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            let show_edit_modal = show_edit_modal.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("/api/roles/{}", role_id);
                let resp = Api::put(&url, &req, user_ctx, navigator).await;
                match resp {
                    Ok(r) if r.ok() => {
                        show_edit_modal.set(None);
                        fetch_roles.emit(());
                    }
                    Ok(r) => {
                        pages::set_error(error, i18n, r, "roles-error-update");
                    }
                    Err(e) => error.set(Some(
                        i18n.t_args("coa-error-network", &fluent_args!["error" => e.to_string()]),
                    )),
                }
            });
        })
    };

    let on_delete_click = {
        let role_to_delete = role_to_delete.clone();
        Callback::from(move |role: Role| {
            role_to_delete.set(Some(role));
        })
    };

    let on_delete_confirm = {
        let fetch_roles = fetch_roles.clone();
        let error = error.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        let role_to_delete = role_to_delete.clone();
        Callback::from(move |role_id: Uuid| {
            let fetch_roles = fetch_roles.clone();
            let error = error.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            let role_to_delete = role_to_delete.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("/api/roles/{}", role_id);
                let resp = Api::delete(&url, user_ctx, navigator).await;
                role_to_delete.set(None);
                match resp {
                    Ok(r) if r.ok() => {
                        fetch_roles.emit(());
                    }
                    Ok(r) => {
                        pages::set_error(error, i18n, r, "roles-error-delete");
                    }
                    Err(e) => error.set(Some(
                        i18n.t_args("coa-error-network", &fluent_args!["error" => e.to_string()]),
                    )),
                }
            });
        })
    };

    let on_delete_confirm_click = {
        let role_to_delete = role_to_delete.clone();

        Callback::from(move |_| {
            let role_to_delete = role_to_delete.clone();
            let id = role_to_delete.as_ref().unwrap().id;
            on_delete_confirm.emit(id)
        })
    };

    let on_delete_cancel = {
        let role_to_delete = role_to_delete.clone();
        Callback::from(move |()| {
            role_to_delete.set(None);
        })
    };

    html! {
        <Layout>
            <h1>{ i18n.t("roles-title") }</h1>
            <p>{ i18n.t("roles-list-description") }</p>
            <div class="table-actions">
                <button class="button" onclick={on_add_role_click}>
                    { i18n.t("roles-add-button") }
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
                            <th class="table__text-col">{ i18n.t("roles-header-name") }</th>
                            <th class="table__col-actions">{ i18n.t("common-actions") }</th>
                        </tr>
                    </thead>
                    <tbody>
                        { for roles.iter().map(|role| {
                            let role_clone = role.clone();
                            let role_clone2 = role.clone();
                            let on_delete = on_delete_click.clone();
                            let on_edit = on_edit_role_click.clone();
                            html! {
                                <tr key={role.id.to_string()}>
                                    <td>{ &role.name }</td>
                                    <td class="table__col-actions">
                                        <div class="actions-wrapper">
                                            <button class="icon-button btn-action" onclick={move |_| on_edit.emit(role_clone.clone())}>
                                                <img src="/images/edit.svg" alt={i18n.t("common-edit")} />
                                            </button>
                                            <button class="icon-button btn-action" onclick={move |_| on_delete.emit(role_clone2.clone())}>
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
                <AddRoleModal on_close={on_add_modal_close} on_submit={on_add_modal_submit} />
            }
            if let Some(role) = &*show_edit_modal {
                <EditRoleModal role={role.clone()} on_close={on_edit_modal_close} on_submit={on_edit_modal_submit.clone()} />
            }
            if let Some(role) = &*role_to_delete {
                <GenericDeleteConfirmationModal
                    title={i18n.t("delete-role-confirm-title")}
                    message={i18n.t_args("delete-role-confirm-message", &fluent_args!["role" => role.name.clone()])}
                    on_confirm={on_delete_confirm_click}
                    on_cancel={on_delete_cancel}
                />
            }
        </Layout>
    }
}
