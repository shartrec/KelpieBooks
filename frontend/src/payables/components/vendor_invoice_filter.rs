/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::NaiveDate;
use gloo_console::info;
use rust_decimal::{
    dec,
    Decimal,
};
use shared_core::partners::dtos::partner_list_item::PartnerListItem;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{
    api::Api,
    contexts::{
        auth_context::use_user_context,
        locale_context::use_locale,
    },
    core::components::currency_input::DecimalInput,
    payables::vendor_invoice_filter_context::{
        use_vendor_invoice_filter,
        PaymentStatusFilter,
        VendorInvoiceFilterAction,
    },
};

#[function_component(VendorInvoiceFilter)]
pub fn vendor_invoice_filter() -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();
    let filter_ctx = use_vendor_invoice_filter();
    let vendors = use_state(Vec::new);

    {
        let vendors = vendors.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        use_effect_with((), move |_| {
            let vendors = vendors.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(response) = Api::get("/api/partners", user_ctx, navigator).await {
                    if let Ok(data) = response.json::<Vec<PartnerListItem>>().await {
                        vendors.set(data);
                    }
                }
            });
            || ()
        });
    }

    let on_start_change = {
        let filter_ctx = filter_ctx.clone();
        Callback::from(move |e: Event| {
            let target: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(new_date) = NaiveDate::parse_from_str(&target.value(), "%Y-%m-%d") {
                filter_ctx.dispatch(VendorInvoiceFilterAction::SetStartDate(new_date));
            }
        })
    };

    let on_end_change = {
        let filter_ctx = filter_ctx.clone();
        Callback::from(move |e: Event| {
            let target: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(new_date) = NaiveDate::parse_from_str(&target.value(), "%Y-%m-%d") {
                filter_ctx.dispatch(VendorInvoiceFilterAction::SetEndDate(new_date));
            }
        })
    };

    let on_vendor_change = {
        let filter_ctx = filter_ctx.clone();
        Callback::from(move |e: Event| {
            let target: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let value = target.value();
            info!("Vendor change:");
            if value.is_empty() {
                filter_ctx.dispatch(VendorInvoiceFilterAction::SetPartnerId(None));
            } else if let Ok(id) = Uuid::parse_str(&value) {
                filter_ctx.dispatch(VendorInvoiceFilterAction::SetPartnerId(Some(id)));
            }
        })
    };

    let on_min_amount_change = {
        let filter_ctx = filter_ctx.clone();
        Callback::from(move |amount: Decimal| {
            if amount == dec!(0.00) {
                filter_ctx.dispatch(VendorInvoiceFilterAction::SetMinAmount(None));
            } else {
                filter_ctx.dispatch(VendorInvoiceFilterAction::SetMinAmount(Some(amount)));
            }
        })
    };

    let set_status = {
        let filter_ctx = filter_ctx.clone();
        Callback::from(move |filter: PaymentStatusFilter| {
            filter_ctx.dispatch(VendorInvoiceFilterAction::SetStatus(filter));
        })
    };

    html! {
        <div class="report__options">
            <div class="report__action-bar">
                <div class="filter-bar-row" style="display: flex; align-items: center; gap: 0.75rem; margin-bottom: 1rem;">
                    <div class="filter-segmented-control">
                        <button
                            type="button"
                            class={classes!("segment-trigger", (filter_ctx.status == PaymentStatusFilter::Outstanding).then_some("segment-trigger--active"))}
                            onclick={let s = set_status.clone(); move |_| s.emit(PaymentStatusFilter::Outstanding)}
                        >
                            { i18n.t("vendor-invoice-filter-outstanding") }
                        </button>
                        <button
                            type="button"
                            class={classes!("segment-trigger", (filter_ctx.status == PaymentStatusFilter::Paid).then_some("segment-trigger--active"))}
                            onclick={let s = set_status.clone(); move |_| s.emit(PaymentStatusFilter::Paid)}
                        >
                            { i18n.t("vendor-invoice-filter-fully-paid") }
                        </button>
                        <button
                            type="button"
                            class={classes!("segment-trigger", (filter_ctx.status == PaymentStatusFilter::All).then_some("segment-trigger--active"))}
                            onclick={let s = set_status.clone(); move |_| s.emit(PaymentStatusFilter::All)}
                        >
                            { i18n.t("vendor-invoice-filter-all-invoices") }
                        </button>
                    </div>
                    <div class="report__date-range-selector">
                        <label>{ i18n.t("vendor-invoice-filter-from-label") }</label>
                        <input type="date" value={filter_ctx.start_date.format("%Y-%m-%d").to_string()} onchange={on_start_change} />
                        <label>{ i18n.t("vendor-invoice-filter-to-label") }</label>
                        <input type="date" value={filter_ctx.end_date.format("%Y-%m-%d").to_string()} onchange={on_end_change} />
                    </div>
                </div>
            </div>
            <div class="report__advanced-filters">
                <div class="report__filter-group">
                    <label>{ i18n.t("vendor-invoice-filter-vendor-label") }</label>
                    <select onchange={on_vendor_change}>
                        <option value="" selected=true>{ i18n.t("vendor-invoice-filter-all-vendors") }</option>
                        { for (*vendors).iter().map(|vendor| html! {
                            <option value={vendor.id.to_string()}>{ &vendor.legal_name }</option>
                        })}
                    </select>
                </div>
                <div class="report__filter-group">
                    <label>{ i18n.t("vendor-invoice-filter-min-amount-label") }</label>
                    <DecimalInput
                        value={filter_ctx.min_amount.unwrap_or(dec!(0.00))}
                        on_change={on_min_amount_change}
                        placeholder={i18n.t("journal-entry-currency-placeholder")}
                    />
                </div>
            </div>
        </div>
    }
}
