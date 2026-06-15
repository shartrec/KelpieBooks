/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::{
    Local,
    NaiveDate,
    Utc,
};
use fluent::fluent_args;
use rust_decimal::Decimal;
use shared_core::{
    ledger::models::account::Account,
    payables::{
        models::{
            vendor_invoice::VendorInvoice,
            vendor_payment::VendorPayment,
            vendor_payment_allocation::VendorPaymentAllocation,
        },
        requests::vendor_payment::CreateVendorPaymentRequest,
    },
};
use uuid::Uuid;
use web_sys::{
    HtmlInputElement,
    HtmlSelectElement,
};
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{
    api::Api,
    core::components::currency_input::CurrencyInput,
    contexts::{
        auth_context::use_user_context,
        locale_context::use_locale,
    },
};

#[derive(Properties, PartialEq, Clone)]
pub struct PaymentsViewProps {
    pub invoice: VendorInvoice,
    pub on_change: Callback<()>,
}

#[function_component(PaymentsView)]
pub fn payments_view(props: &PaymentsViewProps) -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();
    let payments = use_state(Vec::new);
    let accounts = use_state(Vec::new);
    let error = use_state(|| None::<String>);
    let request = use_state(|| {
        let user = user_ctx.user.as_ref().unwrap();
        CreateVendorPaymentRequest {
            partner_id: props.invoice.partner_id,
            payment_date: Local::now().date_naive(),
            bank_account_id: Uuid::nil(),
            amount: props.invoice.amount_remaining,
            reference: None,
            allocations: vec![VendorPaymentAllocation {
                id: Uuid::new_v4(),
                organization_id: user.organization_id,
                vendor_invoice_id: props.invoice.id,
                vendor_payment_id: Uuid::nil(),
                allocated_amount: props.invoice.amount_remaining,
                created_at: Utc::now(),
            }],
        }
    });

    let fetch_payments = {
        let payments = payments.clone();
        let error = error.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        let invoice_id = props.invoice.id;
        Callback::from(move |_: ()| {
            let payments = payments.clone();
            let error = error.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_payments = Api::get(
                    &format!("/api/vendor-invoices/{}/payments", invoice_id),
                    user_ctx,
                    navigator,
                )
                .await;
                match fetched_payments {
                    Ok(response) if response.ok() => {
                        match response.json::<Vec<VendorPayment>>().await {
                            Ok(data) => payments.set(data),
                            Err(e) => error.set(Some(i18n.t_args(
                                "payments-view-error-parse-payments",
                                &fluent_args!["error" => e.to_string()],
                            ))),
                        }
                    }
                    Ok(response) => error.set(Some(i18n.t_args(
                        "payments-view-error-fetch-payments",
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

    let fetch_accounts = {
        let accounts = accounts.clone();
        let error = error.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        Callback::from(move |_: ()| {
            let accounts = accounts.clone();
            let error = error.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_accounts =
                    Api::get("/api/accounts/payment-methods", user_ctx, navigator).await;
                match fetched_accounts {
                    Ok(response) if response.ok() => match response.json::<Vec<Account>>().await {
                        Ok(data) => accounts.set(data),
                        Err(e) => error.set(Some(i18n.t_args(
                            "payments-view-error-parse-accounts",
                            &fluent_args!["error" => e.to_string()],
                        ))),
                    },
                    Ok(response) => error.set(Some(i18n.t_args(
                        "payments-view-error-fetch-accounts",
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

    let fetch_payments_clone = fetch_payments.clone();
    use_effect_with((), move |()| {
        fetch_payments_clone.emit(());
        fetch_accounts.emit(());
        || ()
    });

    let on_input = |field_updater: fn(&mut CreateVendorPaymentRequest, String)| {
        let state = request.clone();
        Callback::from(move |e: InputEvent| {
            let mut info = (*state).clone();
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            field_updater(&mut info, value);
            state.set(info);
        })
    };

    let on_date_change = |field_updater: fn(&mut CreateVendorPaymentRequest, NaiveDate)| {
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

    let on_select_change = |field_updater: fn(&mut CreateVendorPaymentRequest, String)| {
        let state = request.clone();
        Callback::from(move |e: Event| {
            let mut info = (*state).clone();
            let value = e.target_unchecked_into::<HtmlSelectElement>().value();
            field_updater(&mut info, value);
            state.set(info);
        })
    };

    let on_amount_change = {
        let request = request.clone();
        Callback::from(move |value: Decimal| {
            let mut new_request = (*request).clone();
            new_request.amount = value;
            new_request.allocations[0].allocated_amount = value;
            request.set(new_request);
        })
    };

    let on_submit = {
        let request = request.clone();
        let on_change = props.on_change.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        let i18n = i18n.clone();
        let error = error.clone();
        let fetch_payments = fetch_payments.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let request = request.clone();
            let on_change = on_change.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            let error = error.clone();
            let fetch_payments = fetch_payments.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Api::post("/api/vendor-payments", &*request, user_ctx, navigator).await;
                match resp {
                    Ok(r) if r.ok() => {
                        on_change.emit(());
                        fetch_payments.emit(());
                    }
                    Ok(r) => error.set(Some(i18n.t_args(
                        "payments-view-error-make-payment",
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

    let i18n = use_locale();

    html! {
        <div class="payments-view">
            <form onsubmit={on_submit}>
                <div class="data-form">
                    <label>{i18n.t("payments-view-payment-date-label")}</label>
                    <input type="date" value={request.payment_date.format("%Y-%m-%d").to_string()} onchange={on_date_change(|r, v| r.payment_date = v)} required=true />

                    <label>{i18n.t("payments-view-bank-account-label")}</label>
                    <select onchange={on_select_change(|r, v| r.bank_account_id = Uuid::parse_str(&v).unwrap_or_default())} required=true>
                        <option value="" disabled=true selected=true>{i18n.t("journal-entry-select-account")}</option>
                        { for (*accounts).iter().map(|account| html! {
                            <option value={account.id.to_string()}>{&account.name}</option>
                        })}
                    </select>

                    <label>{i18n.t("common-amount")}</label>
                    <CurrencyInput value={request.amount} on_change={on_amount_change} />

                    <label>{i18n.t("payments-view-reference-label")}</label>
                    <input type="text" value={request.reference.as_deref().unwrap_or("").to_string()} oninput={on_input(|r, v| r.reference = Some(v))} />
                </div>
                <div class="voucher-footer">
                    if let Some(e) = &*error {
                        <div class="error">{e}</div>
                    }
                    <button type="submit" class="button-primary">{ i18n.t("payments-view-make-payment-button") }</button>
                </div>
            </form>

            <table class="table">
                <thead>
                    <tr>
                        <th>{i18n.t("common-date")}</th>
                        <th>{i18n.t("common-amount")}</th>
                        <th>{i18n.t("payments-view-reference-label")}</th>
                    </tr>
                </thead>
                <tbody>
                    { for (*payments).iter().map(|payment| html! {
                        <tr>
                            <td>{ i18n.format_date(payment.payment_date) }</td>
                            <td>{i18n.format_currency(payment.amount)}</td>
                            <td>{payment.reference.as_deref().unwrap_or("")}</td>
                        </tr>
                    })}
                </tbody>
            </table>
        </div>
    }
}
