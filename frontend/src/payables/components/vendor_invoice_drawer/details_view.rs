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
use shared_core::payables::{
    models::vendor_invoice::VendorInvoice,
    requests::vendor_invoice::UpdateVendorInvoiceRequest,
};
use web_sys::HtmlInputElement;
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
pub struct DetailsViewProps {
    pub invoice: VendorInvoice,
    pub on_change: Callback<()>,
}

#[function_component(DetailsView)]
pub fn details_view(props: &DetailsViewProps) -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();
    let request = use_state(|| UpdateVendorInvoiceRequest {
        id: props.invoice.id,
        invoice_number: props.invoice.invoice_number.clone(),
        issue_date: props.invoice.issue_date,
        due_date: props.invoice.due_date,
        notes: props.invoice.notes.clone(),
    });
    let error = use_state(|| None::<String>);
    let show_saved = use_state(|| false);

    let on_input = |field_updater: fn(&mut UpdateVendorInvoiceRequest, String)| {
        let state = request.clone();
        Callback::from(move |e: InputEvent| {
            let mut info = (*state).clone();
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            field_updater(&mut info, value);
            state.set(info);
        })
    };

    let on_date_change = |field_updater: fn(&mut UpdateVendorInvoiceRequest, NaiveDate)| {
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
                    &format!("/api/vendor-invoices/{}", request.id),
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
                    <label>{i18n.t("vendor-invoice-table-invoice-number")}</label>
                    <input type="text" value={request.invoice_number.clone()} oninput={on_input(|r, v| r.invoice_number = v)} required=true />

                    <label>{i18n.t("vendor-invoice-table-invoice-date")}</label>
                    <input type="date" value={request.issue_date.format("%Y-%m-%d").to_string()} onchange={on_date_change(|r, v| r.issue_date = v)} required=true />

                    <label>{i18n.t("common-due-date")}</label>
                    <input type="date" value={request.due_date.format("%Y-%m-%d").to_string()} onchange={on_date_change(|r, v| r.due_date = v)} required=true />

                    <label>{i18n.t("details-view-notes-label")}</label>
                    <textarea oninput={on_input(|r, v| r.notes = Some(v))} value={request.notes.clone().unwrap_or_default()} />
                </div>
                <div class="voucher-footer">
                    if let Some(e) = &*error {
                        <div class="message__error">{e}</div>
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
