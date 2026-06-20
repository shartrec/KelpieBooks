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
    router::Route,
    sales::{
        components::{
            sales_invoice_filter::SalesInvoiceFilter,
            sales_invoice_table::SalesInvoiceTable,
        },
        contexts::sales_invoice_filter_context::SalesInvoiceFilterProvider,
    },
};

#[function_component(SalesLedgerPage)]
pub fn sales_ledger_page() -> Html {
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();
    let on_add_click = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::NewSalesInvoice);
        })
    };

    html! {
        <Layout>
            <SalesInvoiceFilterProvider>
                <div class="report-header">
                    <h1>{ i18n.t("sales-ledger-title") }</h1>
                    <SalesInvoiceFilter />
                </div>
                <div class="table-actions">
                    <button class="button-primary" onclick={on_add_click}>{ i18n.t("sales-ledger-new-invoice-button") }</button>
                </div>
                <SalesInvoiceTable />
            </SalesInvoiceFilterProvider>
        </Layout>
    }
}
