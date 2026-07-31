/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use shared_core::{
    core::models::auth::SystemPrivilege,
    sales::models::item::Item,
};
use yew::prelude::*;

use crate::contexts::{
    auth_context::use_user_context,
    locale_context::use_locale,
};

#[derive(Properties, PartialEq, Clone)]
pub struct ItemRowProps {
    pub item: Item,
    pub on_edit: Callback<Item>,
    #[cfg(feature = "inventory")]
    pub on_receive: Option<Callback<Item>>,
    #[cfg(feature = "inventory")]
    pub on_adjust: Option<Callback<Item>>,
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
    let inventory_actions = {
        #[cfg(feature = "inventory")]
        if user_ctx.has_privilege(&SystemPrivilege::ManageInventory) {
            let on_receive = {
                let on_receive = props.on_receive.clone();
                let item = props.item.clone();
                Callback::from(move |_| {
                    if let Some(cb) = &on_receive {
                        cb.emit(item.clone());
                    }
                })
            };

            let on_adjust = {
                let on_adjust = props.on_adjust.clone();
                let item = props.item.clone();
                Callback::from(move |_| {
                    if let Some(cb) = &on_adjust {
                        cb.emit(item.clone());
                    }
                })
            };

            html! {
                <>
                    <button class="icon-button btn-action" onclick={on_receive} title={i18n.t("inventory-receive-stock")}>
                        <img src="/images/receive.svg" alt={i18n.t("inventory-receive-stock")} />
                    </button>
                    <button class="icon-button btn-action" onclick={on_adjust} title={i18n.t("inventory-adjust-stock")}>
                        <img src="/images/adjust.svg" alt={i18n.t("inventory-adjust-stock")} />
                    </button>
                </>
            }
        } else {
            html! {}
        }

        #[cfg(not(feature = "inventory"))]
        html! {}
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
                            <button class="icon-button btn-action" onclick={on_edit} title={i18n.t("common-edit")}>
                                <img src="/images/edit.svg" alt={i18n.t("common-edit")} />
                            </button>
                        }
                    } else {
                        html!{}
                    }}
                    { inventory_actions }
                </div>
            </td>
        </tr>
    }
}
