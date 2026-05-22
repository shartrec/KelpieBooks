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
use crate::components::generic_delete_confirmation_modal::GenericDeleteConfirmationModal;
use crate::components::vendor_invoice_drawer::item_edit_card::ItemEditCard;
use crate::contexts::auth_context::use_user_context;
use shared_core::models::account::Account;
use shared_core::models::vendor_invoice::VendorInvoice;
use shared_core::models::vendor_invoice_item::VendorInvoiceItem;
use shared_core::util::format_currency;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use shared_core::models::account_category::AccountCategory;

#[derive(Properties, PartialEq, Clone)]
pub struct ItemsViewProps {
    pub invoice: VendorInvoice,
    pub on_change: Callback<()>,
}

#[function_component(ItemsView)]
pub fn items_view(props: &ItemsViewProps) -> Html {
    let user_ctx = use_user_context();
    let navigator = use_navigator().unwrap();
    let items = use_state(|| props.invoice.items.clone());
    let accounts = use_state(Vec::new);
    let error = use_state(|| None::<String>);
    let invoice_id = props.invoice.id;
    let item_to_edit = use_state(|| None::<VendorInvoiceItem>);
    let item_to_delete = use_state(|| None::<VendorInvoiceItem>);

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
                let url = format!("/api/accounts_by_category/{}", AccountCategory::Expense.to_string());
                let fetched_accounts = Api::get(&url, user_ctx, navigator).await;
                match fetched_accounts {
                    Ok(response) if response.ok() => {
                        match response.json::<Vec<Account>>().await {
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

    let on_edit_click = {
        let item_to_edit = item_to_edit.clone();
        Callback::from(move |item: VendorInvoiceItem| {
            item_to_edit.set(Some(item));
        })
    };

    let on_save_item = {
        let items = items.clone();
        let item_to_edit = item_to_edit.clone();
        Callback::from(move |item: VendorInvoiceItem| {
            let mut current_items = (*items).clone();
            if let Some(pos) = current_items.iter().position(|i| i.id == item.id) {
                current_items[pos] = item;
            } else {
                current_items.push(item);
            }
            items.set(current_items);
            item_to_edit.set(None);
        })
    };

    let on_cancel_edit = {
        let item_to_edit = item_to_edit.clone();
        Callback::from(move |()| {
            item_to_edit.set(None);
        })
    };

    let on_delete_click = {
        let item_to_delete = item_to_delete.clone();
        Callback::from(move |item: VendorInvoiceItem| {
            item_to_delete.set(Some(item));
        })
    };

    let on_delete_confirm = {
        let items = items.clone();
        let item_to_delete = item_to_delete.clone();
        Callback::from(move |()| {
            if let Some(item_to_delete) = &*item_to_delete {
                let mut current_items = (*items).clone();
                current_items.retain(|i| i.id != item_to_delete.id);
                items.set(current_items);
            }
            item_to_delete.set(None);
        })
    };

    let on_delete_cancel = {
        let item_to_delete = item_to_delete.clone();
        Callback::from(move |()| {
            item_to_delete.set(None);
        })
    };

    let add_item = {
        let item_to_edit = item_to_edit.clone();
        Callback::from(move |_| {
            item_to_edit.set(Some(VendorInvoiceItem {
                id: Uuid::new_v4(),
                vendor_invoice_id: invoice_id,
                account_id: Uuid::nil(),
                description: String::new(),
                net_amount: 0,
                tax_amount: 0,
                total_amount: 0,
            }));
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
        <div class="items-view">
            if let Some(item) = &*item_to_edit {
                <ItemEditCard
                    item={item.clone()}
                    accounts={(*accounts).clone()}
                    on_save={on_save_item}
                    on_cancel={on_cancel_edit}
                />
            } else {
                <form onsubmit={on_submit}>
                    { for (*items).iter().map(|item| {
                        let on_edit = {
                            let on_edit_click = on_edit_click.clone();
                            let item = item.clone();
                            Callback::from(move |_| {
                                on_edit_click.emit(item.clone());
                            })
                        };
                        let on_delete = {
                            let on_delete_click = on_delete_click.clone();
                            let item = item.clone();
                            Callback::from(move |_| {
                                on_delete_click.emit(item.clone());
                            })
                        };
                        let account_display = accounts.iter()
                            .find(|acc| acc.id == item.account_id) // Match the GL ID
                            .map(|acc| format!("{} - {}", acc.code, acc.name)) // Format as "6100 - Software Expenses"
                            .unwrap_or_else(|| "Unknown GL Account".to_string());

                        html! {
                            <div class="card-item-compact">
                                // Top Layout Line: GL Destination Code & Action Buttons
                                <div class="card-item-compact__meta">
                                    // Replace placeholder text with your structural account lookup if available
                                    <span class="card-item-compact__account-badge">
                                        { format!("GL: {}", account_display) }
                                    </span>

                                    // Inline micro-actions shifted up to eliminate dedicated footer bars
                                    <div class="card__actions" style="display: flex; gap: 4px;">
                                        <button type="button" class="icon-button" onclick={on_edit}>
                                            <img src="/images/edit.svg" alt="Edit" style="width: 13px; height: 13px;" />
                                        </button>
                                        <button type="button" class="icon-button" onclick={on_delete}>
                                            <img src="/images/delete.svg" alt="Delete" style="width: 13px; height: 13px;" />
                                        </button>
                                    </div>
                                </div>

                                // Bottom Layout Split Line: Context and Financial Auditing Calculations
                                <div class="card-item-compact__body">
                                    // Left Column
                                    <p class="card-item-compact__desc">{ &item.description }</p>

                                    // Right Column
                                    <div class="card-item-compact__financials">
                                        <p class="card-item-compact__total">
                                            { format_currency(&item.total_amount) }
                                        </p>
                                        <p class="card-item-compact__sub-breakdown">
                                            { format!("Net: {} | Tax: {}", format_currency(&item.net_amount), format_currency(&item.tax_amount)) }
                                        </p>
                                    </div>
                                </div>
                            </div>
                        }
                    })}
                    <div class="table-actions">
                        <button type="button" class="button-primary" onclick={add_item}>{ "+ Add Item" }</button>
                    </div>
                    <div class="voucher-footer">
                        if let Some(e) = &*error {
                            <div class="error">{e}</div>
                        }
                        <button type="submit" class="button-primary">{ "Save Changes" }</button>
                    </div>
                </form>
            }
            if let Some(item) = &*item_to_delete {
                <GenericDeleteConfirmationModal
                    title="Delete Item"
                    message={format!("Are you sure you want to delete the item: {}?", item.description)}
                    on_confirm={on_delete_confirm}
                    on_cancel={on_delete_cancel}
                />
            }
        </div>
    }
}
