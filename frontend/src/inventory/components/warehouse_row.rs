/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 * (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use shared_core::{
    core::models::auth::SystemPrivilege,
    inventory::models::warehouse::Warehouse,
};
use yew::prelude::*;

use crate::contexts::{
    auth_context::use_user_context,
    locale_context::use_locale,
};

#[derive(Properties, PartialEq, Clone)]
pub struct WarehouseRowProps {
    pub warehouse: Warehouse,
    pub on_edit: Callback<Warehouse>,
    pub on_delete: Callback<Warehouse>,
}

#[function_component(WarehouseRow)]
pub fn warehouse_row(props: &WarehouseRowProps) -> Html {
    let i18n = use_locale();
    let user_ctx = use_user_context();

    let on_edit = {
        let on_edit = props.on_edit.clone();
        let wh = props.warehouse.clone();
        Callback::from(move |_| {
            on_edit.emit(wh.clone());
        })
    };

    let on_delete = {
        let on_delete = props.on_delete.clone();
        let wh = props.warehouse.clone();
        Callback::from(move |_| {
            on_delete.emit(wh.clone());
        })
    };

    html! {
        <tr>
            <td class="table__text-col">{ &props.warehouse.code }</td>
            <td class="table__text-col">{ &props.warehouse.name }</td>
            <td class="table__text-col"><input type="checkbox" checked={props.warehouse.is_active} disabled=true /></td>
            <td class="table__col-actions">
                <div class="actions-wrapper">
                    { if user_ctx.has_privilege(&SystemPrivilege::ManageInventory) {
                        html! {
                            <>
                                <button class="icon-button btn-action" onclick={on_edit}>
                                    <img src="/images/edit.svg" alt={i18n.t("common-edit")} />
                                </button>
                                <button class="icon-button btn-action" onclick={on_delete}>
                                    <img src="/images/delete.svg" alt={i18n.t("common-delete")} />
                                </button>
                            </>
                        }
                    } else {
                        html!{}
                    }}
                </div>
            </td>
        </tr>
    }
}
