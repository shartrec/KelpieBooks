/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use fluent::fluent_args;
use shared_core::sales::{
    models::{
        sales_document_status::SalesDocumentStatus,
    },
};
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;
use shared_core::sales::dtos::sales_order_dto::SalesOrderDto;
use shared_core::sales::models::sales_order::SalesOrder;
use crate::{
    api::Api,
    contexts::{
        auth_context::use_user_context,
        locale_context::{
            use_locale,
            LocaleContext,
        },
    },
    core::components::layout::Layout,
    router::Route,
    sales::components::sales_order_drawer::SalesOrderDrawer,
};

#[function_component(SalesOrdersPage)]
pub fn sales_orders_page() -> Html {
    let i18n = use_locale();
    let user_ctx = use_user_context();
    let navigator = use_navigator().unwrap();

    // List state
    let orders = use_state(Vec::<SalesOrder>::new);
    let status_filter = use_state(|| "Open".to_string());
    let list_error = use_state(|| None::<String>);

    // Drawer state
    let selected_order = use_state(|| None::<SalesOrderDto>);
    let drawer_error = use_state(|| None::<String>);

    // Actions drop down
    let show_actions = use_state(|| None::<uuid::Uuid>);

    // Fetch the order list whenever the status filter changes
    let fetch_orders = {
        let orders = orders.clone();
        let status_filter = status_filter.clone();
        let list_error = list_error.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        let i18n = i18n.clone();
        Callback::from(move |_: ()| {
            let orders = orders.clone();
            let status_filter = status_filter.clone();
            let list_error = list_error.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            let i18n = i18n.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = if *status_filter == "All" {
                    "/api/sales-orders".to_string()
                } else {
                    format!("/api/sales-orders?status={}", *status_filter)
                };
                let resp = Api::get(&url, user_ctx, navigator).await;
                match resp {
                    Ok(r) if r.ok() => match r.json::<Vec<SalesOrder>>().await {
                        Ok(data) => {
                            orders.set(data);
                            list_error.set(None);
                        }
                        Err(e) => list_error.set(Some(i18n.t_args(
                            "sales-orders-list-error-parse",
                            &fluent_args!["error" => e.to_string()],
                        ))),
                    },
                    Ok(r) => list_error.set(Some(i18n.t_args(
                        "sales-orders-list-error-fetch",
                        &fluent_args!["status" => r.status().to_string()],
                    ))),
                    Err(e) => list_error.set(Some(i18n.t_args(
                        "common-network-error",
                        &fluent_args!["error" => e.to_string()],
                    ))),
                }
            });
        })
    };

    // Initial load
    {
        let fetch_orders = fetch_orders.clone();
        use_effect_with((*status_filter).clone(), move |_| {
            fetch_orders.emit(());
            || ()
        });
    }

    let on_status_change = {
        let status_filter = status_filter.clone();
        Callback::from(move |e: Event| {
            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
            status_filter.set(select.value());
        })
    };

    let on_new_order = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::NewSalesOrder);
        })
    };

    // When a row is clicked, fetch the full order and open the drawer
    let on_view_click = {
        let selected_order = selected_order.clone();
        let drawer_error = drawer_error.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        let i18n = i18n.clone();
        Callback::from(move |id: Uuid| {
            let selected_order = selected_order.clone();
            let drawer_error = drawer_error.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            let i18n = i18n.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("/api/sales-orders/{}", id);
                let resp = Api::get(&url, user_ctx, navigator).await;
                match resp {
                    Ok(r) if r.ok() => match r.json::<SalesOrderDto>().await {
                        Ok(order) => {
                            selected_order.set(Some(order));
                            drawer_error.set(None);
                        }
                        Err(e) => drawer_error.set(Some(i18n.t_args(
                            "sales-orders-drawer-error-parse",
                            &fluent_args!["error" => e.to_string()],
                        ))),
                    },
                    Ok(r) => drawer_error.set(Some(i18n.t_args(
                        "sales-orders-drawer-error-fetch",
                        &fluent_args!["status" => r.status().to_string()],
                    ))),
                    Err(e) => drawer_error.set(Some(i18n.t_args(
                        "common-network-error",
                        &fluent_args!["error" => e.to_string()],
                    ))),
                }
            });
        })
    };

    let on_drawer_close = {
        let selected_order = selected_order.clone();
        Callback::from(move |()| selected_order.set(None))
    };

    // On confirm: navigate to SalesOrders so the user sees the new order
    let on_confirmed = {
        let navigator = navigator.clone();
        Callback::from(move |_order: SalesOrderDto| {
            navigator.push(&Route::SalesOrders);
        })
    };

    // On cancel: close drawer and refresh list
    let on_cancelled = {
        let selected_order = selected_order.clone();
        let fetch_orders = fetch_orders.clone();
        Callback::from(move |()| {
            selected_order.set(None);
            fetch_orders.emit(());
        })
    };

    html! {
        <Layout>
            <div class="report-header">
                <h1>{ i18n.t("sales-orders-list-title") }</h1>
            </div>
            <div class="table-actions">
                <select onchange={on_status_change} value={(*status_filter).clone()}>
                    <option value="Draft" selected={*status_filter == "Draft"}>{ i18n.t("sales-order-status-draft") }</option>
                    <option value="Open" selected={*status_filter == "Open"}>{ i18n.t("sales-order-status-open") }</option>
                    <option value="Completed" selected={*status_filter == "Completed"}>{ i18n.t("sales-order-status-completed") }</option>
                    <option value="Cancelled" selected={*status_filter == "Cancelled"}>{ i18n.t("sales-order-status-cancelled") }</option>
                    <option value="All" selected={*status_filter == "All"}>{ i18n.t("sales-orders-list-filter-all") }</option>
                </select>
                <button class="button-primary" onclick={on_new_order}>{ i18n.t("sales-orders-list-new-button") }</button>
            </div>

            if let Some(e) = &*list_error {
                <div class="error">{ e }</div>
            }
            if let Some(e) = &*drawer_error {
                <div class="error">{ e }</div>
            }

            <table class="table">
                <thead>
                    <tr>
                        <th class="table__text-col">{ i18n.t("sales-orders-list-col-number") }</th>
                        <th class="table__text-col">{ i18n.t("common-customer") }</th>
                        <th class="table__text-col">{ i18n.t("sales-orders-list-col-warehouse") }</th>
                        <th class="table__value-col">{ i18n.t("new-sales-order-date-label") }</th>
                        <th class="table__text-col">{ i18n.t("common-status") }</th>
                        <th class="table__value-col">{ i18n.t("common-total") }</th>
                        <th class="table__col-actions">{ i18n.t("common-actions") }</th>
                    </tr>
                </thead>
                <tbody>
                    { for (*orders).iter().map(|order| {
                        let id = order.id;
                        let on_view = {
                            let on_view_click = on_view_click.clone();
                            let show_actions = show_actions.clone();
                            let order_id = id;
                            Callback::from(move |_| {
                                show_actions.set(None);
                                on_view_click.emit(order_id);
                            })
                        };
                       let on_actions_toggle = {
                            let show_actions = show_actions.clone();
                            let order_id = id;
                            Callback::from(move |_| {
                                if show_actions.as_ref() == Some(&order_id) {
                                    show_actions.set(None);
                                } else {
                                    show_actions.set(Some(order_id));
                                }
                            })
                        };
                        let status_class = status_chip_class(&order.document_status);
                        html! {
                            <tr class="clickable-row">
                                <td class="table__text-col">{ &order.order_number }</td>
                                <td class="table__text-col">{ &order.partner_name.as_ref().unwrap_or(&"".to_string()) }</td>
                                <td class="table__text-col">{ &order.warehouse_name.as_ref().unwrap_or(&"".to_string()) }</td>
                                <td class="table__value-col">{ i18n.format_date(order.order_date) }</td>
                                <td class="table__text-col"><span class={status_class}>{ status_label(&order.document_status, &i18n) }</span></td>
                                <td class="table__value-col">{ i18n.format_currency(order.amount_remaining) }</td>
                                <td class="table__col-actions">
                                    <div class="actions-dropdown">
                                        <button class="icon-button" onclick={on_actions_toggle} title={i18n.t("common-actions")}>
                                            <img src="/images/more-vertical.svg" alt={i18n.t("common-actions")} class="dropdown-trigger-icon" />
                                        </button>
                                        if *show_actions == Some(id) {
                                            <div class="actions-dropdown__content">
                                                <button class="dropdown-item" onclick={on_view}>
                                                    <img src="/images/view.svg" alt={i18n.t("common-view")} />
                                                    <span>{ i18n.t("common-view") }</span>
                                                </button>
                                                <button class="dropdown-item">
                                                    <img src="/images/delete.svg" alt={i18n.t("common-delete")} />
                                                    <span>{ i18n.t("common-delete") }</span>
                                                </button>
                                            </div>
                                        }
                                    </div>
                                </td>
                            </tr>
                        }
                    }) }
                </tbody>
            </table>

            if let Some(order) = (*selected_order).clone() {
                <SalesOrderDrawer
                    order={order}
                    on_close={on_drawer_close}
                    on_confirmed={on_confirmed}
                    on_cancelled={on_cancelled}
                />
            }
        </Layout>
    }
}

fn status_chip_class(status: &SalesDocumentStatus) -> &'static str {
    match status {
        SalesDocumentStatus::Draft => "status-badge status-badge--draft",
        SalesDocumentStatus::Open => "status-badge status-badge--open",
        SalesDocumentStatus::Completed => "status-badge status-badge--completed",
        SalesDocumentStatus::Cancelled => "status-badge status-badge--cancelled",
    }
}

fn status_label(status: &SalesDocumentStatus, i18n: &LocaleContext) -> String {
    match status {
        SalesDocumentStatus::Draft => i18n.t("sales-order-status-draft"),
        SalesDocumentStatus::Open => i18n.t("sales-order-status-open"),
        SalesDocumentStatus::Completed => i18n.t("sales-order-status-confirmed"),
        SalesDocumentStatus::Cancelled => i18n.t("sales-order-status-cancelled"),
    }
}
