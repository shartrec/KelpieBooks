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
    payables::{
        dtos::vendor_invoice_list_item::VendorInvoiceListItem,
        models::{
            invoice_status::InvoiceStatus,
            vendor_invoice::VendorInvoice,
        },
    },
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
    payables::{
        components::vendor_invoice_drawer::{
            InvoiceDrawerTab,
            VendorInvoiceDrawer,
        },
        vendor_invoice_filter_context::{
            use_vendor_invoice_filter,
            PaymentStatusFilter,
        },
    },
};

#[function_component(VendorInvoiceTable)]
pub fn vendor_invoice_table() -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();
    let filter_ctx = use_vendor_invoice_filter();
    let invoices = use_state(Vec::new);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);
    let invoice_to_edit = use_state(|| None::<VendorInvoice>);
    let partner_to_edit = use_state(|| None::<Partner>);
    let show_actions = use_state(|| None::<Uuid>);
    let initial_tab = use_state(|| InvoiceDrawerTab::General);

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
                    "/api/vendor-invoices?start_date={}&end_date={}",
                    filter_ctx.start_date, filter_ctx.end_date
                );
                if let Some(partner_id) = filter_ctx.partner_id {
                    url.push_str(&format!("&partner_id={}", partner_id));
                }
                if let Some(min_amount) = filter_ctx.min_amount {
                    url.push_str(&format!("&min_amount={}", min_amount));
                }
                match filter_ctx.status {
                    PaymentStatusFilter::Outstanding => {
                        url.push_str(&format!(
                            "&status={},{}",
                            InvoiceStatus::Open.as_str(),
                            InvoiceStatus::PartiallyPaid.as_str()
                        ));
                    }
                    PaymentStatusFilter::Paid => {
                        url.push_str(&format!("&status={}", InvoiceStatus::Paid.as_str()));
                    }
                    PaymentStatusFilter::All => {}
                }
                let fetched_invoices = Api::get(&url, user_ctx, navigator).await;
                loading.set(false);
                match fetched_invoices {
                    Ok(response) if response.ok() => {
                        match response.json::<Vec<VendorInvoiceListItem>>().await {
                            Ok(data) => {
                                invoices.set(data);
                            }
                            Err(e) => error.set(Some(i18n.t_args(
                                "vendor-invoice-table-error-parse-invoices",
                                &fluent_args!["error" => e.to_string()],
                            ))),
                        }
                    }
                    Ok(response) => error.set(Some(i18n.t_args(
                        "vendor-invoice-table-error-fetch-invoices",
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

    let on_edit_click = {
        let invoice_to_edit = invoice_to_edit.clone();
        let partner_to_edit = partner_to_edit.clone();
        let error = error.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        let initial_tab = initial_tab.clone();
        Callback::from(move |(id, tab): (Uuid, InvoiceDrawerTab)| {
            let invoice_to_edit = invoice_to_edit.clone();
            let partner_to_edit = partner_to_edit.clone();
            let error = error.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            initial_tab.set(tab);
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Api::get(
                    &format!("/api/vendor-invoices/{}", id),
                    user_ctx.clone(),
                    navigator.clone(),
                )
                .await;
                match resp {
                    Ok(r) if r.ok() => match r.json::<VendorInvoice>().await {
                        Ok(invoice) => {
                            let partner_resp = Api::get(
                                &format!("/api/partners/{}", invoice.partner_id),
                                user_ctx,
                                navigator,
                            )
                            .await;
                            match partner_resp {
                                Ok(pr) if pr.ok() => match pr.json::<Partner>().await {
                                    Ok(partner) => partner_to_edit.set(Some(partner)),
                                    Err(e) => error.set(Some(i18n.t_args(
                                        "vendor-invoice-table-error-parse-partner",
                                        &fluent_args!["error" => e.to_string()],
                                    ))),
                                },
                                Ok(pr) => error.set(Some(i18n.t_args(
                                    "vendor-invoice-table-error-fetch-partner",
                                    &fluent_args!["status" => pr.status()],
                                ))),
                                Err(e) => error.set(Some(i18n.t_args(
                                    "common-network-error",
                                    &fluent_args!["error" => e.to_string()],
                                ))),
                            }
                            invoice_to_edit.set(Some(invoice));
                        }
                        Err(e) => error.set(Some(i18n.t_args(
                            "vendor-invoice-table-error-parse-invoice",
                            &fluent_args!["error" => e.to_string()],
                        ))),
                    },
                    Ok(r) => error.set(Some(i18n.t_args(
                        "vendor-invoice-table-error-fetch-invoice",
                        &fluent_args!["status" => r.status()],
                    ))),
                    Err(e) => error.set(Some(i18n.t_args(
                        "common-network-error",
                        &fluent_args!["error" => e.to_string()],
                    ))),
                }
            });
        })
    };

    let on_drawer_close = {
        let invoice_to_edit = invoice_to_edit.clone();
        let partner_to_edit = partner_to_edit.clone();
        Callback::from(move |()| {
            invoice_to_edit.set(None);
            partner_to_edit.set(None);
        })
    };

    let on_drawer_change = {
        let invoice_id = invoice_to_edit.as_ref().map(|i| i.id);
        let on_edit_click = on_edit_click.clone();
        let fetch_invoices = fetch_invoices.clone();
        Callback::from(move |()| {
            if let Some(id) = invoice_id {
                on_edit_click.emit((id, InvoiceDrawerTab::General));
            }
            fetch_invoices.emit(());
        })
    };

    let i18n = use_locale();

    if *loading {
        return html! { <p>{ i18n.t("common-loading") }</p> };
    }
    if let Some(err) = &*error {
        return html! { <div class="message__error">{ err }</div> };
    }

    html! {
        <>
            if let (Some(invoice), Some(partner)) = (&*invoice_to_edit, &*partner_to_edit) {
                <VendorInvoiceDrawer
                    invoice={invoice.clone()}
                    partner={partner.clone()}
                    on_close={on_drawer_close}
                    on_change={on_drawer_change}
                    initial_tab={*initial_tab}
                />
            }
            <table class="table">
                <thead>
                    <tr>
                        <th class="table__text-col">{ i18n.t("common-vendor") }</th>
                        <th class="table__text-col">{ i18n.t("vendor-invoice-table-invoice-number") }</th>
                        <th class="table__value-col">{ i18n.t("vendor-invoice-table-invoice-date") }</th>
                        <th class="table__value-col">{ i18n.t("common-due-date") }</th>
                        <th class="table__value-col">{ i18n.t("common-net") }</th>
                        <th class="table__value-col">{ i18n.t("common-tax") }</th>
                        <th class="table__value-col">{ i18n.t("common-gross") }</th>
                        <th class="table__value-col">{ i18n.t("vendor-invoice-table-balance-due") }</th>
                        <th class="table__col-actions">{ i18n.t("common-actions") }</th>
                    </tr>
                </thead>
                <tbody>
                    { for (*invoices).iter().map(|invoice| {
                        let on_edit = {
                            let on_edit_click = on_edit_click.clone();
                            let invoice_id = invoice.id;
                            Callback::from(move |_| {
                                on_edit_click.emit((invoice_id, InvoiceDrawerTab::General));
                            })
                        };
                        let on_pay = {
                            let on_edit_click = on_edit_click.clone();
                            let invoice_id = invoice.id;
                            Callback::from(move |_| {
                                on_edit_click.emit((invoice_id, InvoiceDrawerTab::Payments));
                            })
                        };
                        let on_actions_toggle = {
                            let show_actions = show_actions.clone();
                            let invoice_id = invoice.id;
                            Callback::from(move |_| {
                                if show_actions.as_ref() == Some(&invoice_id) {
                                    show_actions.set(None);
                                } else {
                                    show_actions.set(Some(invoice_id));
                                }
                            })
                        };
                        html! {
                            <tr>
                                <td class="table__text-col">{ &invoice.partner_name }</td>
                                <td class="table__text-col">{ &invoice.invoice_number }</td>
                                <td class="table__value-col">{ i18n.format_date(invoice.issue_date) }</td>
                                <td class="table__value-col">{ i18n.format_date(invoice.due_date) }</td>
                                <td class="table__value-col">{ i18n.format_currency(invoice.net_amount) }</td>
                                <td class="table__value-col">{ i18n.format_currency(invoice.tax_amount) }</td>
                                <td class="table__value-col">{ i18n.format_currency(invoice.gross_amount) }</td>
                                <td class="table__value-col">{ i18n.format_currency(invoice.amount_remaining) }</td>
                                <td class="table__col-actions">
                                    <button class="btn-pay-action" disabled={invoice.amount_remaining == 0} onclick={on_pay}>
                                        <span class="btn-pay-icon">
                                            <img src="/images/credit-card.svg" alt="" style="width:100%; height:100%;" />
                                        </span>
                                        <span>{ i18n.t("common-pay") }</span>
                                    </button>
                                    <div class="actions-dropdown">
                                        <button class="icon-button" onclick={on_actions_toggle} title={i18n.t("common-actions")}>
                                            <img src="/images/more-vertical.svg" alt={i18n.t("common-actions")} class="dropdown-trigger-icon" />
                                        </button>
                                        if *show_actions == Some(invoice.id) {
                                            <div class="actions-dropdown__content">
                                                <button class="dropdown-item" onclick={on_edit}>
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
