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
};
use fluent::fluent_args;
use rust_decimal::dec;
use shared_core::{
    ledger::models::{
        account::Account,
        account_category::AccountCategory,
    },
    partners::dtos::partner_list_item::PartnerListItem,
    payables::{
        models::vendor_invoice_item::VendorInvoiceItem,
        requests::vendor_invoice::CreateVendorInvoiceRequest,
    },
};
use uuid::Uuid;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{
    api::Api,
    core::components::layout::Layout,
    contexts::{
        auth_context::use_user_context,
        locale_context::use_locale,
    },
    payables::components::vendor_invoice_item_row::VendorInvoiceItemRow,
    router::Route,
};

#[function_component(NewVendorInvoicePage)]
pub fn new_vendor_invoice_page() -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();
    let request = use_state(|| {
        let today = Local::now().date_naive();
        CreateVendorInvoiceRequest {
            issue_date: today,
            due_date: today,
            items: vec![VendorInvoiceItem {
                id: Uuid::new_v4(),
                vendor_invoice_id: Uuid::nil(),
                account_id: Uuid::nil(),
                description: String::new(),
                net_amount: dec!(0.00),
                tax_amount: dec!(0.00),
                total_amount: dec!(0.00),
            }],
            ..Default::default()
        }
    });
    let vendors = use_state(Vec::new);
    let accounts = use_state(Vec::new);
    let error = use_state(|| None::<String>);

    let fetch_data = {
        let vendors = vendors.clone();
        let accounts = accounts.clone();
        let error = error.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        Callback::from(move |_: ()| {
            let vendors = vendors.clone();
            let accounts = accounts.clone();
            let error = error.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_vendors =
                    Api::get("/api/partners", user_ctx.clone(), navigator.clone()).await;
                match fetched_vendors {
                    Ok(response) if response.ok() => {
                        match response.json::<Vec<PartnerListItem>>().await {
                            Ok(data) => {
                                vendors.set(data.into_iter().filter(|p| p.is_vendor).collect())
                            }
                            Err(e) => error.set(Some(i18n.t_args(
                                "new-vendor-invoice-error-parse-vendors",
                                &fluent_args!["error" => e.to_string()],
                            ))),
                        }
                    }
                    Ok(response) => error.set(Some(i18n.t_args(
                        "new-vendor-invoice-error-fetch-vendors",
                        &fluent_args!["status" => response.status()],
                    ))),
                    Err(e) => error.set(Some(i18n.t_args(
                        "common-network-error",
                        &fluent_args!["error" => e.to_string()],
                    ))),
                }

                let url = format!(
                    "/api/accounts_by_category/{}",
                    AccountCategory::Expense.to_string()
                );
                let fetched_accounts = Api::get(&url, user_ctx, navigator).await;
                match fetched_accounts {
                    Ok(response) if response.ok() => match response.json::<Vec<Account>>().await {
                        Ok(data) => accounts.set(data),
                        Err(e) => error.set(Some(i18n.t_args(
                            "new-vendor-invoice-error-parse-accounts",
                            &fluent_args!["error" => e.to_string()],
                        ))),
                    },
                    Ok(response) => error.set(Some(i18n.t_args(
                        "new-vendor-invoice-error-fetch-accounts",
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

    use_effect_with((), move |()| {
        fetch_data.emit(());
        || ()
    });

    let on_partner_change = {
        let state = request.clone();
        Callback::from(move |e: Event| {
            let mut info = (*state).clone();
            let value = e
                .target_unchecked_into::<web_sys::HtmlSelectElement>()
                .value();
            info.partner_id = Uuid::parse_str(&value).unwrap_or_default();
            state.set(info);
        })
    };

    let on_input = |field_updater: fn(&mut CreateVendorInvoiceRequest, String)| {
        let state = request.clone();
        Callback::from(move |e: InputEvent| {
            let mut info = (*state).clone();
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            field_updater(&mut info, value);
            state.set(info);
        })
    };

    let on_date_change = |field_updater: fn(&mut CreateVendorInvoiceRequest, NaiveDate)| {
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

    let on_item_change = {
        let request = request.clone();
        Callback::from(move |item: VendorInvoiceItem| {
            let mut req = (*request).clone();
            if let Some(pos) = req.items.iter().position(|i| i.id == item.id) {
                req.items[pos] = item;
                request.set(req);
            }
        })
    };

    let on_item_delete = {
        let request = request.clone();
        Callback::from(move |id: Uuid| {
            let mut req = (*request).clone();
            req.items.retain(|i| i.id != id);
            request.set(req);
        })
    };

    let add_item = {
        let request = request.clone();
        Callback::from(move |_| {
            let mut req = (*request).clone();
            let last_account_id = req.items.last().map_or(Uuid::nil(), |item| item.account_id);
            req.items.push(VendorInvoiceItem {
                id: Uuid::new_v4(),
                vendor_invoice_id: Uuid::nil(),
                account_id: last_account_id,
                description: String::new(),
                net_amount: dec!(0.00),
                tax_amount: dec!(0.00),
                total_amount: dec!(0.00),
            });
            request.set(req);
        })
    };

    let on_submit = {
        let request = request.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        let error = error.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let request = request.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Api::post(
                    "/api/vendor-invoices",
                    &*request,
                    user_ctx,
                    navigator.clone(),
                )
                .await;
                match resp {
                    Ok(r) if r.ok() => {
                        navigator.push(&Route::Payables);
                    }
                    Ok(r) => error.set(Some(i18n.t_args(
                        "new-vendor-invoice-error-create-invoice",
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
        <Layout>
            <h1>{ i18n.t("new-vendor-invoice-title") }</h1>
            <form onsubmit={on_submit} class="voucher__form">
                <div class="data-form">
                    <label>{i18n.t("common-vendor")}</label>
                    <select onchange={on_partner_change} required=true>
                        <option value="" disabled=true selected=true>{i18n.t("new-vendor-invoice-select-vendor")}</option>
                        { for (*vendors).iter().map(|vendor| html! {
                            <option value={vendor.id.to_string()}>{&vendor.legal_name}</option>
                        })}
                    </select>

                    <label>{i18n.t("new-vendor-invoice-number-label")}</label>
                    <input type="text" class="voucher__form__invoice" oninput={on_input(|r, v| r.invoice_number = v)} required=true />

                    <label>{i18n.t("new-vendor-invoice-date-label")}</label>
                    <input type="date" value={request.issue_date.format("%Y-%m-%d").to_string()} onchange={on_date_change(|r, v| r.issue_date = v)} required=true />

                    <label>{i18n.t("new-vendor-invoice-due-date-label")}</label>
                    <input type="date" value={request.due_date.format("%Y-%m-%d").to_string()} onchange={on_date_change(|r, v| r.due_date = v)} required=true />
                </div>

                <div class="voucher__entries">
                    <div class="voucher__entry-header">
                                <span>{i18n.t("common-description")}</span>
                                <span>{i18n.t("common-account")}</span>
                                <span>{i18n.t("new-vendor-invoice-net-amount")}</span>
                                <span>{i18n.t("new-vendor-invoice-tax-amount")}</span>
                                <span>{i18n.t("common-total")}</span>
                                <span></span>
                    </div>
                            { for request.items.iter().map(|item| html! {
                                <VendorInvoiceItemRow
                                    item={item.clone()}
                                    accounts={(*accounts).clone()}
                                    on_change={on_item_change.clone()}
                                    on_delete={on_item_delete.clone()}
                                />
                            })}
                </div>
                <div class="table-actions">
                    <button type="button" class="button-primary" onclick={add_item}>{ i18n.t("new-vendor-invoice-add-line-button") }</button>
                </div>
                <div class="voucher-footer">
                    if let Some(e) = &*error {
                        <div class="error">{e}</div>
                    }
                    <button type="submit" class="button-primary">{ i18n.t("new-vendor-invoice-save-button") }</button>
                </div>
            </form>
        </Layout>
    }
}
