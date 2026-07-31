/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use fluent::fluent_args;
use shared_core::{
    core::models::auth::SystemPrivilege,
    sales::models::item::Item,
};
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[cfg(feature = "inventory")]
use crate::inventory::components::{
    receiving_modal::ReceivingModal,
    stock_adjustment_modal::StockAdjustmentModal,
};
use crate::{
    api::Api,
    contexts::{
        auth_context::use_user_context,
        locale_context::use_locale,
    },
    sales::{
        components::{
            add_item_modal::AddItemModal,
            edit_item_modal::EditItemModal,
            item_row::ItemRow,
        },
        contexts::item_filter_context::use_item_filter,
    },
};

#[function_component(ItemListTable)]
pub fn item_list_table() -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();
    let filter_ctx = use_item_filter();
    let items = use_state(Vec::new);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);
    let show_add_modal = use_state(|| false);
    let item_to_edit = use_state(|| None::<Item>);

    #[cfg(feature = "inventory")]
    let item_to_receive = use_state(|| None::<Item>);
    #[cfg(feature = "inventory")]
    let item_to_adjust = use_state(|| None::<Item>);

    let fetch_items = {
        let items = items.clone();
        let error = error.clone();
        let loading = loading.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        let filter_ctx = filter_ctx.clone();
        Callback::from(move |_: ()| {
            let items = items.clone();
            let error = error.clone();
            let loading = loading.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            let filter_ctx = filter_ctx.clone();
            loading.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                let mut url = format!("/api/sales/items?limit={}", filter_ctx.limit);
                if !filter_ctx.search_term.is_empty() {
                    url.push_str(&format!("&search_term={}", filter_ctx.search_term));
                }
                if let Some(item_type) = &filter_ctx.item_type {
                    url.push_str(&format!("&item_type={}", item_type));
                }
                if filter_ctx.include_inactive {
                    url.push_str("&include_inactive=true");
                }

                let fetched_items = Api::get(&url, user_ctx, navigator).await;
                loading.set(false);
                match fetched_items {
                    Ok(response) if response.ok() => match response.json::<Vec<Item>>().await {
                        Ok(data) => items.set(data),
                        Err(e) => error.set(Some(i18n.t_args(
                            "item-list-error-parse-items",
                            &fluent_args!["error" => e.to_string()],
                        ))),
                    },
                    Ok(response) => error.set(Some(i18n.t_args(
                        "item-list-error-fetch-items",
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

    let fetch_items_clone = fetch_items.clone();
    use_effect_with(
        (
            filter_ctx.search_term.clone(),
            filter_ctx.item_type.clone(),
            filter_ctx.include_inactive,
            filter_ctx.limit,
        ),
        move |_| {
            fetch_items_clone.emit(());
            || ()
        },
    );

    let on_add_click = {
        let show_add_modal = show_add_modal.clone();
        Callback::from(move |_| show_add_modal.set(true))
    };

    let on_edit_click = {
        let item_to_edit = item_to_edit.clone();
        Callback::from(move |item: Item| {
            item_to_edit.set(Some(item));
        })
    };

    #[cfg(feature = "inventory")]
    let on_receive_click = {
        let item_to_receive = item_to_receive.clone();
        Callback::from(move |item: Item| {
            item_to_receive.set(Some(item));
        })
    };

    #[cfg(feature = "inventory")]
    let on_adjust_click = {
        let item_to_adjust = item_to_adjust.clone();
        Callback::from(move |item: Item| {
            item_to_adjust.set(Some(item));
        })
    };

    let on_modal_close = {
        let show_add_modal = show_add_modal.clone();
        let item_to_edit = item_to_edit.clone();

        #[cfg(feature = "inventory")]
        let item_to_receive = item_to_receive.clone();
        #[cfg(feature = "inventory")]
        let item_to_adjust = item_to_adjust.clone();

        Callback::from(move |_: ()| {
            show_add_modal.set(false);
            item_to_edit.set(None);

            #[cfg(feature = "inventory")]
            {
                item_to_receive.set(None);
                item_to_adjust.set(None);
            }
        })
    };

    let on_submit = {
        let fetch_items = fetch_items.clone();
        let on_modal_close = on_modal_close.clone();
        Callback::from(move |_: ()| {
            fetch_items.emit(());
            on_modal_close.emit(());
        })
    };

    if *loading {
        return html! { <p>{ i18n.t("common-loading") }</p> };
    }
    if let Some(err) = &*error {
        return html! { <div class="error">{ err }</div> };
    }

    // Pre-render inventory modals outside html! macro
    let inventory_modals = {
        #[cfg(feature = "inventory")]
        if true {
            html! {
                <>
                    { if let Some(item) = &*item_to_receive {
                        html! { <ReceivingModal item={item.clone()} on_close={on_modal_close.clone()} on_submit={on_submit.clone()} /> }
                    } else {
                        html! {}
                    }}
                    { if let Some(item) = &*item_to_adjust {
                        html! { <StockAdjustmentModal item={item.clone()} on_close={on_modal_close.clone()} on_submit={on_submit.clone()} /> }
                    } else {
                        html! {}
                    }}
                </>
            }
        } else {
            html! {}
        }

        #[cfg(not(feature = "inventory"))]
        html! {}
    };

    html! {
        <>
            <div class="table-actions">
                { if user_ctx.has_privilege(&SystemPrivilege::ManageSales) {
                    html! {
                        <button onclick={on_add_click}>{ i18n.t("item-list-add-item-button") }</button>
                    }
                } else {
                    html! {}
                }}
            </div>

            if *show_add_modal {
                <AddItemModal on_close={on_modal_close.clone()} on_submit={on_submit.clone()} />
            }
            if let Some(item) = &*item_to_edit {
                <EditItemModal item={item.clone()} on_close={on_modal_close.clone()} on_submit={on_submit.clone()} />
            }

            { inventory_modals }

            <table class="table">
                <thead>
                    <tr>
                        <th class="table__text-col">{ i18n.t("item-list-code") }</th>
                        <th class="table__text-col">{ i18n.t("item-list-name") }</th>
                        <th class="table__text-col">{ i18n.t("item-list-type") }</th>
                        <th class="table__value-col">{ i18n.t("item-list-price") }</th>
                        <th class="table__col-actions">{ i18n.t("common-actions") }</th>
                    </tr>
                </thead>
                <tbody>
                    { for (*items).iter().map(|item| {
                        #[cfg(feature = "inventory")]
                        {
                            html! {
                                <ItemRow
                                    item={item.clone()}
                                    on_edit={on_edit_click.clone()}
                                    on_receive={Some(on_receive_click.clone())}
                                    on_adjust={Some(on_adjust_click.clone())}
                                />
                            }
                        }
                        #[cfg(not(feature = "inventory"))]
                        {
                            html! {
                                <ItemRow
                                    item={item.clone()}
                                    on_edit={on_edit_click.clone()}
                                />
                            }
                        }
                    })}
                </tbody>
            </table>
        </>
    }
}
