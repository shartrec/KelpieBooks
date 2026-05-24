/*
 * Copyright (c) 2026. Trevor Campbell and others.
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

use crate::components::aged_trial_balance_matrix::AgedTrialBalanceMatrix;
use crate::components::layout::Layout;
use crate::components::vendor_invoice_filter::VendorInvoiceFilter;
use crate::components::vendor_invoice_table::VendorInvoiceTable;
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
                    <h3>{ "Payables Ledger" }</h3>
                    <VendorInvoiceFilter />
                </div>
                <div class="table-actions">
                    <button class="button-primary" onclick={on_add_click}>{ "+ New Invoice" }</button>
                    <div class="view-toggle">
                        <button class={if *view == View::List { "active" } else { "" }} onclick={set_view.reform(|_| View::List)}>{ "List" }</button>
                        <button class={if *view == View::Aged { "active" } else { "" }} onclick={set_view.reform(|_| View::Aged)}>{ "Aged" }</button>
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
