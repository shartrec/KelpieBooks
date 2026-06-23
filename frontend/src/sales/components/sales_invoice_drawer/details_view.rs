/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use chrono::NaiveDate;
use fluent::fluent_args;
use gloo_timers::callback::Timeout;
use shared_core::sales::{
    models::sales_invoice::SalesInvoice,
    requests::sales_invoice::UpdateSalesInvoiceRequest,
};
use web_sys::HtmlInputElement;
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
pub struct DetailsViewProps {
    pub invoice: SalesInvoice,
    pub on_change: Callback<()>,
}

#[function_component(DetailsView)]
pub fn details_view(props: &DetailsViewProps) -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let inv = &props.invoice;
    let navigator = use_navigator().unwrap();
    let request = use_state(|| UpdateSalesInvoiceRequest {
        id: props.invoice.id,
        issue_date: props.invoice.issue_date,
        due_date: props.invoice.due_date,
        billing_address_id: props.invoice.billing_address_id,
        shipping_address_id: props.invoice.shipping_address_id,
        bill_to: props.invoice.bill_to.clone(),
        ship_to: props.invoice.ship_to.clone(),
    });

    let error = use_state(|| None::<String>);
    let show_saved = use_state(|| false);

    let on_date_change = |field_updater: fn(&mut UpdateSalesInvoiceRequest, NaiveDate)| {
        let state = request.clone();
        Callback::from(move |e: Event| {
            let mut info = (*state).clone();
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            if let Ok(date) = NaiveDate::parse_from_str(&value, "%Y-%m-%d") {
                field_updater(&mut info, date);
                state.set(info);
            }
        })
    };

    let on_submit = {
        let request = request.clone();
        let on_change = props.on_change.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        let error = error.clone();
        let show_saved = show_saved.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let request = request.clone();
            let on_change = on_change.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            let error = error.clone();
            let show_saved = show_saved.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Api::put(
                    &format!("/api/sales-invoices/{}", request.id),
                    &*request,
                    user_ctx,
                    navigator,
                )
                .await;
                match resp {
                    Ok(r) if r.ok() => {
                        on_change.emit(());
                        {
                            show_saved.set(true);
                            let show_saved = show_saved.clone();
                            let timeout = Timeout::new(2000, move || {
                                show_saved.set(false);
                            });
                            timeout.forget();
                        }
                    }
                    Ok(r) => error.set(Some(i18n.t_args(
                        "details-view-error-update",
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

    html! {
        <div class="details-view">
            <form onsubmit={on_submit}>
                <div class="data-form">
                    <label>{ i18n.t("sales-invoice-table-invoice-number") }</label>
                    <label>{ inv.invoice_number.clone() }</label>

                    <label>{ i18n.t("sales-invoice-table-invoice-date") }</label>
                    <input type="date" value={inv.issue_date.format("%Y-%m-%d").to_string()} onchange={on_date_change(|r, v| r.issue_date = v)} required=true />

                    <label>{ i18n.t("common-due-date") }</label>
                    <input type="date" value={inv.due_date.format("%Y-%m-%d").to_string()} onchange={on_date_change(|r, v| r.due_date = v)} required=true />
                    <div>{ i18n.format_date(inv.due_date) }</div>
                </div>
                <div class="voucher-footer">
                    if let Some(e) = &*error {
                        <div class="error">{e}</div>
                    }
                    <div class="table-actions">
                        <button type="submit" class="button-primary">{ i18n.t("account-modal-save-button") }</button>
                    </div>
                    if *show_saved {
                        <span class="fade-out message__success" style="margin-left: 1rem;">{ i18n.t("common-saved") }</span>
                    }
                </div>
            </form>
        </div>
    }
}
