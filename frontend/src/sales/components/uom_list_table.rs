/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use yew::prelude::*;
use crate::api::Api;
use crate::contexts::auth_context::use_user_context;
use crate::contexts::locale_context::use_locale;
use fluent::fluent_args;
use shared_core::sales::models::item::UnitOfMeasure;
use yew_router::prelude::use_navigator;
use shared_core::core::models::auth::SystemPrivilege;
use crate::sales::components::uom_row::UomRow;
use crate::sales::components::add_uom_modal::AddUomModal;
use crate::sales::components::edit_uom_modal::EditUomModal;
use crate::core::components::delete_confirmation_modal::DeleteConfirmationModal;

#[function_component(UomListTable)]
pub fn uom_list_table() -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();
    let uoms = use_state(Vec::new);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);
    let show_add_modal = use_state(|| false);
    let uom_to_edit = use_state(|| None::<UnitOfMeasure>);
    let uom_to_delete = use_state(|| None::<UnitOfMeasure>);

    let fetch_uoms = {
        let uoms = uoms.clone();
        let error = error.clone();
        let loading = loading.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        Callback::from(move |_: ()| {
            let uoms = uoms.clone();
            let error = error.clone();
            let loading = loading.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            loading.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_uoms = Api::get("/api/sales/uoms", user_ctx, navigator).await;
                loading.set(false);
                match fetched_uoms {
                    Ok(response) if response.ok() => {
                        match response.json::<Vec<UnitOfMeasure>>().await {
                            Ok(data) => uoms.set(data),
                            Err(e) => error.set(Some(i18n.t_args(
                                "uom-list-error-parse-uoms",
                                &fluent_args!["error" => e.to_string()],
                            ))),
                        }
                    }
                    Ok(response) => error.set(Some(i18n.t_args(
                        "uom-list-error-fetch-uoms",
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

    let fetch_uoms_clone = fetch_uoms.clone();
    use_effect_with((), move |()| {
        fetch_uoms_clone.emit(());
        || ()
    });

    let on_add_click = {
        let show_add_modal = show_add_modal.clone();
        Callback::from(move |_| show_add_modal.set(true))
    };

    let on_edit_click = {
        let uom_to_edit = uom_to_edit.clone();
        Callback::from(move |uom: UnitOfMeasure| {
            uom_to_edit.set(Some(uom));
        })
    };

    let on_delete_click = {
        let uom_to_delete = uom_to_delete.clone();
        Callback::from(move |uom: UnitOfMeasure| {
            uom_to_delete.set(Some(uom));
        })
    };

    let on_modal_close = {
        let show_add_modal = show_add_modal.clone();
        let uom_to_edit = uom_to_edit.clone();
        let uom_to_delete = uom_to_delete.clone();
        Callback::from(move |_: ()| {
            show_add_modal.set(false);
            uom_to_edit.set(None);
            uom_to_delete.set(None);
        })
    };

    let on_submit = {
        let fetch_uoms = fetch_uoms.clone();
        let on_modal_close = on_modal_close.clone();
        Callback::from(move |_: ()| {
            fetch_uoms.emit(());
            on_modal_close.emit(());
        })
    };

    let on_delete_confirm = {
        let on_submit = on_submit.clone();
        let on_modal_close = on_modal_close.clone();
        let uom_to_delete = uom_to_delete.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        let error = error.clone();
        let i18n = i18n.clone();

        Callback::from(move |_| {
            let on_submit = on_submit.clone();
            let uom_to_delete = uom_to_delete.clone();
            let on_modal_close = on_modal_close.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            let error = error.clone();
            let i18n = i18n.clone();

            if let Some(uom) = &*uom_to_delete {
                let uom_id = uom.id;
                wasm_bindgen_futures::spawn_local(async move {
                    let resp = Api::delete(&format!("/api/sales/uoms/{}", uom_id), user_ctx, navigator).await;
                    match resp {
                        Ok(r) if r.ok() => {
                            on_submit.emit(());
                        }
                        Ok(_r) => {
                            error.set(Some(i18n.t("uom-delete-error")));
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
                { if user_ctx.has_privilege(&SystemPrivilege::ManageSales) {
                    html! {
                        <button onclick={on_add_click}>{ i18n.t("uom-list-add-uom-button") }</button>
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
                        <th class="table__text-col">{ i18n.t("uom-list-code") }</th>
                        <th class="table__text-col">{ i18n.t("uom-list-name") }</th>
                        <th class="table__text-col">{ i18n.t("uom-list-is-active") }</th>
                        <th class="table__text-col">{ i18n.t("common-actions") }</th>
                    </tr>
                </thead>
                <tbody>
                    { for (*uoms).iter().map(|uom| html! {
                        <UomRow
                            uom={uom.clone()}
                            on_edit={on_edit_click.clone()}
                            on_delete={on_delete_click.clone()}
                        />
                    })}
                </tbody>
            </table>

            if *show_add_modal {
                <AddUomModal on_close={on_modal_close.clone()} on_submit={on_submit.clone()} />
            }
            if let Some(uom) = &*uom_to_edit {
                <EditUomModal uom={uom.clone()} on_close={on_modal_close.clone()} on_submit={on_submit.clone()} />
            }
            if let Some(uom) = &*uom_to_delete {
                <DeleteConfirmationModal
                    title={i18n.t("uom-delete-title")}
                    message={i18n.t_args("uom-delete-confirm-message", &fluent_args!["name" => uom.name.clone()])}
                    on_confirm={on_delete_confirm}
                    on_cancel={on_modal_close}
                />
            }

        </>
    }
}
