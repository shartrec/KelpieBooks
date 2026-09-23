/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use std::ops::Add;

use fluent::fluent_args;
use rust_decimal::Decimal;
use shared_core::{
    sales::{
        dtos::picklist::PickLineItem,
        models::shipment::{
            Shipment,
            ShipmentItem,
        },
    },
    OrderId,
    ShipmentId,
    WarehouseId,
};
use web_sys::{
    HtmlInputElement,
    InputEvent,
    KeyboardEvent,
};
use yew::{
    function_component,
    html,
    use_effect_with,
    use_node_ref,
    use_state,
    Callback,
    Html,
    Properties,
    TargetCast,
};

use crate::{
    contexts::locale_context::use_locale,
    core::components::number_input::DecimalInput,
};

pub enum Msg {
    OnScanInputKeyDown(KeyboardEvent),
    UpdatePickedQty(usize, u32),
    PickAll,
    SubmitPicking,
}

#[derive(Properties, PartialEq)]
pub struct SalesOrderPickProps {
    pub order_id: OrderId,
    pub order_number: String,
    pub warehouse_id: WarehouseId,
    pub warehouse_name: String,
    pub initial_items: Vec<PickLineItem>,
    pub on_submit: Callback<Shipment>,
    pub on_cancel: Callback<()>,
}

