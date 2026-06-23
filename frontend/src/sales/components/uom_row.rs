/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use shared_core::{
    core::models::auth::SystemPrivilege,
    sales::models::item::UnitOfMeasure,
};
use yew::prelude::*;

use crate::contexts::{
    auth_context::use_user_context,
    locale_context::use_locale,
};

#[derive(Properties, PartialEq, Clone)]
pub struct UomRowProps {
    pub uom: UnitOfMeasure,
    pub on_edit: Callback<UnitOfMeasure>,
    pub on_delete: Callback<UnitOfMeasure>,
}

#[function_component(UomRow)]
pub fn uom_row(props: &UomRowProps) -> Html {
    let i18n = use_locale();
    let user_ctx = use_user_context();

    let on_edit = {
        let on_edit = props.on_edit.clone();
        let uom = props.uom.clone();
        Callback::from(move |_| {
            on_edit.emit(uom.clone());
        })
    };

    let on_delete = {
        let on_delete = props.on_delete.clone();
        let uom = props.uom.clone();
        Callback::from(move |_| {
            on_delete.emit(uom.clone());
        })
    };

    html! {
        <tr>
            <td class="table__text-col">{ &props.uom.code }</td>
            <td class="table__text-col">{ &props.uom.name }</td>
            <td class="table__text-col"><input type="checkbox" checked={props.uom.is_active} disabled=true /></td>
            <td class="table__col-actions">
                <div class="actions-wrapper">
                    { if user_ctx.has_privilege(&SystemPrivilege::ManageSales) {
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
