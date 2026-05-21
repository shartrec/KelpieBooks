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

use yew::prelude::*;
use shared_core::requests::vendor_invoice::CreateVendorInvoiceRequest;
use shared_core::models::vendor_invoice_item::VendorInvoiceItem;
use uuid::Uuid;
use crate::components::layout::Layout;
use crate::components::vendor_invoice_item_row::VendorInvoiceItemRow;
use crate::api::Api;
use crate::contexts::auth_context::use_user_context;
use yew_router::prelude::use_navigator;
use shared_core::dtos::partner_list_item::PartnerListItem;
use shared_core::dtos::account_with_balance::AccountWithBalance;
use web_sys::HtmlInputElement;
use chrono::{NaiveDate, Local};
use crate::router::Route;

#[function_component(NewVendorInvoicePage)]
pub fn new_vendor_invoice_page() -> Html {
    let user_ctx = use_user_context();
    let navigator = use_navigator().unwrap();
    let request = use_state(|| {
        let today = Local::now().date_naive();
        CreateVendorInvoiceRequest {
            issue_date: today,
            due_date: today,
            items: vec![VendorInvoiceItem {
                id: Uuid::new_v4(),
                vendor_invoice_id: Uuid::nil(),
                account_id: Uuid::nil(),
                description: String::new(),
                net_amount: 0,
                tax_amount: 0,
                total_amount: 0,
            }],
            ..Default::default()
        }
    });
    let vendors = use_state(Vec::new);
    let accounts = use_state(Vec::new);
    let error = use_state(|| None::<String>);

    let fetch_data = {
        let vendors = vendors.clone();
        let accounts = accounts.clone();
        let error = error.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        Callback::from(move |_: ()| {
            let vendors = vendors.clone();
            let accounts = accounts.clone();
            let error = error.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_vendors = Api::get("/api/partners", user_ctx.clone(), navigator.clone()).await;
                match fetched_vendors {
                    Ok(response) if response.ok() => {
                        match response.json::<Vec<PartnerListItem>>().await {
                            Ok(data) => vendors.set(data.into_iter().filter(|p| p.is_vendor).collect()),
                            Err(e) => error.set(Some(format!("Failed to parse vendors: {}", e))),
                        }
                    }
                    Ok(response) => error.set(Some(format!("Failed to fetch vendors: {}", response.status()))),
                    Err(e) => error.set(Some(format!("Network error: {}", e))),
                }

                let fetched_accounts = Api::get("/api/accounts_with_balances", user_ctx, navigator).await;
                match fetched_accounts {
                    Ok(response) if response.ok() => {
                        match response.json::<Vec<AccountWithBalance>>().await {
                            Ok(data) => accounts.set(data),
                            Err(e) => error.set(Some(format!("Failed to parse accounts: {}", e))),
                        }
                    }
                    Ok(response) => error.set(Some(format!("Failed to fetch accounts: {}", response.status()))),
                    Err(e) => error.set(Some(format!("Network error: {}", e))),
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
            let value = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
            info.partner_id = Uuid::parse_str(&value).unwrap_or_default();
            state.set(info);
        })
    };

    let on_input = |field_updater: fn(&mut CreateVendorInvoiceRequest, String)| {
        let state = request.clone();
        Callback::from(move |e: InputEvent| {
            let mut info = (*state).clone();
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            field_updater(&mut info, value);
            state.set(info);
        })
    };

    let on_date_change = |field_updater: fn(&mut CreateVendorInvoiceRequest, NaiveDate)| {
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
        Callback::from(move |item: VendorInvoiceItem| {
            let mut req = (*request).clone();
            if let Some(pos) = req.items.iter().position(|i| i.id == item.id) {
                req.items[pos] = item;
                request.set(req);
            }
        })
    };

    let on_item_delete = {
        let request = request.clone();
        Callback::from(move |id: Uuid| {
            let mut req = (*request).clone();
            req.items.retain(|i| i.id != id);
            request.set(req);
        })
    };

    let add_item = {
        let request = request.clone();
        Callback::from(move |_| {
            let mut req = (*request).clone();
            req.items.push(VendorInvoiceItem {
                id: Uuid::new_v4(),
                vendor_invoice_id: Uuid::nil(),
                account_id: Uuid::nil(),
                description: String::new(),
                net_amount: 0,
                tax_amount: 0,
                total_amount: 0,
            });
            request.set(req);
        })
    };

    let on_submit = {
        let request = request.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        let error = error.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let request = request.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Api::post("/api/vendor-invoices", &*request, user_ctx, navigator.clone()).await;
                match resp {
                    Ok(r) if r.ok() => {
                        navigator.push(&Route::Payables);
                    }
                    Ok(r) => error.set(Some(format!("Failed to create invoice: {}", r.status()))),
                    Err(e) => error.set(Some(format!("Network error: {}", e))),
                }
            });
        })
    };

    html! {
        <Layout>
            <h1>{ "New Vendor Invoice" }</h1>
            <form onsubmit={on_submit} class="voucher__form">
                <div class="data-form">
                    <label>{"Vendor:"}</label>
                    <select onchange={on_partner_change} required=true>
                        <option value="" disabled=true selected=true>{"Select a vendor"}</option>
                        { for (*vendors).iter().map(|vendor| html! {
                            <option value={vendor.id.to_string()}>{&vendor.legal_name}</option>
                        })}
                    </select>

                    <label>{"Invoice Number:"}</label>
                    <input type="text" class="voucher__form__invoice" oninput={on_input(|r, v| r.invoice_number = v)} required=true />

                    <label>{"Invoice Date:"}</label>
                    <input type="date" value={request.issue_date.format("%Y-%m-%d").to_string()} onchange={on_date_change(|r, v| r.issue_date = v)} required=true />

                    <label>{"Due Date:"}</label>
                    <input type="date" value={request.due_date.format("%Y-%m-%d").to_string()} onchange={on_date_change(|r, v| r.due_date = v)} required=true />
                </div>

                <div class="voucher__entries">
                    <div class="voucher__entry-header">
                                <span>{"Description"}</span>
                                <span>{"Account"}</span>
                                <span>{"Net Amount"}</span>
                                <span>{"Tax Amount"}</span>
                                <span>{"Total"}</span>
                                <span></span>
                    </div>
                            { for request.items.iter().map(|item| html! {
                                <VendorInvoiceItemRow
                                    item={item.clone()}
                                    accounts={(*accounts).clone()}
                                    on_change={on_item_change.clone()}
                                    on_delete={on_item_delete.clone()}
                                />
                            })}
                </div>
                <div class="table-actions">
                    <button type="button" class="button-primary" onclick={add_item}>{ "+ Add Line" }</button>
                </div>
                <div class="voucher-footer">
                    if let Some(e) = &*error {
                        <div class="error">{e}</div>
                    }
                    <button type="submit" class="button-primary">{ "Save Invoice" }</button>
                </div>
            </form>
        </Layout>
    }
}