#[function_component(SalesOrderPickPage)]
pub fn sales_order_pick_page(props: &SalesOrderPickProps) -> Html {
    let i18n = use_locale();

    let items = use_state(|| props.initial_items.clone());
    let scan_query = use_state(|| String::new());
    let scan_input_ref = use_node_ref();

    // Auto-focus the barcode scanner input when component mounts
    {
        let scan_input_ref = scan_input_ref.clone();
        use_effect_with((), move |_| {
            if let Some(input) = scan_input_ref.cast::<HtmlInputElement>() {
                let _ = input.focus();
            }
            || ()
        });
    }

    // Handlers
    let on_pick_all = {
        let items = items.clone();
        Callback::from(move |_| {
            let mut updated = (*items).clone();
            for line in updated.iter_mut() {
                line.picked_qty = line.ordered_qty;
            }
            items.set(updated);
        })
    };

    let on_update_qty = {
        let items = items.clone();
        Callback::from(move |(idx, qty): (usize, Decimal)| {
            let mut updated = (*items).clone();
            if let Some(line) = updated.get_mut(idx) {
                line.picked_qty = qty;
            }
            items.set(updated);
        })
    };

    let on_scan_keydown = {
        let items = items.clone();
        let scan_query = scan_query.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let sku = scan_query.trim().to_uppercase();
                if !sku.is_empty() {
                    let mut updated = (*items).clone();
                    if let Some(line) = updated.iter_mut().find(|i| i.code.to_uppercase() == sku) {
                        line.picked_qty = line.picked_qty.add(Decimal::new(1, 0));
                    }
                    items.set(updated);
                    scan_query.set(String::new()); // Clear input after scan
                }
            }
        })
    };

    let on_submit_click = {
        let items = items.clone();
        let order_id = props.order_id.clone();
        let order_number = props.order_number.clone();
        let warehouse_id = props.warehouse_id.clone();
        let on_submit = props.on_submit.clone();
        Callback::from(move |_| {
            let lines = (*items)
                .iter()
                .map(|item| ShipmentItem {
                    id: ShipmentId::default(),
                    warehouse_id: warehouse_id,
                    location_id: item.location_id.unwrap_or_default(),
                    order_line_id: item.order_line_id,
                    item_id: item.item_id,
                    qty: item.picked_qty,
                })
                .collect();
            // Create a shipment from the items
            let shipment = Shipment {
                id: ShipmentId::default(),
                order_id: order_id,
                order_number: order_number.clone(),
                shipment_number: Default::default(),
                carrier: Default::default(),
                tracking_number: Default::default(),
                shipped_at: Default::default(),
                created_at: Default::default(),
                lines: lines,
            };
            on_submit.emit(shipment);
        })
    };

    let on_cancel_click = {
        let on_cancel = props.on_cancel.clone();
        Callback::from(move |_| {
            on_cancel.emit(());
        })
    };

    html! {
        <div class="picking-screen">
            <header class="picking-screen__header">
                <div class="picking-screen__title-group">
                    <h2>{ i18n.t_args("picking-header-title", &fluent_args!["order_number" => props.order_number.clone()]) }</h2>
                    <span class="badge badge--info">
                        { i18n.t_args("picking-header-warehouse", &fluent_args!["warehouse_name" => props.warehouse_name.clone()]) }
                    </span>
                </div>
                <div class="picking-screen__actions">
                    <button class="btn btn--secondary" onclick={on_pick_all}>
                        { i18n.t("picking-btn-pick-all") }
                    </button>
                </div>
            </header>

            <div class="picking-screen__scan-bar">
                <label for="barcode-scanner-input">{ i18n.t("picking-scan-label") }</label>
                <input
                    id="barcode-scanner-input"
                    ref={scan_input_ref}
                    type="text"
                    class="input-scan"
                    placeholder={i18n.t("picking-scan-placeholder")}
                    value={(*scan_query).clone()}
                    oninput={
                        let scan_query = scan_query.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            scan_query.set(input.value());
                        })
                    }
                    onkeydown={on_scan_keydown}
                />
            </div>

            <table class="picking-table">
                <thead>
                    <tr>
                        <th style="width: 15%;">{ i18n.t("picking-table-header-location") }</th>
                        <th style="width: 35%;">{ i18n.t("picking-table-header-item") }</th>
                        <th style="width: 12%;" class="text-right">{ i18n.t("picking-table-header-ordered") }</th>
                        <th style="width: 12%;" class="text-right">{ i18n.t("picking-table-header-uom") }</th>
                        <th style="width: 16%;" class="text-center">{ i18n.t("picking-table-header-picked-qty") }</th>
                        <th style="width: 10%;" class="text-center">{ i18n.t("picking-table-header-status") }</th>
                    </tr>
                </thead>
                <tbody>
                    { for items.iter().enumerate().map(|(idx, line)| {
                        let is_complete = line.picked_qty == line.ordered_qty;
                        let is_over = line.picked_qty > line.ordered_qty;
                        let on_update_qty = on_update_qty.clone();

                        let row_class = if is_over {
                            "picking-row picking-row--error"
                        } else if is_complete {
                            "picking-row picking-row--complete"
                        } else if line.picked_qty > Decimal::ZERO {
                            "picking-row picking-row--in-progress"
                        } else {
                            "picking-row"
                        };

                        html! {
                            <tr class={row_class} key={format!("{}", line.item_id).clone()}>
                                <td class="location-cell">
                                    <strong>{ line.location.as_ref().unwrap_or(&"".to_string()) }</strong>
                                </td>
                                <td>
                                    <div class="item-name">{ &line.name }</div>
                                    if !line.code.is_empty() {
                                        <div class="item-code">{ &line.code }</div>
                                    }
                                </td>
                                <td class="text-right">{ i18n.format_decimal(line.ordered_qty) }</td>
                                <td class="text-right">{ line.uom.as_ref().unwrap_or(&"".to_string()) }</td>
                                <td class="text-center">
                                    <DecimalInput
                                        class="input-qty"
                                        value={line.picked_qty}
                                        on_focus={Callback::from(|e: HtmlInputElement| {
                                            e.select();
                                        })}
                                        on_change={Callback::from(move |value: Decimal| {
                                            on_update_qty.emit((idx, value));
                                        })}
                                    />
                                </td>
                                <td class="text-center">
                                    if is_complete {
                                        <span class="badge badge--success">{ i18n.t("picking-status-ok") }</span>
                                    } else if is_over {
                                        <span class="badge badge--danger">{ i18n.t("picking-status-over") }</span>
                                    } else {
                                        <span class="badge badge--neutral">{ i18n.t("picking-status-pending") }</span>
                                    }
                                </td>
                            </tr>
                        }
                    })}
                </tbody>
            </table>
                { html! {
                        <div class="picking-screen__actions">
                            <button class="btn button-secondary" onclick={on_cancel_click}>{ i18n.t("common-cancel") }</button>
                            <button class="btn btn--success" onclick={on_submit_click}>{ i18n.t("picking-btn-confirm") }</button>
                        </div>
                    }
                }
        </div>
    }
}
