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

use yew::prelude::*;
use shared_core::models::vendor_invoice::VendorInvoice;
use shared_core::models::vendor_invoice_payment::VendorInvoicePayment;
use crate::api::Api;
use crate::contexts::auth_context::use_user_context;
use yew_router::prelude::use_navigator;
use shared_core::dtos::account_with_balance::AccountWithBalance;
use uuid::Uuid;
use shared_core::requests::vendor_invoice_payment::CreateVendorInvoicePaymentRequest;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use chrono::{Local, NaiveDate};
use shared_core::models::account_category::AccountCategory;

#[derive(Properties, PartialEq, Clone)]
pub struct PaymentsViewProps {
    pub invoice: VendorInvoice,
    pub on_change: Callback<()>,
}

#[function_component(PaymentsView)]
pub fn payments_view(props: &PaymentsViewProps) -> Html {
    let user_ctx = use_user_context();
    let navigator = use_navigator().unwrap();
    let payments = use_state(Vec::new);
    let accounts = use_state(Vec::new);
    let error = use_state(|| None::<String>);
    let request = use_state(|| CreateVendorInvoicePaymentRequest {
        vendor_invoice_id: props.invoice.id,
        payment_date: Local::now().date_naive(),
        bank_account_id: Uuid::nil(),
        amount: props.invoice.amount_remaining,
        reference: None,
    });

    let fetch_payments = {
        let payments = payments.clone();
        let error = error.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        let invoice_id = props.invoice.id;
        Callback::from(move |_: ()| {
            let payments = payments.clone();
            let error = error.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_payments = Api::get(&format!("/api/vendor-invoices/{}/payments", invoice_id), user_ctx, navigator).await;
                match fetched_payments {
                    Ok(response) if response.ok() => {
                        match response.json::<Vec<VendorInvoicePayment>>().await {
                            Ok(data) => payments.set(data),
                            Err(e) => error.set(Some(format!("Failed to parse payments: {}", e))),
                        }
                    }
                    Ok(response) => error.set(Some(format!("Failed to fetch payments: {}", response.status()))),
                    Err(e) => error.set(Some(format!("Network error: {}", e))),
                }
            });
        })
    };

    let fetch_accounts = {
        let accounts = accounts.clone();
        let error = error.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        Callback::from(move |_: ()| {
            let accounts = accounts.clone();
            let error = error.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_accounts = Api::get("/api/accounts_with_balances", user_ctx, navigator).await;
                match fetched_accounts {
                    Ok(response) if response.ok() => {
                        match response.json::<Vec<AccountWithBalance>>().await {
                            Ok(data) => accounts.set(data.into_iter().filter(|a| a.category == AccountCategory::Asset).collect()),
                            Err(e) => error.set(Some(format!("Failed to parse accounts: {}", e))),
                        }
                    }
                    Ok(response) => error.set(Some(format!("Failed to fetch accounts: {}", response.status()))),
                    Err(e) => error.set(Some(format!("Network error: {}", e))),
                }
            });
        })
    };

    use_effect_with((), move |()| {
        fetch_payments.emit(());
        fetch_accounts.emit(());
        || ()
    });

    let on_input = |field_updater: fn(&mut CreateVendorInvoicePaymentRequest, String)| {
        let state = request.clone();
        Callback::from(move |e: InputEvent| {
            let mut info = (*state).clone();
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            field_updater(&mut info, value);
            state.set(info);
        })
    };

    let on_date_change = |field_updater: fn(&mut CreateVendorInvoicePaymentRequest, NaiveDate)| {
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

    let on_select_change = |field_updater: fn(&mut CreateVendorInvoicePaymentRequest, String)| {
        let state = request.clone();
        Callback::from(move |e: Event| {
            let mut info = (*state).clone();
            let value = e.target_unchecked_into::<HtmlSelectElement>().value();
            field_updater(&mut info, value);
            state.set(info);
        })
    };

    let on_submit = {
        let request = request.clone();
        let on_change = props.on_change.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        let error = error.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let request = request.clone();
            let on_change = on_change.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Api::post("/api/vendor-invoice-payments", &*request, user_ctx, navigator).await;
                match resp {
                    Ok(r) if r.ok() => {
                        on_change.emit(());
                    }
                    Ok(r) => error.set(Some(format!("Failed to make payment: {}", r.status()))),
                    Err(e) => error.set(Some(format!("Network error: {}", e))),
                }
            });
        })
    };

    html! {
        <div class="payments-view">
            <form onsubmit={on_submit}>
                <div class="data-form">
                    <label>{"Payment Date:"}</label>
                    <input type="date" value={request.payment_date.format("%Y-%m-%d").to_string()} onchange={on_date_change(|r, v| r.payment_date = v)} required=true />

                    <label>{"Bank Account:"}</label>
                    <select onchange={on_select_change(|r, v| r.bank_account_id = Uuid::parse_str(&v).unwrap_or_default())} required=true>
                        <option value="" disabled=true selected=true>{"Select an account"}</option>
                        { for (*accounts).iter().map(|account| html! {
                            <option value={account.id.to_string()}>{&account.name}</option>
                        })}
                    </select>

                    <label>{"Amount:"}</label>
                    <input type="number" value={request.amount.to_string()} oninput={on_input(|r, v| r.amount = v.parse().unwrap_or(0))} required=true />

                    <label>{"Reference:"}</label>
                    <input type="text" value={request.reference.as_deref().unwrap_or("").to_string()} oninput={on_input(|r, v| r.reference = Some(v))} />
                </div>
                <div class="voucher-footer">
                    if let Some(e) = &*error {
                        <div class="error">{e}</div>
                    }
                    <button type="submit" class="button-primary">{ "Make Payment" }</button>
                </div>
            </form>

            <table class="table">
                <thead>
                    <tr>
                        <th>{"Date"}</th>
                        <th>{"Amount"}</th>
                        <th>{"Reference"}</th>
                    </tr>
                </thead>
                <tbody>
                    { for (*payments).iter().map(|payment| html! {
                        <tr>
                            <td>{payment.payment_date.format("%d %b %Y").to_string()}</td>
                            <td>{payment.amount}</td>
                            <td>{payment.reference.as_deref().unwrap_or("")}</td>
                        </tr>
                    })}
                </tbody>
            </table>
        </div>
    }
}
