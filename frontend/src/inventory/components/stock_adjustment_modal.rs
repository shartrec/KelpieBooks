/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 * (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use std::str::FromStr;

use rust_decimal::Decimal;
use shared_core::{
    inventory::{
        dtos::inventory::{
            AdjustStockItemLine,
            AdjustmentReason,
            StockAdjustmentRequest,
        },
        models::warehouse::{
            Warehouse,
            WarehouseLocation,
        },
    },
    sales::models::item::Item,
    LocationEntryId,
    WarehouseId,
};
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
pub struct StockAdjustmentModalProps {
    pub item: Item,
    pub target_warehouse_id: Option<WarehouseId>,
    pub target_location_id: Option<LocationEntryId>,
    pub on_close: Callback<()>,
    pub on_submit: Callback<()>,
}

#[function_component(StockAdjustmentModal)]
pub fn stock_adjustment_modal(props: &StockAdjustmentModalProps) -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();

    let warehouses = use_state(Vec::<Warehouse>::new);
    let locations = use_state(Vec::<WarehouseLocation>::new);
    let request = use_state(|| StockAdjustmentRequest {
        warehouse_id: WarehouseId::default(),
        items: vec![AdjustStockItemLine {
            location_id: LocationEntryId::default(),
            item_id: props.item.id,
            quantity_delta: Decimal::ZERO,
            reason: AdjustmentReason::CycleCount,
            notes: None,
        }],
    });
    let error = use_state(|| None::<String>);

    // 1. Fetch active warehouses on mount
    {
        let target_warehouse_id = props.target_warehouse_id;
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

                            // Pick explicitly requested warehouse, or fall back to first active
                            let selected_id = target_warehouse_id
                                .filter(|id| active_list.iter().any(|w| w.id == *id))
                                .or_else(|| active_list.first().map(|w| w.id));

                            if let Some(id) = selected_id {
                                let mut req = (*request).clone();
                                req.warehouse_id = id;
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

    // 2. Fetch locations for selected warehouse and pick first one automatically
    {
        let target_location_id = props.target_location_id;
        let locations = locations.clone();
        let request = request.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        let warehouse_id = request.warehouse_id;

        use_effect_with(warehouse_id, move |w_id| {
            let w_id = *w_id;
            if w_id != WarehouseId::default() {
                wasm_bindgen_futures::spawn_local(async move {
                    let url = format!("/api/inventory/warehouses/{}/locations", w_id);
                    if let Ok(resp) = Api::get(&url, user_ctx, navigator).await {
                        if resp.ok() {
                            if let Ok(loc_list) = resp.json::<Vec<WarehouseLocation>>().await {
                                // Pick target location if available in this warehouse, else pick first
                                let selected_loc_id = target_location_id
                                    .filter(|id| loc_list.iter().any(|l| l.id == *id))
                                    .or_else(|| loc_list.first().map(|l| l.id));

                                if let Some(loc_id) = selected_loc_id {
                                    let mut req = (*request).clone();
                                    if let Some(line) = req.items.get_mut(0) {
                                        line.location_id = loc_id;
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
                    Api::post("/api/inventory/adjustments", &*request, user_ctx, navigator).await;

                if resp.is_ok() {
                    on_submit.emit(());
                } else {
                    error.set(Some("Failed to submit stock adjustment.".to_string()));
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
                <h2>{ i18n.t("inventory-adjust-stock-title") }</h2>
                <form onsubmit={on_form_submit} class="modal__form">
                    <label>{ i18n.t("inventory-warehouse-label") }</label>
                    <select
                        value={request.warehouse_id.to_string()}
                        onchange={
                            let state = request.clone();
                            Callback::from(move |e: Event| {
                                let val = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                                if let Ok(id) = WarehouseId::from_str(&val) {
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
                                    if let Ok(loc_id) = LocationEntryId::from_str(&val) {
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

                    <label>{ i18n.t("inventory-reason-label") }</label>
                    <select
                        onchange={
                            let state = request.clone();
                            Callback::from(move |e: Event| {
                                let mut req = (*state).clone();
                                let val = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                                let reason = match val.as_str() {
                                    "DAMAGE" => AdjustmentReason::Damage,
                                    "SCRAP" => AdjustmentReason::Scrap,
                                    "AUDIT_CORRECTION" => AdjustmentReason::AuditCorrection,
                                    "FOUND_STOCK" => AdjustmentReason::FoundStock,
                                    "OTHER" => AdjustmentReason::Other,
                                    _ => AdjustmentReason::CycleCount,
                                };
                                if let Some(line) = req.items.get_mut(0) {
                                    line.reason = reason;
                                }
                                state.set(req);
                            })
                        }
                    >
                        <option value="CYCLE_COUNT">{ i18n.t("inventory-reason-cycle-count") }</option>
                        <option value="DAMAGE">{ i18n.t("inventory-reason-damage") }</option>
                        <option value="SCRAP">{ i18n.t("inventory-reason-scrap") }</option>
                        <option value="AUDIT_CORRECTION">{ i18n.t("inventory-reason-audit") }</option>
                        <option value="FOUND_STOCK">{ i18n.t("inventory-reason-found") }</option>
                        <option value="OTHER">{ i18n.t("inventory-reason-other") }</option>
                    </select>

                    <label>{ i18n.t("inventory-delta-label") }</label>
                    <input
                        type="number"
                        step="0.0001"
                        value={request.items.first().map(|i| i.quantity_delta.to_string()).unwrap_or_default()}
                        oninput={
                            let state = request.clone();
                            Callback::from(move |e: InputEvent| {
                                let mut req = (*state).clone();
                                let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                if let Ok(q) = val.parse::<Decimal>() {
                                    if let Some(line) = req.items.get_mut(0) {
                                        line.quantity_delta = q;
                                    }
                                }
                                state.set(req);
                            })
                        }
                        required=true
                    />

                    <label>{ i18n.t("inventory-notes-label") }</label>
                    <input
                        type="text"
                        value={request.items.first().and_then(|i| i.notes.clone()).unwrap_or_default()}
                        oninput={
                            let state = request.clone();
                            Callback::from(move |e: InputEvent| {
                                let mut req = (*state).clone();
                                let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                if let Some(line) = req.items.get_mut(0) {
                                    line.notes = if val.is_empty() { None } else { Some(val) };
                                }
                                state.set(req);
                            })
                        }
                    />

                    <div class="modal__form__actions">
                        <button type="button" onclick={on_cancel} class="button-secondary">{ i18n.t("common-cancel") }</button>
                        <button type="submit">{ i18n.t("inventory-commit-adjustment") }</button>
                    </div>

                    if let Some(err) = &*error {
                        <div class="error">{ err }</div>
                    }
                </form>
            </div>
        </div>
    }
}
