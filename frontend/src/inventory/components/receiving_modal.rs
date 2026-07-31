/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 * (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use rust_decimal::Decimal;
use shared_core::{
    inventory::{
        dtos::inventory::{
            ReceiveItemLine,
            ReceiveStockRequest,
        },
        models::warehouse::{
            Warehouse,
            WarehouseLocation,
        },
    },
    sales::models::item::Item,
};
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{
    api::Api,
    contexts::{
        auth_context::use_user_context,
        locale_context::use_locale,
    },
};

#[derive(Properties, PartialEq, Clone)]
pub struct ReceivingModalProps {
    pub item: Item,
    pub on_close: Callback<()>,
    pub on_submit: Callback<()>,
}

#[function_component(ReceivingModal)]
pub fn receiving_modal(props: &ReceivingModalProps) -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();

    let warehouses = use_state(Vec::<Warehouse>::new);
    let locations = use_state(Vec::<WarehouseLocation>::new);
    let request = use_state(|| ReceiveStockRequest {
        warehouse_id: Uuid::nil(),
        vendor_id: None,
        po_number: None,
        notes: None,
        items: vec![ReceiveItemLine {
            item_id: props.item.id,
            location_id: Uuid::nil(),
            quantity: Decimal::ONE,
        }],
    });
    let error = use_state(|| None::<String>);

    // 1. Fetch active warehouses on component mount
    {
        let warehouses = warehouses.clone();
        let request = request.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(resp) = Api::get("/api/inventory/warehouses", user_ctx, navigator).await {
                    if resp.ok() {
                        if let Ok(list) = resp.json::<Vec<Warehouse>>().await {
                            let active_list: Vec<Warehouse> =
                                list.into_iter().filter(|w| w.is_active).collect();
                            if let Some(first) = active_list.first() {
                                let mut req = (*request).clone();
                                req.warehouse_id = first.id;
                                request.set(req);
                            }
                            warehouses.set(active_list);
                        }
                    }
                }
            });
            || ()
        });
    }

    // 2. Fetch locations for the selected warehouse and pick the first one by default
    {
        let locations = locations.clone();
        let request = request.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        let warehouse_id = request.warehouse_id;

        use_effect_with(warehouse_id, move |w_id| {
            let w_id = *w_id;
            if w_id != Uuid::nil() {
                wasm_bindgen_futures::spawn_local(async move {
                    let url = format!("/api/inventory/warehouses/{}/locations", w_id);
                    if let Ok(resp) = Api::get(&url, user_ctx, navigator).await {
                        if resp.ok() {
                            if let Ok(loc_list) = resp.json::<Vec<WarehouseLocation>>().await {
                                if let Some(first_loc) = loc_list.first() {
                                    let mut req = (*request).clone();
                                    if let Some(line) = req.items.get_mut(0) {
                                        line.location_id = first_loc.id;
                                    }
                                    request.set(req);
                                }
                                locations.set(loc_list);
                            }
                        }
                    }
                });
            }
            || ()
        });
    }

    let on_form_submit = {
        let on_submit = props.on_submit.clone();
        let request = request.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        let error = error.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let on_submit = on_submit.clone();
            let request = request.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            let error = error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let resp =
                    Api::post("/api/inventory/receiving", &*request, user_ctx, navigator).await;

                if resp.is_ok() {
                    on_submit.emit(());
                } else {
                    error.set(Some("Failed to process receipt.".to_string()));
                }
            });
        })
    };

    let on_cancel = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| {
            on_close.emit(());
        })
    };

    html! {
        <div class="modal-overlay" onclick={on_cancel.clone()}>
            <div class="modal-content" onclick={|e: MouseEvent| e.stop_propagation()}>
                <h2>{ i18n.t("inventory-receive-stock-title") }</h2>
                <form onsubmit={on_form_submit} class="modal__form">
                    <label>{ i18n.t("inventory-warehouse-label") }</label>
                    <select
                        value={request.warehouse_id.to_string()}
                        onchange={
                            let state = request.clone();
                            Callback::from(move |e: Event| {
                                let val = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                                if let Ok(id) = Uuid::parse_str(&val) {
                                    let mut req = (*state).clone();
                                    req.warehouse_id = id;
                                    state.set(req);
                                }
                            })
                        }
                        required=true
                    >
                        { for (*warehouses).iter().map(|w| {
                            html! { <option value={w.id.to_string()}>{ &w.name }</option> }
                        })}
                    </select>

                    if !locations.is_empty() {
                        <label>{ i18n.t("inventory-location-label") }</label>
                        <select
                            value={request.items.first().map(|i| i.location_id.to_string()).unwrap_or_default()}
                            onchange={
                                let state = request.clone();
                                Callback::from(move |e: Event| {
                                    let val = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                                    if let Ok(loc_id) = Uuid::parse_str(&val) {
                                        let mut req = (*state).clone();
                                        if let Some(line) = req.items.get_mut(0) {
                                            line.location_id = loc_id;
                                        }
                                        state.set(req);
                                    }
                                })
                            }
                            required=true
                        >
                            { for (*locations).iter().map(|l| {
                                html! { <option value={l.id.to_string()}>{ &l.display_label }</option> }
                            })}
                        </select>
                    }

                    <label>{ i18n.t("inventory-item-label") }</label>
                    <input type="text" value={props.item.name.clone()} disabled=true />

                    <label>{ i18n.t("inventory-po-number-label") }</label>
                    <input
                        type="text"
                        value={request.po_number.clone().unwrap_or_default()}
                        oninput={
                            let state = request.clone();
                            Callback::from(move |e: InputEvent| {
                                let mut req = (*state).clone();
                                let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                req.po_number = if val.is_empty() { None } else { Some(val) };
                                state.set(req);
                            })
                        }
                    />

                    <label>{ i18n.t("inventory-quantity-label") }</label>
                    <input
                        type="number"
                        step="0.0001"
                        min="0.0001"
                        value={request.items.first().map(|i| i.quantity.to_string()).unwrap_or_default()}
                        oninput={
                            let state = request.clone();
                            Callback::from(move |e: InputEvent| {
                                let mut req = (*state).clone();
                                let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                if let Ok(q) = val.parse::<Decimal>() {
                                    if let Some(line) = req.items.get_mut(0) {
                                        line.quantity = q;
                                    }
                                }
                                state.set(req);
                            })
                        }
                        required=true
                    />

                    <div class="modal__form__actions">
                        <button type="button" onclick={on_cancel} class="button-secondary">{ i18n.t("common-cancel") }</button>
                        <button type="submit">{ i18n.t("inventory-post-receipt") }</button>
                    </div>

                    if let Some(err) = &*error {
                        <div class="error">{ err }</div>
                    }
                </form>
            </div>
        </div>
    }
}
