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
use rust_decimal::Decimal;
use shared_core::{
    partners::{
        dtos::partner_list_item::PartnerListItem,
        models::{
            address_type::AddressType,
            partner_address::PartnerAddress,
        },
    },
    sales::{
        models::{
            invoice_address::InvoiceAddress,
            sales_invoice::SalesInvoice,
            sales_invoice_item::SalesInvoiceLine,
        },
        requests::sales_invoice::CreateSalesInvoiceRequest,
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
    contexts::{
        auth_context::use_user_context,
        locale_context::use_locale,
    },
    core::components::{
        layout::Layout,
        progressive_search::ProgressiveSearch,
        SearchableItem,
    },
    sales::components::sales_invoice_item_row::SalesInvoiceItemRow,
};

#[derive(PartialEq, Clone, Copy)]
enum AddressTab {
    Billing,
    Shipping,
}
#[function_component(NewSalesInvoicePage)]
pub fn new_sales_invoice_page() -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();
    let active_tab = use_state(|| AddressTab::Billing);

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
                name: String::new(),
                quantity: Decimal::ONE,
                unit_price: Decimal::ZERO,
                tax_category_id: None,
                tax_amount: Decimal::ZERO,
                tax_rate: Decimal::ZERO,
                line_total: Decimal::ZERO, // Ensure line_total is zero for new lines
                sort_order: 1,
            }],
            // New address fields (initialized with sensible defaults)
            billing_address_id: None,
            shipping_address_id: None,
            bill_to: InvoiceAddress::default(),
            ship_to: InvoiceAddress::default(),
        }
    });
    let customers = use_state(Vec::new);
    let customer_search = use_state(String::new);
    let error = use_state(|| None::<String>);
    let success = use_state(|| None::<String>);
    let partner_addresses = use_state(Vec::<PartnerAddress>::new);

    let on_partner_search = {
        let customers = customers.clone();
        let customer_search = customer_search.clone();
        let error = error.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        Callback::from(move |text: String| {
            customer_search.set(text.clone());
            let customers = customers.clone();
            let error = error.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_customers = Api::get(
                    &format!("/api/partners/search?term={}", text),
                    user_ctx,
                    navigator,
                )
                .await;
                match fetched_customers {
                    Ok(response) if response.ok() => {
                        match response.json::<Vec<PartnerListItem>>().await {
                            Ok(data) => customers.set(data),
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
            });
        })
    };

    let on_partner_select = {
        let state = request.clone();
        let customers = customers.clone();
        let customer_search = customer_search.clone();
        let partner_addresses_state = partner_addresses.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        let error_for_select = error.clone();
        Callback::from(move |customer: PartnerListItem| {
            let mut info = (*state).clone();
            info.partner_id = customer.id;
            state.set(info);
            let display_name = customer.display_label();
            customer_search.set(display_name.clone());
            customers.set(vec![]);

            // Fetch partner addresses and set sensible defaults for billing/shipping
            let partner_id = customer.id;
            let partner_addresses_state = partner_addresses_state.clone();
            let state2 = state.clone();
            let user_ctx2 = user_ctx.clone();
            let navigator2 = navigator.clone();
            let error2 = error_for_select.clone();
            let i18n2 = i18n.clone();
            let display_name2 = display_name.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match Api::get(
                    &format!("/api/partners/{}/addresses", partner_id),
                    user_ctx2,
                    navigator2,
                )
                .await
                {
                    Ok(resp) if resp.ok() => {
                        match resp.json::<Vec<PartnerAddress>>().await {
                            Ok(addresses) => {
                                partner_addresses_state.set(addresses.clone());
                                // Choose defaults: look for explicit billing/shipping; fall back to first
                                let mut req = (*state2).clone();
                                if let Some(bill) = addresses
                                    .iter()
                                    .find(|a| matches!(a.address_type, AddressType::Billing))
                                    .or_else(|| addresses.first())
                                {
                                    req.billing_address_id = Some(bill.id);
                                    req.bill_to = InvoiceAddress {
                                        name: Some(display_name2.clone()),
                                        attention: req.bill_to.attention.clone(),
                                        address_line1: Some(bill.address_line1.clone()),
                                        address_line2: bill.address_line2.clone(),
                                        city: Some(bill.city.clone()),
                                        state_province: bill.state_province.clone(),
                                        postal_code: bill.postal_code.clone(),
                                        country: Some(bill.country.clone()),
                                    };
                                }
                                if let Some(ship) = addresses
                                    .iter()
                                    .find(|a| matches!(a.address_type, AddressType::Shipping))
                                    .or_else(|| addresses.first())
                                {
                                    req.shipping_address_id = Some(ship.id);
                                    req.ship_to = InvoiceAddress {
                                        name: Some(display_name2.clone()),
                                        attention: req.ship_to.attention.clone(),
                                        address_line1: Some(ship.address_line1.clone()),
                                        address_line2: ship.address_line2.clone(),
                                        city: Some(ship.city.clone()),
                                        state_province: ship.state_province.clone(),
                                        postal_code: ship.postal_code.clone(),
                                        country: Some(ship.country.clone()),
                                    };
                                }
                                state2.set(req);
                            }
                            Err(e) => {
                                error2.set(Some(i18n2.t_args(
                                    "new-sales-invoice-error-parse-addresses",
                                    &fluent_args!["error" => e.to_string()],
                                )));
                            }
                        }
                    }
                    Ok(resp) => {
                        error2.set(Some(i18n2.t_args(
                            "new-sales-invoice-error-fetch-addresses",
                            &fluent_args!["status" => resp.status()],
                        )));
                    }
                    Err(e) => {
                        error2.set(Some(i18n2.t_args(
                            "common-network-error",
                            &fluent_args!["error" => e.to_string()],
                        )));
                    }
                }
            });
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
                name: String::new(),
                description: String::new(),
                quantity: Decimal::ONE,
                unit_price: Decimal::ZERO,
                tax_category_id: None,
                tax_amount: Decimal::ZERO,
                tax_rate: Decimal::ZERO,
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
        let success = success.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let request = request.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            let error = error.clone();
            let success = success.clone();
            // remove any empty invoice rows
            let mut req = (*request).clone();
            req.lines.retain(|line| line.item_id != Uuid::nil());
            request.set(req);

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
                        match r.json::<SalesInvoice>().await {
                            Ok(invoice) => {
                                // Update local state with returned invoice number so we can render it in the UI
                                let mut current = (*request).clone();
                                current.invoice_number = invoice.invoice_number.clone();
                                request.set(current);
                                // Show a success message with the created invoice number
                                success.set(Some(i18n.t_args(
                                    "new-sales-invoice-success",
                                    &fluent_args!["number" => invoice.invoice_number.clone()],
                                )));
                                // Optionally, you can keep the user on the page to add another invoice
                                // or later navigate to a detail page if desired.
                            }
                            Err(e) => error.set(Some(i18n.t_args(
                                "new-sales-invoice-error-parse-response",
                                &fluent_args!["error" => e.to_string()],
                            ))),
                        }
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
            <form onsubmit={on_submit} class="sale__form">
                <div class="sale__form__header">
                    if !request.invoice_number.is_empty() {
                        <div style="margin-left: auto; display: flex; align-items: center; gap: 0.5rem;">
                            <label>{ i18n.t("new-sales-invoice-number-label") }{":"}</label>
                            <span class="sale__form__invoice">{ request.invoice_number.clone() }</span>
                        </div>
                    }
                </div>
                <div class="data-form">
                    <label>{i18n.t("common-customer")}</label>
                    <ProgressiveSearch<PartnerListItem>
                        placeholder="Search partners..."
                        query={(*customer_search).clone()}
                        suggestions={(*customers).clone()}
                        on_input={on_partner_search}
                        on_select={on_partner_select}
                    />
                </div>
            {
            if request.partner_id != Uuid::nil() {
                html! {
                    <>
                    <div class="data-form">
                        <label>{i18n.t("new-sales-invoice-date-label")}</label>
                        <input type="date" value={request.issue_date.format("%Y-%m-%d").to_string()} onchange={on_date_change(|r, v| r.issue_date = v)} required=true />

                        <label>{i18n.t("new-sales-invoice-due-date-label")}</label>
                        <input type="date" value={request.due_date.format("%Y-%m-%d").to_string()} onchange={on_date_change(|r, v| r.due_date = v)} required=true />
                    </div>


                    <div class="address-section">
                        <div class="tabs-nav">
                            <button type="button"
                                class={classes!("tab-button", (*active_tab == AddressTab::Billing).then_some("active"))}
                                onclick={let active_tab = active_tab.clone(); Callback::from(move |_| active_tab.set(AddressTab::Billing))}
                            >
                                { i18n.t("new-sales-invoice-billing-address") }
                            </button>
                            <button type="button"
                                class={classes!("tab-button", (*active_tab == AddressTab::Shipping).then_some("active"))}
                                onclick={let active_tab = active_tab.clone(); Callback::from(move |_| active_tab.set(AddressTab::Shipping))}
                            >
                                { i18n.t("new-sales-invoice-shipping-address") }
                            </button>
                        </div>

                        <div class="tab-content-panel">
                            if *active_tab == AddressTab::Billing {
                                <div class="compact-address-grid">
                                    <div class="full-width-row">
                                        <label>{ i18n.t("new-sales-invoice-select-billing") }</label>
                                        <select onchange={
                                            let state = request.clone();
                                            let partner_addresses = partner_addresses.clone();
                                            Callback::from(move |e: Event| {
                                                let value = e.target_unchecked_into::<HtmlSelectElement>().value();
                                                if let Ok(id) = Uuid::parse_str(&value) {
                                                    if let Some(addr) = (*partner_addresses).iter().find(|a| a.id == id) {
                                                        let mut req = (*state).clone();
                                                        req.billing_address_id = Some(id);
                                                        req.bill_to.address_line1 = Some(addr.address_line1.clone());
                                                        req.bill_to.address_line2 = addr.address_line2.clone();
                                                        req.bill_to.city = Some(addr.city.clone());
                                                        req.bill_to.state_province = addr.state_province.clone();
                                                        req.bill_to.postal_code = addr.postal_code.clone();
                                                        req.bill_to.country = Some(addr.country.clone());
                                                        state.set(req);
                                                    }
                                                }
                                            })
                                        }>
                                            { for (*partner_addresses).iter().map(|a| html! {
                                                <option value={a.id.to_string()} selected={request.billing_address_id == Some(a.id)}>
                                                    { format!("{}{}, {}", a.address_line1, a.address_line2.as_ref().map(|s| format!(", {}", s)).unwrap_or_default(), a.city) }
                                                </option>
                                            }) }
                                        </select>
                                    </div>

                                    <div class="split-row">
                                        <input value={request.bill_to.name.clone().unwrap_or_default()} oninput={on_input(|r, v| r.bill_to.name = Some(v))} placeholder={i18n.t("common-name")} />
                                        <input value={request.bill_to.attention.clone().unwrap_or_default()} oninput={on_input(|r, v| r.bill_to.attention = Some(v))} placeholder={i18n.t("address-attention")} />
                                    </div>

                                    <textarea class="full-width-row" rows="1" value={request.bill_to.address_line1.clone().unwrap_or_default()} oninput={on_input(|r, v| r.bill_to.address_line1 = Some(v))} placeholder={i18n.t("address-line1")} />
                                    <input class="full-width-row" value={request.bill_to.address_line2.clone().unwrap_or_default()} oninput={on_input(|r, v| r.bill_to.address_line2 = Some(v))} placeholder={i18n.t("address-line2")} />

                                    <div class="geo-quad-row">
                                        <input value={request.bill_to.city.clone().unwrap_or_default()} oninput={on_input(|r, v| r.bill_to.city = Some(v))} placeholder={i18n.t("address-city")} />
                                        <input value={request.bill_to.state_province.clone().unwrap_or_default()} oninput={on_input(|r, v| r.bill_to.state_province = Some(v))} placeholder={i18n.t("address-region")} />
                                        <input value={request.bill_to.postal_code.clone().unwrap_or_default()} oninput={on_input(|r, v| r.bill_to.postal_code = Some(v))} placeholder={i18n.t("address-postal-code")} />
                                        <input value={request.bill_to.country.clone().unwrap_or_default()} oninput={on_input(|r, v| r.bill_to.country = Some(v))} placeholder={i18n.t("address-country")} />
                                    </div>
                                </div>
                            } else {
                                <div class="compact-address-grid">
                                    <div class="full-width-row">
                                        <label>{ i18n.t("new-sales-invoice-select-shipping") }</label>
                                        <select onchange={
                                            let state = request.clone();
                                            let partner_addresses = partner_addresses.clone();
                                            Callback::from(move |e: Event| {
                                                let value = e.target_unchecked_into::<HtmlSelectElement>().value();
                                                if let Ok(id) = Uuid::parse_str(&value) {
                                                    if let Some(addr) = (*partner_addresses).iter().find(|a| a.id == id) {
                                                        let mut req = (*state).clone();
                                                        req.shipping_address_id = Some(id);
                                                        req.ship_to.address_line1 = Some(addr.address_line1.clone());
                                                        req.ship_to.address_line2 = addr.address_line2.clone();
                                                        req.ship_to.city = Some(addr.city.clone());
                                                        req.ship_to.state_province = addr.state_province.clone();
                                                        req.ship_to.postal_code = addr.postal_code.clone();
                                                        req.ship_to.country = Some(addr.country.clone());
                                                        state.set(req);
                                                    }
                                                }
                                            })
                                        }>
                                            { for (*partner_addresses).iter().map(|a| html! {
                                                <option value={a.id.to_string()} selected={request.shipping_address_id == Some(a.id)}>
                                                    { format!("{}{}, {}", a.address_line1, a.address_line2.as_ref().map(|s| format!(", {}", s)).unwrap_or_default(), a.city) }
                                                </option>
                                            }) }
                                        </select>
                                    </div>

                                    <div class="split-row">
                                        <input value={request.ship_to.name.clone().unwrap_or_default()} oninput={on_input(|r, v| r.ship_to.name = Some(v))} placeholder={i18n.t("common-name")} />
                                        <input value={request.ship_to.attention.clone().unwrap_or_default()} oninput={on_input(|r, v| r.ship_to.attention = Some(v))} placeholder={i18n.t("address-attention")} />
                                    </div>

                                    <textarea class="full-width-row" rows="1" value={request.ship_to.address_line1.clone().unwrap_or_default()} oninput={on_input(|r, v| r.ship_to.address_line1 = Some(v))} placeholder={i18n.t("address-line1")} />
                                    <input class="full-width-row" value={request.ship_to.address_line2.clone().unwrap_or_default()} oninput={on_input(|r, v| r.ship_to.address_line2 = Some(v))} placeholder={i18n.t("address-line2")} />

                                    <div class="geo-quad-row">
                                        <input value={request.ship_to.city.clone().unwrap_or_default()} oninput={on_input(|r, v| r.ship_to.city = Some(v))} placeholder={i18n.t("address-city")} />
                                        <input value={request.ship_to.state_province.clone().unwrap_or_default()} oninput={on_input(|r, v| r.ship_to.state_province = Some(v))} placeholder={i18n.t("address-region")} />
                                        <input value={request.ship_to.postal_code.clone().unwrap_or_default()} oninput={on_input(|r, v| r.ship_to.postal_code = Some(v))} placeholder={i18n.t("address-postal-code")} />
                                        <input value={request.ship_to.country.clone().unwrap_or_default()} oninput={on_input(|r, v| r.ship_to.country = Some(v))} placeholder={i18n.t("address-country")} />
                                    </div>
                                </div>
                            }
                        </div>
                    </div>

                    <div class="sale__entries">
                        <div class="sale__entry-header">
                            <span class="table__text-col">{i18n.t("common-item")}</span>
                            <span class="table__text-col">{i18n.t("common-description")}</span>
                            <span class="table__value-col">{i18n.t("common-quantity")}</span>
                            <span class="table__value-col">{i18n.t("common-price")}</span>
                            <span class="table__value-col">{i18n.t("common-tax-rate")}</span>
                            <span class="table__value-col">{i18n.t("common-tax")}</span>
                            <span class="table__value-col">{i18n.t("common-total")}</span>
                            <span class="table__col-actions"></span>
                        </div>
                        { for request.lines.iter().map(|item| html! {
                            <SalesInvoiceItemRow
                                key={item.id.to_string()} // Added key prop
                                item={item.clone()}
                                on_change={on_item_change.clone()}
                                on_delete={on_item_delete.clone()}
                            />
                        })}
                    </div>
                    <div class="table-actions">
                        <button type="button" class="button-primary" onclick={add_item}>{ i18n.t("new-sales-invoice-add-line-button") }</button>
                    </div>
                    <div class="sale-footer">
                        if let Some(msg) = &*success {
                            <div class="message message__success">{ msg }</div>
                        }
                        if let Some(e) = &*error {
                            <div class="error">{e}</div>
                        }
                        <button type="submit" class="button-primary">{ i18n.t("new-sales-invoice-save-button") }</button>
                    </div>
                </>
                }
                } else {
                html! {

                    }
                }
            }
            </form>
        </Layout>
    }
}
