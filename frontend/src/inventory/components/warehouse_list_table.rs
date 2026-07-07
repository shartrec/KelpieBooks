/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 * (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use fluent::fluent_args;
use shared_core::{
    core::models::auth::SystemPrivilege,
    inventory::models::warehouse::Warehouse,
};
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{
    api::Api,
    contexts::{
        auth_context::use_user_context,
        locale_context::use_locale,
    },
    core::components::delete_confirmation_modal::DeleteConfirmationModal,
    inventory::components::{
        warehouse_modal::WarehouseModal,
        warehouse_row::WarehouseRow,
    },
};

#[function_component(WarehouseListTable)]
pub fn warehouse_list_table() -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();
    let warehouses = use_state(Vec::new);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);
    let show_add_modal = use_state(|| false);
    let wh_to_edit = use_state(|| None::<Warehouse>);
    let wh_to_delete = use_state(|| None::<Warehouse>);

    let fetch_warehouses = {
        let warehouses = warehouses.clone();
        let error = error.clone();
        let loading = loading.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        Callback::from(move |_: ()| {
            let warehouses = warehouses.clone();
            let error = error.clone();
            let loading = loading.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            loading.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                let fetched = Api::get("/api/inventory/warehouses", user_ctx, navigator).await;
                loading.set(false);
                match fetched {
                    Ok(response) if response.ok() => {
                        match response.json::<Vec<Warehouse>>().await {
                            Ok(data) => warehouses.set(data),
                            Err(e) => error.set(Some(i18n.t_args(
                                "warehouse-list-error-parse",
                                &fluent_args!["error" => e.to_string()],
                            ))),
                        }
                    }
                    Ok(response) => error.set(Some(i18n.t_args(
                        "warehouse-list-error-fetch",
                        &fluent_args!["status" => response.status()],
                    ))),
                    Err(e) => error.set(Some(i18n.t_args(
                        "common-network-error",
                        &fluent_args!["error" => e.to_string()],
                    ))),
                }
            });
        })
    };

    let fetch_warehouses_clone = fetch_warehouses.clone();
    use_effect_with((), move |()| {
        fetch_warehouses_clone.emit(());
        || ()
    });

    let on_add_click = {
        let show_add_modal = show_add_modal.clone();
        Callback::from(move |_| show_add_modal.set(true))
    };

    let on_edit_click = {
        let wh_to_edit = wh_to_edit.clone();
        Callback::from(move |wh: Warehouse| {
            wh_to_edit.set(Some(wh));
        })
    };

    let on_delete_click = {
        let wh_to_delete = wh_to_delete.clone();
        Callback::from(move |wh: Warehouse| {
            wh_to_delete.set(Some(wh));
        })
    };

    let on_modal_close = {
        let show_add_modal = show_add_modal.clone();
        let wh_to_edit = wh_to_edit.clone();
        let wh_to_delete = wh_to_delete.clone();
        Callback::from(move |_: ()| {
            show_add_modal.set(false);
            wh_to_edit.set(None);
            wh_to_delete.set(None);
        })
    };

    let on_submit = {
        let fetch_warehouses = fetch_warehouses.clone();
        let on_modal_close = on_modal_close.clone();
        Callback::from(move |_: ()| {
            fetch_warehouses.emit(());
            on_modal_close.emit(());
        })
    };

    let on_delete_confirm = {
        let on_submit = on_submit.clone();
        let on_modal_close = on_modal_close.clone();
        let wh_to_delete = wh_to_delete.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        let error = error.clone();
        let i18n = i18n.clone();

        Callback::from(move |_| {
            let on_submit = on_submit.clone();
            let wh_to_delete = wh_to_delete.clone();
            let on_modal_close = on_modal_close.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            let error = error.clone();
            let i18n = i18n.clone();

            if let Some(wh) = &*wh_to_delete {
                let wh_id = wh.id;
                wasm_bindgen_futures::spawn_local(async move {
                    let resp =
                        Api::delete(&format!("/api/inventory/warehouses/{}", wh_id), user_ctx, navigator)
                            .await;
                    match resp {
                        Ok(r) if r.ok() => {
                            on_submit.emit(());
                        }
                        Ok(r) => {
                            if r.status() == 409 {
                                error.set(Some(i18n.t("warehouse-delete-error-conflict")));
                            } else {
                                error.set(Some(i18n.t("warehouse-delete-error")));
                            }
                            on_modal_close.emit(());
                        }
                        Err(e) => {
                            error.set(Some(i18n.t_args(
                                "common-network-error",
                                &fluent_args!["error" => e.to_string()],
                            )));
                            on_modal_close.emit(());
                        }
                    }
                });
            }
        })
    };

    if *loading {
        return html! { <p>{ i18n.t("common-loading") }</p> };
    }

    html! {
        <>
            <div class="table-actions">
                { if user_ctx.has_privilege(&SystemPrivilege::ManageInventory) {
                    html! {
                        <button onclick={on_add_click}>{ i18n.t("warehouse-list-add-button") }</button>
                    }
                } else {
                    html! {}
                }}
            </div>

            if let Some(err) = &*error {
                <div class="message__error">{ err }</div>
            }
            <table class="table">
                <thead>
                    <tr>
                        <th class="table__text-col">{ i18n.t("warehouse-list-code") }</th>
                        <th class="table__text-col">{ i18n.t("warehouse-list-name") }</th>
                        <th class="table__text-col">{ i18n.t("warehouse-list-is-active") }</th>
                        <th class="table__text-col">{ i18n.t("common-actions") }</th>
                    </tr>
                </thead>
                <tbody>
                    { for (*warehouses).iter().map(|wh| html! {
                        <WarehouseRow
                            warehouse={wh.clone()}
                            on_edit={on_edit_click.clone()}
                            on_delete={on_delete_click.clone()}
                        />
                    })}
                </tbody>
            </table>

            if *show_add_modal {
                <WarehouseModal warehouse={None} on_close={on_modal_close.clone()} on_submit={on_submit.clone()} />
            }
            if let Some(wh) = &*wh_to_edit {
                <WarehouseModal warehouse={Some(wh.clone())} on_close={on_modal_close.clone()} on_submit={on_submit.clone()} />
            }
            if let Some(wh) = &*wh_to_delete {
                <DeleteConfirmationModal
                    title={i18n.t("warehouse-delete-title")}
                    message={i18n.t_args("warehouse-delete-confirm-message", &fluent_args!["name" => wh.name.clone()])}
                    on_confirm={on_delete_confirm}
                    on_cancel={on_modal_close}
                />
            }
        </>
    }
}