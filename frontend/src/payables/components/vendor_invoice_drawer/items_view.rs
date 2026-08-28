/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use chrono::Utc;
use fluent::fluent_args;
use rust_decimal::dec;
use shared_core::{ledger::models::account_category::AccountCategory, payables::{
    dtos::vendor_invoice_dto::VendorInvoiceDto,
    models::vendor_invoice_item::VendorInvoiceItem,
}, AccountId};
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{
    api::Api,
    contexts::{
        auth_context::use_user_context,
        locale_context::use_locale,
    },
    core::components::delete_confirmation_modal::DeleteConfirmationModal,
    ledger::util::get_accounts_by_category,
    payables::components::vendor_invoice_drawer::item_edit_card::ItemEditCard,
};

#[derive(Properties, PartialEq, Clone)]
pub struct ItemsViewProps {
    pub invoice: VendorInvoiceDto,
    pub on_change: Callback<()>,
}

#[function_component(ItemsView)]
pub fn items_view(props: &ItemsViewProps) -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();
    let items = use_state(|| props.invoice.items.clone());
    let accounts = use_state(Vec::new);
    let error = use_state(|| None::<String>);
    let invoice_id = props.invoice.invoice.id;
    let item_to_edit = use_state(|| None::<VendorInvoiceItem>);
    let item_to_delete = use_state(|| None::<VendorInvoiceItem>);

    let fetch_accounts = {
        let accounts = accounts.clone();
        let error = error.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        Callback::from(move |_: ()| {
            let accounts = accounts.clone();
            let error = error.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_accounts = get_accounts_by_category(
                    AccountCategory::Expense,
                    user_ctx,
                    navigator,
                    &i18n,
                    false,
                )
                .await;
                match fetched_accounts {
                    Ok(postable_accounts) => {
                        accounts.set(postable_accounts);
                    }
                    Err(e) => error.set(Some(e)),
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
                account_id: AccountId::default(),
                description: None,
                net_amount: dec!(0.00),
                tax_amount: dec!(0.00),
                total_amount: dec!(0.00),
                created_at: Utc::now(),
            }));
        })
    };

    let on_submit = {
        let items = items.clone();
        let on_change = props.on_change.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        let error = error.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let items = items.clone();
            let on_change = on_change.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Api::put(
                    &format!("/api/vendor-invoices/{}/items", invoice_id),
                    &*items,
                    user_ctx,
                    navigator,
                )
                .await;
                match resp {
                    Ok(r) if r.ok() => {
                        on_change.emit(());
                    }
                    Ok(r) => error.set(Some(i18n.t_args(
                        "items-view-error-update-items",
                        &fluent_args!["status" => r.status()],
                    ))),
                    Err(e) => error.set(Some(i18n.t_args(
                        "common-network-error",
                        &fluent_args!["error" => e.to_string()],
                    ))),
                }
            });
        })
    };

    let i18n = use_locale();

    html! {
        <div class="items-view">
            if let Some(item) = &*item_to_edit {
                <ItemEditCard
                    item={item.clone()}
                    accounts={(*accounts).clone()}
                    on_save={on_save_item}
                    on_cancel={on_cancel_edit}
                />
            }
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
                            .unwrap_or_else(|| i18n.t("items-view-unknown-gl-account"));

                        html! {
                            <div class="card-item-compact">
                                // Top Layout Line: GL Destination Code & Action Buttons
                                <div class="card-item-compact__meta">
                                    // Replace placeholder text with your structural account lookup if available
                                    <span class="card-item-compact__account-badge">
                                        { i18n.t_args("items-view-gl-label", &fluent_args!["account" => account_display]) }
                                    </span>

                                    // Inline micro-actions shifted up to eliminate dedicated footer bars
                                    <div class="card__actions" style="display: flex; gap: 4px;">
                                        <button type="button" class="icon-button" onclick={on_edit}>
                                            <img src="/images/edit.svg" alt={i18n.t("common-edit")} style="width: 13px; height: 13px;" />
                                        </button>
                                        <button type="button" class="icon-button" onclick={on_delete}>
                                            <img src="/images/delete.svg" alt={i18n.t("common-delete")} style="width: 13px; height: 13px;" />
                                        </button>
                                    </div>
                                </div>

                                // Bottom Layout Split Line: Context and Financial Auditing Calculations
                                <div class="card-item-compact__body">
                                    // Left Column
                                    <p class="card-item-compact__desc">{ item.description.as_deref().unwrap_or("") }</p>

                                    // Right Column
                                    <div class="card-item-compact__financials">
                                        <p class="card-item-compact__total">
                                            { i18n.format_currency(item.total_amount) }
                                        </p>
                                        <p class="card-item-compact__sub-breakdown">
                                            { i18n.t_args("items-view-net-tax-breakdown", &fluent_args!["net" => i18n.format_currency(item.net_amount), "tax" => i18n.format_currency(item.tax_amount)]) }
                                        </p>
                                    </div>
                                </div>
                            </div>
                        }
                    })}
                    <div class="table-actions">
                        <button type="button" class="button-primary" onclick={add_item}>{ i18n.t("items-view-add-item-button") }</button>
                    </div>
                    <div class="voucher-footer">
                        if let Some(e) = &*error {
                            <div class="message__error">{e}</div>
                        }
                        <button type="submit" class="button-primary">{ i18n.t("account-modal-save-button") }</button>
                    </div>
                </form>

            if let Some(item) = &*item_to_delete {
                <DeleteConfirmationModal
                    title={i18n.t("items-view-delete-item-title")}
                    message={i18n.t_args("items-view-delete-item-message", &fluent_args!["description" => item.description.clone()])}
                    on_confirm={on_delete_confirm}
                    on_cancel={on_delete_cancel}
                />
            }
        </div>
    }
}
