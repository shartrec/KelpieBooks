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

use crate::components::layout::Layout;
use crate::components::vendor_invoice_table::VendorInvoiceTable;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;

#[function_component(PayablesLedgerPage)]
pub fn payables_ledger_page() -> Html {
    let navigator = use_navigator().unwrap();
    let on_add_click = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::NewVendorInvoice);
        })
    };

    html! {
        <Layout>
            <div class="table-actions">
                <button class="button-primary" onclick={on_add_click}>{ "+ New Invoice" }</button>
            </div>
            <VendorInvoiceTable />
        </Layout>
    }
}
