/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

pub(crate) mod addresses_view;
pub(crate) mod lines_view;

use fluent::fluent_args;
use shared_core::{
    core::models::auth::SystemPrivilege,
    sales::{
        dtos::sales_order_dto::SalesOrderDto,
        models::sales_document_status::SalesDocumentStatus,
    },
};
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{
    api::Api,
    contexts::{
        auth_context::use_user_context,
        locale_context::use_locale,
    },
    sales::components::sales_order_drawer::{
        addresses_view::AddressesView,
        lines_view::LinesView,
    },
    BackendError,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DrawerTab {
    Lines,
    Addresses,
}

#[derive(Properties, PartialEq, Clone)]
pub struct SalesOrderDrawerProps {
    pub order: SalesOrderDto,
    pub on_close: Callback<()>,
    pub on_confirmed: Callback<SalesOrderDto>,
    pub on_cancelled: Callback<()>,
}

#[function_component(SalesOrderDrawer)]
pub fn sales_order_drawer(props: &SalesOrderDrawerProps) -> Html {
    let i18n = use_locale();
    let user_ctx = use_user_context();
    let navigator = use_navigator().unwrap();
    let active_tab = use_state(|| DrawerTab::Lines);
    let confirm_error = use_state(|| None::<String>);
    let cancel_error = use_state(|| None::<String>);
    let is_confirming = use_state(|| false);
    let is_cancelling = use_state(|| false);

    let order_id = props.order.order.id;
    let can_manage = user_ctx.has_privilege(&SystemPrivilege::ManageSales);
    let is_draft = props.order.order.document_status == SalesDocumentStatus::Draft;

    let on_close = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| on_close.emit(()))
    };

    let on_confirm_click = {
        let on_confirmed = props.on_confirmed.clone();
        let confirm_error = confirm_error.clone();
        let is_confirming = is_confirming.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        let i18n = i18n.clone();
        Callback::from(move |_| {
            let on_confirmed = on_confirmed.clone();
            let confirm_error = confirm_error.clone();
            let is_confirming = is_confirming.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            let i18n = i18n.clone();
            is_confirming.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("/api/sales-orders/{}/confirm", order_id);
                let resp = Api::post(&url, &(), user_ctx, navigator).await;
                is_confirming.set(false);
                match resp {
                    Ok(r) if r.ok() => match r.json::<SalesOrderDto>().await {
                        Ok(order) => {
                            confirm_error.set(None);
                            on_confirmed.emit(order);
                        }
                        Err(e) => confirm_error.set(Some(i18n.t_args(
                            "sales-orders-drawer-error-confirm-parse",
                            &fluent_args!["error" => e.to_string()],
                        ))),
                    },
                    Ok(r) => {
                        let error_message = match r.json::<BackendError>().await {
                            Ok(api_err) => api_err.error,
                            Err(_) => r.text().await.unwrap_or_else(|_| r.status_text()),
                        };
                        confirm_error.set(Some(i18n.t_args(
                            "sales-orders-drawer-error-confirm",
                            &fluent_args!["status" => error_message], // I want the message text here
                        )))
                    }
                    Err(e) => confirm_error.set(Some(i18n.t_args(
                        "common-network-error",
                        &fluent_args!["error" => e.to_string()],
                    ))),
                }
            });
        })
    };

    let on_cancel_click = {
        let on_cancelled = props.on_cancelled.clone();
        let cancel_error = cancel_error.clone();
        let is_cancelling = is_cancelling.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        let i18n = i18n.clone();
        Callback::from(move |_| {
            let on_cancelled = on_cancelled.clone();
            let cancel_error = cancel_error.clone();
            let is_cancelling = is_cancelling.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            let i18n = i18n.clone();
            is_cancelling.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("/api/sales-orders/{}/cancel", order_id);
                let resp = Api::post(&url, &(), user_ctx, navigator).await;
                is_cancelling.set(false);
                match resp {
                    Ok(r) if r.ok() => {
                        cancel_error.set(None);
                        on_cancelled.emit(());
                    }
                    Ok(r) => cancel_error.set(Some(i18n.t_args(
                        "sales-orders-drawer-error-cancel",
                        &fluent_args!["status" => r.status().to_string()],
                    ))),
                    Err(e) => cancel_error.set(Some(i18n.t_args(
                        "common-network-error",
                        &fluent_args!["error" => e.to_string()],
                    ))),
                }
            });
        })
    };

    let status_class = match props.order.order.document_status {
        SalesDocumentStatus::Draft => "status-badge status-badge--draft",
        SalesDocumentStatus::Open => "status-badge status-badge--open",
        SalesDocumentStatus::Completed => "status-badge status-badge--complete",
        SalesDocumentStatus::Cancelled => "status-badge status-badge--cancelled",
    };

    let status_label = match props.order.order.document_status {
        SalesDocumentStatus::Draft => i18n.t("sales-order-status-draft"),
        SalesDocumentStatus::Open => i18n.t("sales-order-status-open"),
        SalesDocumentStatus::Completed => i18n.t("sales-order-status-complete"),
        SalesDocumentStatus::Cancelled => i18n.t("sales-order-status-cancelled"),
    };

    html! {
        <div class="drawer-overlay" onclick={on_close.clone()}>
            <div class="drawer" onclick={|e: MouseEvent| e.stop_propagation()}>
                // Header
                <header class="drawer__header">
                    <h3 class="payment-context-banner__vendor">{ &props.order.order.order_number }</h3>
                    <button class="btn-close" type="button" onclick={on_close.clone()}>
                        <img src="/images/x.svg" alt={i18n.t("common-close")} />
                    </button>
                </header>

                // Summary banner
                <div class="payment-context-banner">
                    <div class="payment-context-banner__details">
                        <span>{ i18n.t_args("sales-orders-drawer-order-number", &fluent_args!["number" => props.order.order.order_number.clone()]) }</span>
                        <span>{ i18n.t_args("sales-orders-drawer-warehouse", &fluent_args!["warehouse" => props.order.order.warehouse_name.clone()]) }</span>
                        <span>{ i18n.format_date(props.order.order.order_date) }</span>
                        <span class={status_class}>{ status_label }</span>
                        <span class="amount-badge amount-badge--gross">
                            { i18n.t_args("vendor-invoice-drawer-gross", &fluent_args!["amount" => i18n.format_currency(props.order.order.total_amount)]) }
                        </span>
                    </div>
                </div>

                // Tabs
                <div class="drawer__tabs">
                    <button
                        class={classes!("tab-trigger", (*active_tab == DrawerTab::Lines).then_some("tab-trigger--active"))}
                        onclick={{
                            let active_tab = active_tab.clone();
                            Callback::from(move |_| active_tab.set(DrawerTab::Lines))
                        }}
                    >
                        { i18n.t("common-items") }
                    </button>
                    <button
                        class={classes!("tab-trigger", (*active_tab == DrawerTab::Addresses).then_some("tab-trigger--active"))}
                        onclick={{
                            let active_tab = active_tab.clone();
                            Callback::from(move |_| active_tab.set(DrawerTab::Addresses))
                        }}
                    >
                        { i18n.t("common-addresses") }
                    </button>
                </div>

                // Tab content
                <div class="drawer__content">
                    {
                        match *active_tab {
                            DrawerTab::Lines => html! {
                                <LinesView order={props.order.clone()} />
                            },
                            DrawerTab::Addresses => html! {
                                <AddressesView order={props.order.clone()} />
                            },
                        }
                    }
                </div>

                // Footer with action buttons
                if let Some(e) = &*confirm_error {
                    <div class="error">{ e }</div>
                }
                if let Some(e) = &*cancel_error {
                    <div class="error">{ e }</div>
                }
                <footer class="drawer__footer">
                    { if is_draft && can_manage {
                        html! {
                            <>
                                <button
                                    class="button-primary"
                                    onclick={on_confirm_click}
                                    disabled={*is_confirming}
                                >
                                    { i18n.t("sales-orders-drawer-confirm-button") }
                                </button>
                                <button
                                    class="button-danger"
                                    onclick={on_cancel_click}
                                    disabled={*is_cancelling}
                                >
                                    { i18n.t("sales-orders-drawer-cancel-button") }
                                </button>
                            </>
                        }
                    } else {
                        html! {}
                    }}
                    <button class="button-secondary" onclick={on_close.clone()}>{ i18n.t("common-close") }</button>
                </footer>
            </div>
        </div>
    }
}
