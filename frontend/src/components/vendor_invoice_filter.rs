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
use crate::components::currency_input::CurrencyInput;
use crate::contexts::auth_context::use_user_context;
use crate::contexts::vendor_invoice_filter_context::{use_vendor_invoice_filter, PaymentStatusFilter, VendorInvoiceFilterAction};
use chrono::NaiveDate;
use gloo_console::info;
use shared_core::dtos::partner_list_item::PartnerListItem;
use shared_core::i18n::t;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[function_component(VendorInvoiceFilter)]
pub fn vendor_invoice_filter() -> Html {
    let user_ctx = use_user_context();
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
        Callback::from(move |cents: i64| {
            if cents == 0 {
                filter_ctx.dispatch(VendorInvoiceFilterAction::SetMinAmount(None));
            } else {
                filter_ctx.dispatch(VendorInvoiceFilterAction::SetMinAmount(Some(cents)));
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
                            { t("vendor-invoice-filter-outstanding") }
                        </button>
                        <button
                            type="button"
                            class={classes!("segment-trigger", (filter_ctx.status == PaymentStatusFilter::Paid).then_some("segment-trigger--active"))}
                            onclick={let s = set_status.clone(); move |_| s.emit(PaymentStatusFilter::Paid)}
                        >
                            { t("vendor-invoice-filter-fully-paid") }
                        </button>
                        <button
                            type="button"
                            class={classes!("segment-trigger", (filter_ctx.status == PaymentStatusFilter::All).then_some("segment-trigger--active"))}
                            onclick={let s = set_status.clone(); move |_| s.emit(PaymentStatusFilter::All)}
                        >
                            { t("vendor-invoice-filter-all-invoices") }
                        </button>
                    </div>
                    <div class="report__date-range-selector">
                        <label>{ t("vendor-invoice-filter-from-label") }</label>
                        <input type="date" value={filter_ctx.start_date.to_string()} onchange={on_start_change} />
                        <label>{ t("vendor-invoice-filter-to-label") }</label>
                        <input type="date" value={filter_ctx.end_date.to_string()} onchange={on_end_change} />
                    </div>
                </div>
            </div>
            <div class="report__advanced-filters">
                <div class="report__filter-group">
                    <label>{ t("vendor-invoice-filter-vendor-label") }</label>
                    <select onchange={on_vendor_change}>
                        <option value="">{ t("vendor-invoice-filter-all-vendors") }</option>
                        { for (*vendors).iter().map(|vendor| html! {
                            <option value={vendor.id.to_string()}>{ &vendor.legal_name }</option>
                        })}
                    </select>
                </div>
                <div class="report__filter-group">
                    <label>{ t("vendor-invoice-filter-min-amount-label") }</label>
                    <CurrencyInput
                        value={filter_ctx.min_amount.unwrap_or(0)}
                        on_change={on_min_amount_change}
                        placeholder={t("journal-entry-currency-placeholder")}
                    />
                </div>
            </div>
        </div>
    }
}
