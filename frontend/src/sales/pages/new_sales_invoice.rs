/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::{Local, NaiveDate};
use fluent::fluent_args;
use rust_decimal::Decimal;
use shared_core::{
    partners::dtos::partner_list_item::PartnerListItem,
    sales::{
        models::{
            sales_invoice_item::SalesInvoiceLine,
        },
        requests::create_sales_invoice_request::CreateSalesInvoiceRequest,
    },
    sales::models::item::Item,
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
    router::Route,
    sales::components::sales_invoice_item_row::SalesInvoiceItemRow,
};

#[function_component(NewSalesInvoicePage)]
pub fn new_sales_invoice_page() -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();
    let request = use_state(|| {
        let today = Local::now().date_naive();
        CreateSalesInvoiceRequest {
            partner_id: Uuid::nil(),
            invoice_number: String::new(),
            issue_date: today,
            due_date: today,
            lines: vec![SalesInvoiceLine {
                id: Uuid::new_v4(),
                invoice_id: Uuid::nil(),
                item_id: Uuid::nil(),
                description: String::new(),
                quantity: Decimal::ONE,
                unit_price: Decimal::ZERO,
                tax_category_id: None,
                tax_amount: Decimal::ZERO,
                line_total: Default::default(),
                sort_order: 1,
            }],
        }
    });
    let customers = use_state(Vec::new);
    let items = use_state(Vec::new);
    let error = use_state(|| None::<String>);

    let fetch_data = {
        let customers = customers.clone();
        let items = items.clone();
        let error = error.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        Callback::from(move |_: ()| {
            let customers = customers.clone();
            let items = items.clone();
            let error = error.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_customers =
                    Api::get("/api/partners", user_ctx.clone(), navigator.clone()).await;
                match fetched_customers {
                    Ok(response) if response.ok() => {
                        match response.json::<Vec<PartnerListItem>>().await {
                            Ok(data) => {
                                customers.set(data.into_iter().filter(|p| p.is_customer).collect())
                            }
                            Err(e) => error.set(Some(i18n.t_args(
                                "new-sales-invoice-error-parse-customers",
                                &fluent_args!["error" => e.to_string()],
                            ))),
                        }
                    }
                    Ok(response) => error.set(Some(i18n.t_args(
                        "new-sales-invoice-error-fetch-customers",
                        &fluent_args!["status" => response.status()],
                    ))),
                    Err(e) => error.set(Some(i18n.t_args(
                        "common-network-error",
                        &fluent_args!["error" => e.to_string()],
                    ))),
                }

                let fetched_items = Api::get("/api/items", user_ctx, navigator).await;
                match fetched_items {
                    Ok(response) if response.ok() => match response.json::<Vec<Item>>().await {
                        Ok(data) => items.set(data),
                        Err(e) => error.set(Some(i18n.t_args(
                            "new-sales-invoice-error-parse-items",
                            &fluent_args!["error" => e.to_string()],
                        ))),
                    },
                    Ok(response) => error.set(Some(i18n.t_args(
                        "new-sales-invoice-error-fetch-items",
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

    let on_input = |field_updater: fn(&mut CreateSalesInvoiceRequest, String)| {
        let state = request.clone();
        Callback::from(move |e: InputEvent| {
            let mut info = (*state).clone();
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            field_updater(&mut info, value);
            state.set(info);
        })
    };

    let on_date_change = |field_updater: fn(&mut CreateSalesInvoiceRequest, NaiveDate)| {
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
        Callback::from(move |item: SalesInvoiceLine| {
            let mut req = (*request).clone();
            if let Some(pos) = req.lines.iter().position(|i| i.id == item.id) {
                req.lines[pos] = item;
                request.set(req);
            }
        })
    };

    let on_item_delete = {
        let request = request.clone();
        Callback::from(move |id: Uuid| {
            let mut req = (*request).clone();
            req.lines.retain(|i| i.id != id);
            request.set(req);
        })
    };

    let add_item = {
        let request = request.clone();
        Callback::from(move |_| {
            let mut req = (*request).clone();
            req.lines.push(SalesInvoiceLine {
                id: Uuid::new_v4(),
                invoice_id: Uuid::nil(),
                item_id: Uuid::nil(),
                description: String::new(),
                quantity: Decimal::ONE,
                unit_price: Decimal::ZERO,
                tax_category_id: None,
                tax_amount: Decimal::ZERO,
                line_total: Decimal::ZERO,
                sort_order: (req.lines.len() + 1) as i32,
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
                    "/api/sales-invoices",
                    &*request,
                    user_ctx,
                    navigator.clone(),
                )
                .await;
                match resp {
                    Ok(r) if r.ok() => {
                        navigator.push(&Route::Home); // TODO: redirect to sales invoice list page
                    }
                    Ok(r) => error.set(Some(i18n.t_args(
                        "new-sales-invoice-error-create-invoice",
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
            <h1>{ i18n.t("new-sales-invoice-title") }</h1>
            <form onsubmit={on_submit} class="voucher__form">
                <div class="data-form">
                    <label>{i18n.t("common-customer")}</label>
                    <select onchange={on_partner_change} required=true>
                        <option value="" disabled=true selected=true>{i18n.t("new-sales-invoice-select-customer")}</option>
                        { for (*customers).iter().map(|customer| html! {
                            <option value={customer.id.to_string()}>{&customer.legal_name}</option>
                        })}
                    </select>

                    <label>{i18n.t("new-sales-invoice-number-label")}</label>
                    <input type="text" class="voucher__form__invoice" oninput={on_input(|r, v| r.invoice_number = v)} required=true />

                    <label>{i18n.t("new-sales-invoice-date-label")}</label>
                    <input type="date" value={request.issue_date.format("%Y-%m-%d").to_string()} onchange={on_date_change(|r, v| r.issue_date = v)} required=true />

                    <label>{i18n.t("new-sales-invoice-due-date-label")}</label>
                    <input type="date" value={request.due_date.format("%Y-%m-%d").to_string()} onchange={on_date_change(|r, v| r.due_date = v)} required=true />
                </div>

                <div class="voucher__entries">
                    <div class="voucher__entry-header">
                        <span>{i18n.t("common-description")}</span>
                        <span>{i18n.t("common-item")}</span>
                        <span>{i18n.t("common-quantity")}</span>
                        <span>{i18n.t("common-price")}</span>
                        <span>{i18n.t("common-tax")}</span>
                        <span>{i18n.t("common-total")}</span>
                        <span></span>
                    </div>
                    { for request.lines.iter().map(|item| html! {
                        <SalesInvoiceItemRow
                            item={item.clone()}
                            items={(*items).clone()}
                            on_change={on_item_change.clone()}
                            on_delete={on_item_delete.clone()}
                        />
                    })}
                </div>
                <div class="table-actions">
                    <button type="button" class="button-primary" onclick={add_item}>{ i18n.t("new-sales-invoice-add-line-button") }</button>
                </div>
                <div class="voucher-footer">
                    if let Some(e) = &*error {
                        <div class="error">{e}</div>
                    }
                    <button type="submit" class="button-primary">{ i18n.t("new-sales-invoice-save-button") }</button>
                </div>
            </form>
        </Layout>
    }
}
