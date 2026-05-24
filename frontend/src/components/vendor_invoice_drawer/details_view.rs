/*
 * Copyright (c) 2026-2026. Trevor Campbell and others.
 *
 * This file is part of KelpieBooks.
 *
 * KelpieBooks is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License,or
 * (at your option) any later version.
 *
 * KelpieBooks is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with KelpieBooks; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 *
 * Contributors:
 *      Trevor Campbell
 *
 */

use crate::api::Api;
use crate::contexts::auth_context::use_user_context;
use chrono::NaiveDate;
use fluent::fluent_args;
use gloo_timers::callback::Timeout;
use shared_core::i18n::{t, t_args};
use shared_core::models::vendor_invoice::VendorInvoice;
use shared_core::requests::vendor_invoice::UpdateVendorInvoiceRequest;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[derive(Properties, PartialEq, Clone)]
pub struct DetailsViewProps {
    pub invoice: VendorInvoice,
    pub on_change: Callback<()>,
}

#[function_component(DetailsView)]
pub fn details_view(props: &DetailsViewProps) -> Html {
    let user_ctx = use_user_context();
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
        let navigator = navigator.clone();
        let error = error.clone();
        let show_saved = show_saved.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let request = request.clone();
            let on_change = on_change.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            let error = error.clone();
            let show_saved = show_saved.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Api::put(&format!("/api/vendor-invoices/{}", request.id), &*request, user_ctx, navigator).await;
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
                    Ok(r) => error.set(Some(t_args(
                        "details-view-error-update",
                        &fluent_args!["status" => r.status()],
                    ))),
                    Err(e) => error.set(Some(t_args(
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
                    <label>{t("vendor-invoice-table-invoice-number")}</label>
                    <input type="text" value={request.invoice_number.clone()} oninput={on_input(|r, v| r.invoice_number = v)} required=true />

                    <label>{t("vendor-invoice-table-invoice-date")}</label>
                    <input type="date" value={request.issue_date.format("%Y-%m-%d").to_string()} onchange={on_date_change(|r, v| r.issue_date = v)} required=true />

                    <label>{t("common-due-date")}</label>
                    <input type="date" value={request.due_date.format("%Y-%m-%d").to_string()} onchange={on_date_change(|r, v| r.due_date = v)} required=true />

                    <label>{t("details-view-notes-label")}</label>
                    <textarea oninput={on_input(|r, v| r.notes = Some(v))} value={request.notes.clone().unwrap_or_default()} />
                </div>
                <div class="voucher-footer">
                    if let Some(e) = &*error {
                        <div class="error">{e}</div>
                    }
                    <div class="table-actions">
                        <button type="submit" class="button-primary">{ t("account-modal-save-button") }</button>
                    </div>
                    if *show_saved {
                        <span class="fade-out message__success" style="margin-left: 1rem;">{ t("common-saved") }</span>
                    }
                </div>
            </form>
        </div>
    }
}
