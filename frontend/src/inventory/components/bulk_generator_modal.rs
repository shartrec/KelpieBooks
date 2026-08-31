/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 * (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use shared_core::{
    inventory::dtos::inventory::{
        AlphaRange,
        BulkLocationGenerateRequest,
        NumericRange,
    },
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
pub struct GeneratorProps {
    pub warehouse_id: WarehouseId,
    pub on_close: Callback<()>,
    pub on_submit: Callback<()>,
}

#[function_component(BulkGeneratorModal)]
pub fn bulk_generator_modal(props: &GeneratorProps) -> Html {
    let i18n = use_locale();
    let user_ctx = use_user_context();
    let navigator = use_navigator().unwrap();

    // Setup working form data bindings mirroring the JSON structural contract
    let zone = use_state(String::new);
    let is_picking = use_state(|| true);
    let aisle_start = use_state(|| 1);
    let aisle_end = use_state(|| 5);
    let shelf_start = use_state(|| "A".to_string());
    let shelf_end = use_state(|| "D".to_string());
    let bin_start = use_state(|| 1);
    let bin_end = use_state(|| 2);

    let on_submit_form = {
        let warehouse_id = props.warehouse_id;
        let on_submit = props.on_submit.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();

        let zone = zone.clone();
        let is_picking = is_picking.clone();
        let a_s = aisle_start.clone();
        let a_e = aisle_end.clone();
        let s_s = shelf_start.clone();
        let s_e = shelf_end.clone();
        let b_s = bin_start.clone();
        let b_e = bin_end.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let on_submit = on_submit.clone();

            let payload = BulkLocationGenerateRequest {
                zone: (*zone).clone(),
                is_picking_location: *is_picking,
                naming_format: "{zone}-{aisle}-{shelf}-{bin}".to_string(),
                aisles: Some(NumericRange {
                    start: *a_s,
                    end: *a_e,
                }),
                shelves: Some(AlphaRange {
                    start: (*s_s).clone(),
                    end: (*s_e).clone(),
                }),
                bins: Some(NumericRange {
                    start: *b_s,
                    end: *b_e,
                }),
            };

            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let route = format!(
                    "/api/inventory/warehouses/{}/locations/generate",
                    warehouse_id
                );
                if Api::post(&route, &payload, user_ctx, navigator)
                    .await
                    .is_ok()
                {
                    on_submit.emit(());
                }
            });
        })
    };

    let on_close = props.on_close.clone();
    let on_close2 = props.on_close.clone();

    html! {
        <div class="modal-overlay" onclick={move |_| on_close.emit(())}>
            <div class="modal-content" onclick={|e: MouseEvent| e.stop_propagation()}>
                <h2>{ i18n.t("bulk-gen-title") }</h2>
                <form onsubmit={on_submit_form} class="modal__form custom-grid-form">
                    <div class="range-group-box">
                        <h4>{ i18n.t("bulk-gen-zone-label") }</h4>
                        <input type="text" value={(*zone).clone()} oninput={move |e: InputEvent| zone.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value())} required=true placeholder="e.g., Bulk B" />
                    </div>

                    <div class="range-group-box">
                        <h4>{ i18n.t("bulk-gen-aisles") }</h4>
                        <input type="number" value={(*aisle_start).to_string()} oninput={move |e: InputEvent| aisle_start.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().unwrap_or(1))} />
                        <span>{ " to " }</span>
                        <input type="number" value={(*aisle_end).to_string()} oninput={move |e: InputEvent| aisle_end.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().unwrap_or(5))} />
                    </div>

                    <div class="range-group-box">
                        <h4>{ i18n.t("bulk-gen-shelves") }</h4>
                        <input type="text" value={(*shelf_start).clone()} oninput={move |e: InputEvent| shelf_start.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value())} />
                        <span>{ " to " }</span>
                        <input type="text" value={(*shelf_end).clone()} oninput={move |e: InputEvent| shelf_end.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value())} />
                    </div>

                    <div class="range-group-box">
                        <h4>{ i18n.t("bulk-gen-bins") }</h4>
                        <input type="number" value={(*bin_start).to_string()} oninput={move |e: InputEvent| bin_start.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().unwrap_or(1))} />
                        <span>{ " to " }</span>
                        <input type="number" value={(*bin_end).to_string()} oninput={move |e: InputEvent| bin_end.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().unwrap_or(2))} />
                    </div>

                    <div class="form-row checkbox-align">
                        <input type="checkbox" id="picking_loc" checked={*is_picking} onchange={move |e: Event| is_picking.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().checked())} />
                        <label for="picking_loc">{ i18n.t("bulk-gen-picking-label") }</label>
                    </div>

                    <div class="modal__form__actions">
                        <button type="button" onclick={move |_| on_close2.emit(())} class="button-secondary">{ i18n.t("common-cancel") }</button>
                        <button type="submit" class="button-success">{ i18n.t("bulk-gen-btn-execute") }</button>
                    </div>
                </form>
            </div>
        </div>
    }
}
