/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    core::components::layout::Layout,
    contexts::locale_context::use_locale,
    payables::{
        components::{
            vendor_invoice_filter::VendorInvoiceFilter,
            vendor_invoice_table::VendorInvoiceTable,
        },
        vendor_invoice_filter_context::VendorInvoiceFilterProvider,
    },
    router::Route,
};

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

    html! {
        <Layout>
            <VendorInvoiceFilterProvider>
                <div class="report-header">
                    <h1>{ i18n.t("payables-ledger-title") }</h1>
                    <VendorInvoiceFilter />
                </div>
                <div class="table-actions">
                    <button class="button-primary" onclick={on_add_click}>{ i18n.t("payables-ledger-new-invoice-button") }</button>
                </div>
                <VendorInvoiceTable />
            </VendorInvoiceFilterProvider>
        </Layout>
    }
}
