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

use yew::prelude::*;
use shared_core::models::vendor_invoice::VendorInvoice;
use shared_core::models::vendor_invoice_item::VendorInvoiceItem;
use crate::api::Api;
use crate::contexts::auth_context::use_user_context;
use yew_router::prelude::use_navigator;
use shared_core::dtos::account_with_balance::AccountWithBalance;
use crate::components::vendor_invoice_item_row::VendorInvoiceItemRow;
use uuid::Uuid;

#[derive(Properties, PartialEq, Clone)]
pub struct LinesViewProps {
    pub invoice: VendorInvoice,
    pub on_change: Callback<()>,
}

#[function_component(LinesView)]
pub fn lines_view(props: &LinesViewProps) -> Html {
    let user_ctx = use_user_context();
    let navigator = use_navigator().unwrap();
    let items = use_state(|| props.invoice.items.clone());
    let accounts = use_state(Vec::new);
    let error = use_state(|| None::<String>);
    let invoice_id = props.invoice.id;

    let fetch_accounts = {
        let accounts = accounts.clone();
        let error = error.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        Callback::from(move |_: ()| {
            let accounts = accounts.clone();
            let error = error.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
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
        fetch_accounts.emit(());
        || ()
    });

    let on_item_change = {
        let items = items.clone();
        Callback::from(move |item: VendorInvoiceItem| {
            let mut current_items = (*items).clone();
            if let Some(pos) = current_items.iter().position(|i| i.id == item.id) {
                current_items[pos] = item;
                items.set(current_items);
            }
        })
    };

    let on_item_delete = {
        let items = items.clone();
        Callback::from(move |id: Uuid| {
            let mut current_items = (*items).clone();
            current_items.retain(|i| i.id != id);
            items.set(current_items);
        })
    };

    let add_item = {
        let items = items.clone();
        Callback::from(move |_| {
            let mut current_items = (*items).clone();
            current_items.push(VendorInvoiceItem {
                id: Uuid::new_v4(),
                vendor_invoice_id: invoice_id,
                account_id: Uuid::nil(),
                description: String::new(),
                net_amount: 0,
                tax_amount: 0,
                total_amount: 0,
            });
            items.set(current_items);
        })
    };

    let on_submit = {
        let items = items.clone();
        let on_change = props.on_change.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        let error = error.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let items = items.clone();
            let on_change = on_change.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Api::put(&format!("/api/vendor-invoices/{}/items", invoice_id), &*items, user_ctx, navigator).await;
                match resp {
                    Ok(r) if r.ok() => {
                        on_change.emit(());
                    }
                    Ok(r) => error.set(Some(format!("Failed to update invoice items: {}", r.status()))),
                    Err(e) => error.set(Some(format!("Network error: {}", e))),
                }
            });
        })
    };

    html! {
        <div class="lines-view">
            <form onsubmit={on_submit}>
                <div class="voucher__entries">
                    <div class="voucher__entry-header">
                        <span>{"Description"}</span>
                        <span>{"Account"}</span>
                        <span>{"Net Amount"}</span>
                        <span>{"Tax Amount"}</span>
                        <span>{"Total"}</span>
                        <span></span>
                    </div>
                    { for (*items).iter().map(|item| html! {
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
                    <button type="submit" class="button-primary">{ "Save Changes" }</button>
                </div>
            </form>
        </div>
    }
}
