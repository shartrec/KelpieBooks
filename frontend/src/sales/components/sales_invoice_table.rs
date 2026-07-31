/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use fluent::fluent_args;
use shared_core::{
    partners::models::partner::Partner,
    sales::{
        dtos::sales_invoice_list_item::SalesInvoiceListItem,
        models::{
            invoice_status::InvoiceStatus,
            sales_invoice::SalesInvoice,
        },
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
    sales::{
        components::sales_invoice_drawer::SalesInvoiceDrawer,
        contexts::sales_invoice_filter_context::{
            use_sales_invoice_filter,
            PaymentStatusFilter,
        },
    },
};

#[function_component(SalesInvoiceTable)]
pub fn sales_invoice_table() -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();
    let filter_ctx = use_sales_invoice_filter();
    let invoices = use_state(Vec::new);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);
    let invoice_to_view = use_state(|| None::<SalesInvoice>);
    let partner_to_view = use_state(|| None::<Partner>);
    let show_actions = use_state(|| None::<uuid::Uuid>);

    let fetch_invoices = {
        let invoices = invoices.clone();
        let error = error.clone();
        let loading = loading.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        let filter_ctx = filter_ctx.clone();
        Callback::from(move |_: ()| {
            let invoices = invoices.clone();
            let error = error.clone();
            let loading = loading.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            let filter_ctx = filter_ctx.clone();
            loading.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                let mut url = format!(
                    "/api/sales-invoices?start_date={}&end_date={}",
                    filter_ctx.start_date, filter_ctx.end_date
                );
                if let Some(partner_id) = filter_ctx.partner_id {
                    url.push_str(&format!("&partner_id={}", partner_id));
                }
                if let Some(min_amount) = filter_ctx.min_amount {
                    url.push_str(&format!("&min_amount={}", min_amount));
                }
                match filter_ctx.status {
                    PaymentStatusFilter::Draft => {
                        url.push_str(&format!("&status={}", InvoiceStatus::Draft));
                    }
                    PaymentStatusFilter::Outstanding => {
                        url.push_str(&format!("&status={}", InvoiceStatus::Open));
                    }
                    PaymentStatusFilter::Paid => {
                        url.push_str(&format!("&status={}", InvoiceStatus::Paid));
                    }
                    PaymentStatusFilter::All => {}
                }
                let fetched_invoices = Api::get(&url, user_ctx, navigator).await;
                loading.set(false);
                match fetched_invoices {
                    Ok(response) if response.ok() => {
                        match response.json::<Vec<SalesInvoiceListItem>>().await {
                            Ok(data) => {
                                invoices.set(data);
                            }
                            Err(e) => error.set(Some(i18n.t_args(
                                "sales-invoice-table-error-parse-invoices",
                                &fluent_args!["error" => e.to_string()],
                            ))),
                        }
                    }
                    Ok(response) => error.set(Some(i18n.t_args(
                        "sales-invoice-table-error-fetch-invoices",
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

    let fetch_invoices_clone = fetch_invoices.clone();
    use_effect_with(
        (
            filter_ctx.start_date,
            filter_ctx.end_date,
            filter_ctx.partner_id,
            filter_ctx.min_amount,
            filter_ctx.status,
        ),
        move |_| {
            fetch_invoices_clone.emit(());
            || ()
        },
    );

    let on_view_click = {
        let invoice_to_view = invoice_to_view.clone();
        let partner_to_view = partner_to_view.clone();
        let error = error.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        Callback::from(move |id: uuid::Uuid| {
            let invoice_to_view = invoice_to_view.clone();
            let partner_to_view = partner_to_view.clone();
            let error = error.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Api::get(
                    &format!("/api/sales-invoices/{}", id),
                    user_ctx.clone(),
                    navigator.clone(),
                )
                .await;
                match resp {
                    Ok(r) if r.ok() => match r.json::<SalesInvoice>().await {
                        Ok(invoice) => {
                            let partner_resp = Api::get(
                                &format!("/api/partners/{}", invoice.partner_id),
                                user_ctx,
                                navigator,
                            )
                            .await;
                            match partner_resp {
                                Ok(pr) if pr.ok() => match pr.json::<Partner>().await {
                                    Ok(partner) => partner_to_view.set(Some(partner)),
                                    Err(e) => error.set(Some(i18n.t_args(
                                        "sales-invoice-table-error-parse-partner",
                                        &fluent::fluent_args!["error" => e.to_string()],
                                    ))),
                                },
                                Ok(pr) => error.set(Some(i18n.t_args(
                                    "sales-invoice-table-error-fetch-partner",
                                    &fluent::fluent_args!["status" => pr.status()],
                                ))),
                                Err(e) => error.set(Some(i18n.t_args(
                                    "common-network-error",
                                    &fluent::fluent_args!["error" => e.to_string()],
                                ))),
                            }
                            invoice_to_view.set(Some(invoice));
                        }
                        Err(e) => error.set(Some(i18n.t_args(
                            "sales-invoice-table-error-parse-invoice",
                            &fluent::fluent_args!["error" => e.to_string()],
                        ))),
                    },
                    Ok(r) => error.set(Some(i18n.t_args(
                        "sales-invoice-table-error-fetch-invoice",
                        &fluent::fluent_args!["status" => r.status()],
                    ))),
                    Err(e) => error.set(Some(i18n.t_args(
                        "common-network-error",
                        &fluent::fluent_args!["error" => e.to_string()],
                    ))),
                }
            });
        })
    };

    let on_drawer_close = {
        let invoice_to_view = invoice_to_view.clone();
        let partner_to_view = partner_to_view.clone();
        Callback::from(move |()| {
            invoice_to_view.set(None);
            partner_to_view.set(None);
        })
    };

    let on_drawer_change = {
        let invoice_to_view = invoice_to_view.clone();
        let partner_to_view = partner_to_view.clone();
        let fetch_invoices = fetch_invoices.clone();
        Callback::from(move |()| {
            invoice_to_view.set(None);
            partner_to_view.set(None);
            fetch_invoices.emit(());
        })
    };

    if *loading {
        return html! { <p>{ i18n.t("common-loading") }</p> };
    }
    if let Some(err) = &*error {
        return html! { <div class="error">{ err }</div> };
    }

    html! {
        <>
            if let (Some(invoice), Some(partner)) = (&*invoice_to_view, &*partner_to_view) {
                <SalesInvoiceDrawer
                    invoice={invoice.clone()}
                    partner={partner.clone()}
                    on_close={on_drawer_close}
                    on_change={on_drawer_change}
                />
            }
            <table class="table">
                <thead>
                    <tr>
                        <th class="table__text-col">{ i18n.t("common-customer") }</th>
                        <th class="table__text-col">{ i18n.t("sales-invoice-table-invoice-number") }</th>
                        <th class="table__value-col">{ i18n.t("sales-invoice-table-invoice-date") }</th>
                        <th class="table__value-col">{ i18n.t("common-due-date") }</th>
                        <th class="table__value-col">{ i18n.t("common-net") }</th>
                        <th class="table__value-col">{ i18n.t("common-tax") }</th>
                        <th class="table__value-col">{ i18n.t("common-gross") }</th>
                        <th class="table__col-actions">{ i18n.t("common-actions") }</th>
                    </tr>
                </thead>
                <tbody>
                    { for (*invoices).iter().map(|inv| {
                        let on_view = {
                            let on_view_click = on_view_click.clone();
                            let show_actions = show_actions.clone();
                            let invoice_id = inv.id;
                            Callback::from(move |_| {
                                show_actions.set(None);
                                on_view_click.emit(invoice_id);
                            })
                        };
                        let on_actions_toggle = {
                            let show_actions = show_actions.clone();
                            let invoice_id = inv.id;
                            Callback::from(move |_| {
                                if show_actions.as_ref() == Some(&invoice_id) {
                                    show_actions.set(None);
                                } else {
                                    show_actions.set(Some(invoice_id));
                                }
                            })
                        };
                        html!{
                            <tr>
                                <td class="table__text-col">{ &inv.partner_name }</td>
                                <td class="table__text-col">{ &inv.invoice_number }</td>
                                <td class="table__value-col">{ i18n.format_date(inv.issue_date) }</td>
                                <td class="table__value-col">{ i18n.format_date(inv.due_date) }</td>
                                <td class="table__value-col">{ i18n.format_currency(inv.net_amount) }</td>
                                <td class="table__value-col">{ i18n.format_currency(inv.tax_amount) }</td>
                                <td class="table__value-col">{ i18n.format_currency(inv.gross_amount) }</td>
                                <td class="table__col-actions">
                                    <div class="actions-dropdown">
                                        <button class="icon-button" onclick={on_actions_toggle} title={i18n.t("common-actions")}>
                                            <img src="/images/more-vertical.svg" alt={i18n.t("common-actions")} class="dropdown-trigger-icon" />
                                        </button>
                                        if *show_actions == Some(inv.id) {
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
                    })}
                </tbody>
            </table>
        </>
    }
}
