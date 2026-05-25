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

pub mod details_view;
pub mod items_view;
pub mod payments_view;
pub mod item_edit_card;

use crate::components::vendor_invoice_drawer::details_view::DetailsView;
use crate::components::vendor_invoice_drawer::items_view::ItemsView;
use crate::components::vendor_invoice_drawer::payments_view::PaymentsView;
use crate::contexts::auth_context::use_user_context;
use fluent::fluent_args;
use shared_core::i18n::{t, t_args};
use shared_core::models::partner::Partner;
use shared_core::models::vendor_invoice::VendorInvoice;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use shared_core::util::format_currency;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvoiceDrawerTab {
    General,
    Items,
    Payments,
}

#[derive(Properties, PartialEq, Clone)]
pub struct VendorInvoiceDrawerProps {
    pub invoice: VendorInvoice,
    pub partner: Partner,
    pub on_close: Callback<()>,
    pub on_change: Callback<()>,
    #[prop_or(InvoiceDrawerTab::General)]
    pub initial_tab: InvoiceDrawerTab,
}

#[function_component(VendorInvoiceDrawer)]
pub fn vendor_invoice_drawer(props: &VendorInvoiceDrawerProps) -> Html {
    let active_tab = use_state(|| props.initial_tab);
    let user_ctx = use_user_context();
    let navigator = use_navigator().unwrap();


    let set_tab = {
        let active_tab = active_tab.clone();
        Callback::from(move |tab: InvoiceDrawerTab| {
            active_tab.set(tab);
        })
    };
    let on_close = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| {
            on_close.emit(());
        })
    };

    let total_gross = props.invoice.gross_amount;
    let balance_remaining = props.invoice.amount_remaining;
    html! {
        <div class="drawer-overlay" onclick={on_close.clone()}>
            <div class="drawer" onclick={|e: MouseEvent| e.stop_propagation()}>
                <header class="drawer__header">
                    // Vendor Identity Context Line
                    <h3 class="payment-context-banner__vendor">{ &props.partner.trade_name.clone().unwrap_or(props.partner.legal_name.clone()) } </h3>
                        <button class="btn-close" type="button" onclick={on_close.clone()}>
                            <img src="/images/x.svg" alt={t("common-close")} />
                        </button>
                </header>

                <div class="payment-context-banner">
                    // Metadata & Financial Reconciliation Badges
                    <div class="payment-context-banner__details">
                        <span>{ t_args("vendor-invoice-drawer-inv-number", &fluent_args!["number" => props.invoice.invoice_number.clone()]) }</span>
                        <span style="color: var(--border-color, #cbd5e1);">{"|"}</span>

                        // Always display the true historical original invoice gross liability
                        <span class="amount-badge amount-badge--gross">
                            { t_args("vendor-invoice-drawer-gross", &fluent_args!["amount" => format_currency(&total_gross)]) }
                        </span>

                        // Conditionally mount outstanding balances if a partial pay variance exists
                        { if balance_remaining != total_gross {
                            html! {
                                <span class="amount-badge amount-badge--outstanding">
                                    { t_args("vendor-invoice-drawer-outstanding-balance", &fluent_args!["amount" => format_currency(&balance_remaining)]) }
                                </span>
                            }
                        } else {
                            html! {}
                        }}
                    </div>
                </div>

                <div class="drawer__tabs">
                    <button
                        class={classes!("tab-trigger", (*active_tab == InvoiceDrawerTab::General).then_some("tab-trigger--active"))}
                        onclick={set_tab.reform(|_| InvoiceDrawerTab::General)}
                    >
                        { t("common-general") }
                    </button>
                    <button
                        class={classes!("tab-trigger", (*active_tab == InvoiceDrawerTab::Items).then_some("tab-trigger--active"))}
                        onclick={set_tab.reform(|_| InvoiceDrawerTab::Items)}
                    >
                        { t("common-items") }
                    </button>
                    <button
                        class={classes!("tab-trigger", (*active_tab == InvoiceDrawerTab::Payments).then_some("tab-trigger--active"))}
                        onclick={set_tab.reform(|_| InvoiceDrawerTab::Payments)}
                    >
                        { t("common-payments") }
                    </button>
                </div>
                <div class="drawer__content">
                    {
                        match *active_tab {
                            InvoiceDrawerTab::General => html! { <DetailsView invoice={props.invoice.clone()} on_change={props.on_change.clone()} /> },
                            InvoiceDrawerTab::Items => html! { <ItemsView invoice={props.invoice.clone()} on_change={props.on_change.clone()} /> },
                            InvoiceDrawerTab::Payments => html! { <PaymentsView invoice={props.invoice.clone()} on_change={props.on_change.clone()} /> },
                        }
                    }
                </div>
                <footer class="drawer__footer">
                    <button class="button-secondary" onclick={on_close.clone()}>{ t("common-close") }</button>
                </footer>
            </div>
        </div>
    }
}
