/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use rust_decimal::Decimal;
use shared_core::{
    core::models::auth::SystemPrivilege,
    inventory::dtos::inventory::ItemStockBalancesResponse,
    sales::models::item::Item,
    LocationEntryId,
    WarehouseId,
};
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{
    api::Api,
    contexts::{
        auth_context::use_user_context,
        locale_context::use_locale,
    },
};

#[derive(Properties, PartialEq, Clone)]
pub struct ItemRowProps {
    pub item: Item,
    pub on_edit: Callback<Item>,
    #[cfg(feature = "inventory")]
    pub on_receive: Option<Callback<(Item, Option<WarehouseId>, Option<LocationEntryId>)>>,
    #[cfg(feature = "inventory")]
    pub on_adjust: Option<Callback<(Item, Option<WarehouseId>, Option<LocationEntryId>)>>,
}

#[function_component(ItemRow)]
pub fn item_row(props: &ItemRowProps) -> Html {
    let i18n = use_locale();
    let user_ctx = use_user_context();
    let navigator = use_navigator().unwrap();

    let is_collapsed = use_state(|| true);
    let balances = use_state(|| Option::<ItemStockBalancesResponse>::None);

    let on_edit = {
        let on_edit = props.on_edit.clone();
        let item = props.item.clone();
        Callback::from(move |_| {
            on_edit.emit(item.clone());
        })
    };

    let on_toggle_collapse = {
        let is_collapsed = is_collapsed.clone();
        let balances = balances.clone();
        let item_id = props.item.id;
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();

        Callback::from(move |_| {
            let new_state = !*is_collapsed;
            is_collapsed.set(new_state);

            // Lazily fetch breakdown if opening for the first time
            if !new_state && balances.is_none() {
                let balances = balances.clone();
                let user_ctx = user_ctx.clone();
                let navigator = navigator.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let url = format!("/api/inventory/items/{}/balances", item_id);
                    if let Ok(resp) = Api::get(&url, user_ctx, navigator).await {
                        if resp.ok() {
                            if let Ok(data) = resp.json::<ItemStockBalancesResponse>().await {
                                balances.set(Some(data));
                            }
                        }
                    }
                });
            }
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
                        cb.emit((item.clone(), None, None));
                    }
                })
            };

            let on_adjust = {
                let on_adjust = props.on_adjust.clone();
                let item = props.item.clone();
                Callback::from(move |_| {
                    if let Some(cb) = &on_adjust {
                        cb.emit((item.clone(), None, None));
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
    <>
        <tr>
            <td>
                <button onclick={on_toggle_collapse} class="collapse-toggle">
                    if *is_collapsed {
                        <img src="/images/chevron-right.svg" alt={i18n.t("common-expand")} />
                    } else {
                        <img src="/images/chevron-down.svg" alt={i18n.t("common-collapse")} />
                    }
                    </button>
                { &props.item.code }
            </td>
            <td class="table__text-col">{ &props.item.name }</td>
            <td class="table__text-col">{ format!("{:?}", props.item.item_type) }</td>
            <td class="table__value-col">{ i18n.format_currency(props.item.unit_price) }</td>
            <td class="table__value-col">{ i18n.format_currency(props.item.unit_cost) }</td>
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

        // Expandable Sub-Row (Only rendered when expanded and item is Stocked) -->
        if !(*is_collapsed) && props.item.is_stocked() {
            <tr class="table__sub-row">
                <td class="stock-breakdown-row" colspan="99">
                    <div class="stock-breakdown-container">
                        if let Some(response) = &*balances {
                            if response.location_balances.is_empty() {
                                <p class="text-muted">{ i18n.t("inventory-no-stock-found") }</p>
                            } else {
                                <table class="sub-table">
                                    <thead>
                                        <tr>
                                            <th class="table__text-col">{ i18n.t("inventory-warehouse-label") }</th>
                                            <th class="table__text-col">{ i18n.t("inventory-location-label") }</th>
                                            <th class="table__value-col">{ i18n.t("inventory-on-hand-label") }</th>
                                            <th class="table__value-col">{ i18n.t("inventory-allocated-label") }</th>
                                            <th class="table__value-col">{ i18n.t("inventory-available-label") }</th>
                                            <th class="table__col-actions">{ i18n.t("common-actions") }</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        { for response.location_balances.iter().map(|b| {
                                                    let warehouse_id = b.warehouse_id;
                                                    let location_id = b.location_id;

                                                    let on_sub_receive = {
                                                        let on_receive = props.on_receive.clone();
                                                        let item = props.item.clone();
                                                        Callback::from(move |_| {
                                                            if let Some(cb) = &on_receive {
                                                                cb.emit((item.clone(), Some(warehouse_id), Some(location_id)));
                                                            }
                                                        })
                                                    };

                                                    let on_sub_adjust = {
                                                        let on_adjust = props.on_adjust.clone();
                                                        let item = props.item.clone();
                                                        Callback::from(move |_| {
                                                            if let Some(cb) = &on_adjust {
                                                                cb.emit((item.clone(), Some(warehouse_id), Some(location_id)));
                                                            }
                                                        })
                                                    };

                                            html! {
                                                <tr>
                                                    <td class="table__text-col">{ &b.warehouse_name }</td>
                                                    <td class="table__text-col">{ &b.location_display_label }</td>
                                                    <td class="table__value-col">{ i18n.format_decimal(b.quantity_on_hand.unwrap_or(Decimal::ZERO)) }</td>
                                                    <td class="table__value-col">{ i18n.format_decimal(b.quantity_allocated.unwrap_or(Decimal::ZERO)) }</td>
                                                    <td class="table__value-col"><strong>{ i18n.format_decimal(b.quantity_on_hand.unwrap_or(Decimal::ZERO) - b.quantity_allocated.unwrap_or(Decimal::ZERO)) }</strong></td>
                                                    <td class="table__col-actions">
                                                        { if user_ctx.has_privilege(&SystemPrivilege::ManageInventory) {
                                                            html! {
                                                                <div class="actions-wrapper">
                                                                    <button class="icon-button btn-action" onclick={on_sub_receive} title={i18n.t("inventory-receive-stock")}>
                                                                        <img src="/images/receive.svg" alt={i18n.t("inventory-receive-stock")} />
                                                                    </button>
                                                                    <button class="icon-button btn-action" onclick={on_sub_adjust} title={i18n.t("inventory-adjust-stock")}>
                                                                        <img src="/images/adjust.svg" alt={i18n.t("inventory-adjust-stock")} />
                                                                    </button>
                                                                </div>
                                                            }
                                                        } else {
                                                            html! {}
                                                        }}
                                                    </td>
                                                </tr>
                                            }
                                        }) }
                                    </tbody>
                                </table>
                            }
                        } else {
                            <div class="spinner">{ i18n.t("common-loading") }</div>
                        }
                    </div>
                </td>
            </tr>
        }
        </>
    }
}
