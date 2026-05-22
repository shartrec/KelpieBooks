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

use crate::api::Api;
use crate::components::vendor_invoice_drawer::VendorInvoiceDrawer;
use crate::contexts::auth_context::use_user_context;
use shared_core::dtos::vendor_invoice_list_item::VendorInvoiceListItem;
use shared_core::models::vendor_invoice::VendorInvoice;
use shared_core::util::format_currency;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[function_component(VendorInvoiceTable)]
pub fn vendor_invoice_table() -> Html {
    let user_ctx = use_user_context();
    let navigator = use_navigator().unwrap();
    let invoices = use_state(Vec::new);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);
    let invoice_to_edit = use_state(|| None::<VendorInvoice>);

    let fetch_invoices = {
        let invoices = invoices.clone();
        let error = error.clone();
        let loading = loading.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        Callback::from(move |_: ()| {
            let invoices = invoices.clone();
            let error = error.clone();
            let loading = loading.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            loading.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_invoices = Api::get("/api/vendor-invoices", user_ctx, navigator).await;
                loading.set(false);
                match fetched_invoices {
                    Ok(response) if response.ok() => {
                        match response.json::<Vec<VendorInvoiceListItem>>().await {
                            Ok(data) => {
                                invoices.set(data);
                            }
                            Err(e) => error.set(Some(format!("Failed to parse invoices: {}", e))),
                        }
                    }
                    Ok(response) => error.set(Some(format!(
                        "Failed to fetch invoices: {}",
                        response.status()
                    ))),
                    Err(e) => error.set(Some(format!("Network error: {}", e))),
                }
            });
        })
    };

    let fetch_invoices_clone = fetch_invoices.clone();
    use_effect_with((), move |()| {
        fetch_invoices_clone.emit(());
        || ()
    });

    let on_edit_click = {
        let invoice_to_edit = invoice_to_edit.clone();
        let error = error.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        Callback::from(move |id: Uuid| {
            let invoice_to_edit = invoice_to_edit.clone();
            let error = error.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Api::get(&format!("/api/vendor-invoices/{}", id), user_ctx, navigator).await;
                match resp {
                    Ok(r) if r.ok() => {
                        match r.json::<VendorInvoice>().await {
                            Ok(invoice) => invoice_to_edit.set(Some(invoice)),
                            Err(e) => error.set(Some(format!("Failed to parse invoice: {}", e))),
                        }
                    }
                    Ok(r) => error.set(Some(format!("Failed to fetch invoice: {}", r.status()))),
                    Err(e) => error.set(Some(format!("Network error: {}", e))),
                }
            });
        })
    };

    let on_drawer_close = {
        let invoice_to_edit = invoice_to_edit.clone();
        Callback::from(move |()| {
            invoice_to_edit.set(None);
        })
    };

    let on_drawer_change = {
        let invoice_id = invoice_to_edit.as_ref().map(|i| i.id);
        let on_edit_click = on_edit_click.clone();
        Callback::from(move |()| {
            if let Some(id) = invoice_id {
                on_edit_click.emit(id);
            }
        })
    };

    if *loading {
        return html! { <p>{ "Loading..." }</p> };
    }
    if let Some(err) = &*error {
        return html! { <div class="error">{ err }</div> };
    }

    html! {
        <>
            if let Some(invoice) = &*invoice_to_edit {
                <VendorInvoiceDrawer
                    invoice={invoice.clone()}
                    on_close={on_drawer_close}
                    on_change={on_drawer_change}
                />
            }
            <table class="table">
                <thead>
                    <tr>
                        <th class="table__text-col">{ "Vendor" }</th>
                        <th class="table__text-col">{ "Invoice #" }</th>
                        <th class="table__value-col">{ "Invoice Date" }</th>
                        <th class="table__value-col">{ "Due Date" }</th>
                        <th class="table__value-col">{ "Net" }</th>
                        <th class="table__value-col">{ "Tax" }</th>
                        <th class="table__value-col">{ "Gross" }</th>
                        <th class="table__value-col">{ "Balance Due" }</th>
                        <th class="table__col-actions">{ "Actions" }</th>
                    </tr>
                </thead>
                <tbody>
                    { for (*invoices).iter().map(|invoice| {
                        let on_edit = {
                            let on_edit_click = on_edit_click.clone();
                            let invoice_id = invoice.id;
                            Callback::from(move |_| {
                                on_edit_click.emit(invoice_id);
                            })
                        };
                        html! {
                            <tr>
                                <td class="table__text-col">{ &invoice.partner_name }</td>
                                <td class="table__text-col">{ &invoice.invoice_number }</td>
                                <td class="table__value-col">{ invoice.issue_date.format("%d %b %Y").to_string() }</td>
                                <td class="table__value-col">{ invoice.due_date.format("%d %b %Y").to_string() }</td>
                                <td class="table__value-col">{ format_currency(&invoice.net_amount) }</td>
                                <td class="table__value-col">{ format_currency(&invoice.tax_amount) }</td>
                                <td class="table__value-col">{ format_currency(&invoice.gross_amount) }</td>
                                <td class="table__value-col">{ format_currency(&invoice.amount_remaining) }</td>
                                <td class="table__col-actions">
                                    <button class="btn-pay-action">
                                        <span class="btn-pay-icon">
                                            <img src="/images/credit-card.svg" alt="" style="width:100%; height:100%;" />
                                        </span>
                                        <span>{ "Pay" }</span>
                                    </button>
                                    <button class="icon-button" onclick={on_edit}>
                                        <img src="/images/view.svg" alt="View" />
                                    </button>
                                    <button class="icon-button">
                                        <img src="/images/delete.svg" alt="Delete" />
                                    </button>
                                </td>
                            </tr>
                        }
                    })}
                </tbody>
            </table>
        </>
    }
}
