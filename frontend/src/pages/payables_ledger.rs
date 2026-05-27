/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::components::aged_trial_balance_matrix::AgedTrialBalanceMatrix;
use crate::components::layout::Layout;
use crate::components::vendor_invoice_filter::VendorInvoiceFilter;
use crate::components::vendor_invoice_table::VendorInvoiceTable;
use crate::contexts::locale_context::use_locale;
use crate::contexts::vendor_invoice_filter_context::VendorInvoiceFilterProvider;
use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Eq)]
enum View {
    List,
    Aged,
}

#[function_component(PayablesLedgerPage)]
pub fn payables_ledger_page() -> Html {
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();
    let on_add_click = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::NewVendorInvoice);
        })
    };
    let view = use_state(|| View::List);

    let set_view = {
        let view = view.clone();
        Callback::from(move |v: View| {
            view.set(v);
        })
    };

    html! {
        <Layout>
            <VendorInvoiceFilterProvider>
                <div class="report-header">
                    <h3>{ i18n.t("payables-ledger-title") }</h3>
                    <VendorInvoiceFilter />
                </div>
                <div class="table-actions">
                    <button class="button-primary" onclick={on_add_click}>{ i18n.t("payables-ledger-new-invoice-button") }</button>
                    <div class="view-toggle">
                        <button class={if *view == View::List { "active" } else { "" }} onclick={set_view.reform(|_| View::List)}>{ i18n.t("common-list") }</button>
                        <button class={if *view == View::Aged { "active" } else { "" }} onclick={set_view.reform(|_| View::Aged)}>{ i18n.t("common-aged") }</button>
                    </div>
                </div>
                {
                    match *view {
                        View::List => html! { <VendorInvoiceTable /> },
                        View::Aged => html! { <AgedTrialBalanceMatrix /> },
                    }
                }
            </VendorInvoiceFilterProvider>
        </Layout>
    }
}
