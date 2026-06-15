/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use yew::prelude::*;
use shared_core::sales::models::item::Item;
use crate::contexts::locale_context::use_locale;
use shared_core::core::models::auth::SystemPrivilege;
use crate::contexts::auth_context::use_user_context;

#[derive(Properties, PartialEq, Clone)]
pub struct ItemRowProps {
    pub item: Item,
    pub on_edit: Callback<Item>,
}

#[function_component(ItemRow)]
pub fn item_row(props: &ItemRowProps) -> Html {
    let i18n = use_locale();
    let user_ctx = use_user_context();

    let on_edit = {
        let on_edit = props.on_edit.clone();
        let item = props.item.clone();
        Callback::from(move |_| {
            on_edit.emit(item.clone());
        })
    };

    html! {
        <tr>
            <td class="table__text-col">{ &props.item.code }</td>
            <td class="table__text-col">{ &props.item.name }</td>
            <td class="table__text-col">{ format!("{:?}", props.item.item_type) }</td>
            <td class="table__value-col">{ i18n.format_currency(props.item.unit_price) }</td>
            <td class="table__col-actions">
                <div class="actions-wrapper">
                    { if user_ctx.has_privilege(&SystemPrivilege::ManageSales) {
                        html! {
                            <button class="icon-button btn-action" onclick={on_edit}>
                                <img src="/images/edit.svg" alt={i18n.t("common-edit")} />
                            </button>
                        }
                    } else {
                        html!{}
                    }}
                </div>
            </td>
        </tr>
    }
}
