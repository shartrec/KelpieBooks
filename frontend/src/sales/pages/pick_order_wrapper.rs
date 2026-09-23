/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use gloo_net::http::Request;
use log::info;
use shared_core::{
    sales::{
        dtos::picklist::PickLineItem,
        models::{
            sales_order::SalesOrder,
            shipment::Shipment,
        },
    },
    OrderId,
    WarehouseId,
};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    core::components::layout::Layout,
    router::Route,
    sales::pages::sales_order_pick_page::SalesOrderPickPage,
};

#[derive(Clone, PartialEq)]
enum PageState {
    PromptInput,
    Loading(String),
    Ready {
        order_id: OrderId,
        order_number: String,
        warehouse_id: WarehouseId,
        warehouse_name: String,
        items: Vec<PickLineItem>,
    },
    Submitting,
    Error(String),
}

#[function_component(PickOrderWrapper)]
pub fn pick_order_wrapper() -> Html {
    let navigator = use_navigator();
    // Retrieve route param if launched directly via QR code or URL
    let route_params = use_route::<Route>();

    let page_state = use_state(|| PageState::PromptInput);
    let scan_input_ref = use_node_ref();

    // 1. Detect order_number from route or fetch data
    {
        let page_state = page_state.clone();
        use_effect_with(route_params, move |route| {
            let route = route.clone();
            // Inside use_effect_with:
            if let Some(Route::PickOrder { order_number }) = route {
                page_state.set(PageState::Loading(order_number.clone()));

                wasm_bindgen_futures::spawn_local(async move {
                    // Request 1: Get Order Header metadata (order_id, warehouse, etc.)
                    let header_url = format!("/api/orders/{}/header", order_number);

                    let header_res = Request::get(&header_url).send().await;
                    if let Ok(resp) = header_res {
                        if resp.ok() {
                            if let Ok(header) = resp.json::<SalesOrder>().await {
                                // Request 2: Fetch pick details using the returned order_id UUID
                                let items_url = format!("/api/orders/{}/pick-details", header.id);
                                let items_res = Request::get(&items_url).send().await;

                                if let Ok(items_resp) = items_res {
                                    if items_resp.ok() {
                                        if let Ok(items) =
                                            items_resp.json::<Vec<PickLineItem>>().await
                                        {
                                            page_state.set(PageState::Ready {
                                                order_id: header.id,
                                                order_number: header.order_number,
                                                warehouse_id: header.warehouse_id,
                                                warehouse_name: header
                                                    .warehouse_name
                                                    .unwrap_or_default(),
                                                items,
                                            });
                                            return;
                                        }
                                    }
                                }
                            }
                        }
                    }

                    page_state.set(PageState::Error(
                        "Failed to fetch order or pick details.".into(),
                    ));
                });
            }
        });
    }

    // 2. Handle manual input / barcode scanning in prompt mode
    let on_prompt_keydown = {
        let navigator = navigator.clone();
        let scan_input_ref = scan_input_ref.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                if let Some(input) = scan_input_ref.cast::<HtmlInputElement>() {
                    let val = input.value().trim().to_string();
                    if !val.is_empty() {
                        // Extract order_number if raw text or full URL was scanned
                        let order_num = val.rsplit('/').next().unwrap_or(&val).to_string();
                        if let Some(nav) = &navigator {
                            nav.push(&Route::PickOrder {
                                order_number: order_num,
                            });
                        }
                    }
                }
            }
        })
    };

    // 3. Handle final pick submission to backend API
    let on_submit_pick = {
        let page_state = page_state.clone();
        let navigator = navigator.clone();
        Callback::from(move |shipment: Shipment| {
            let page_state = page_state.clone();
            let navigator = navigator.clone();
            page_state.set(PageState::Submitting);

            wasm_bindgen_futures::spawn_local(async move {
                // Post picking payload to backend
                let response = Request::post("/api/orders/pick-submit")
                    .json(&shipment)
                    .unwrap()
                    .send()
                    .await;

                info!("{:?}", response);
                if let Ok(resp) = response {
                    info!("{:?}", resp);
                    if resp.ok() {
                        // Return to scan prompt for next order on success
                        page_state.set(PageState::PromptInput);
                        if let Some(nav) = &navigator {
                            nav.push(&Route::PickPrompt);
                        }
                        return;
                    }
                }
                page_state.set(PageState::Error("Failed to commit pick data.".into()));
            });
        })
    };

    let on_cancel_pick = {
        let page_state = page_state.clone();
        let navigator = navigator.clone();
        Callback::from(move |_: ()| {
            let navigator = navigator.clone();
            page_state.set(PageState::PromptInput);
            if let Some(nav) = &navigator {
                nav.push(&Route::PickPrompt);
            }
        })
    };

    // Render logic based on state machine
    match &*page_state {
        PageState::PromptInput => html! {
            <Layout>
                <div class="pick-prompt-container">
                    <h2>{ "Scan Picklist QR or Enter Order Number" }</h2>
                    <input
                        ref={scan_input_ref}
                        type="text"
                        class="input-scan input-scan--large"
                        placeholder="Scan QR / Barcode..."
                        onkeydown={on_prompt_keydown}
                    />
                </div>
            </ Layout>
        },
        PageState::Loading(ord) => html! {
            <div class="loading-spinner">{ format!("Loading picklist for order #{}...", ord) }</div>
        },
        PageState::Ready {
            order_id,
            order_number,
            warehouse_name,
            warehouse_id,
            items,
        } => html! {
            <Layout>
                <SalesOrderPickPage
                    order_id={order_id.clone()}
                    order_number={order_number.clone()}
                    warehouse_id={warehouse_id.clone()}
                    warehouse_name={warehouse_name.clone()}
                    initial_items={items.clone()}
                    on_submit={on_submit_pick}
                    on_cancel={on_cancel_pick}
                />
            </ Layout>
        },
        PageState::Submitting => html! {
            <Layout>
            <div class="loading-spinner">{ "Finalizing pick state..." }</div>
            </Layout>
        },
        PageState::Error(err) => html! {
            <Layout>
            <div class="error-banner">
                <p>{ err }</p>
                <button onclick={
                    let page_state = page_state.clone();
                    move |_| {
                        page_state.set(PageState::PromptInput);
                    }
                }>{ "Try Another Order" }</button>
            </div>
            </ Layout>
        },
    }
}
