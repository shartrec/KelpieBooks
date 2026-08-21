/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

pub mod details_view;
pub mod item_edit_card;
pub mod items_view;
pub mod payments_view;

use fluent::fluent_args;
use shared_core::{
    partners::models::partner::Partner,
    payables::dtos::vendor_invoice_dto::VendorInvoiceDto,
};
use yew::prelude::*;

use crate::{
    contexts::locale_context::use_locale,
    payables::components::vendor_invoice_drawer::{
        details_view::DetailsView,
        items_view::ItemsView,
        payments_view::PaymentsView,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvoiceDrawerTab {
    General,
    Items,
    Payments,
}

#[derive(Properties, PartialEq, Clone)]
pub struct VendorInvoiceDrawerProps {
    pub invoice: VendorInvoiceDto,
    pub partner: Partner,
    pub on_close: Callback<()>,
    pub on_change: Callback<()>,
    #[prop_or(InvoiceDrawerTab::General)]
    pub initial_tab: InvoiceDrawerTab,
}

#[function_component(VendorInvoiceDrawer)]
pub fn vendor_invoice_drawer(props: &VendorInvoiceDrawerProps) -> Html {
    let active_tab = use_state(|| props.initial_tab);
    let i18n = use_locale();

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

    let invoice = &props.invoice.invoice;
    let total_gross = invoice.gross_amount;
    let balance_remaining = invoice.amount_remaining;

    html! {
        <div class="drawer-overlay" onclick={on_close.clone()}>
            <div class="drawer" onclick={|e: MouseEvent| e.stop_propagation()}>
                <header class="drawer__header">
                    // Vendor Identity Context Line
                    <h3 class="payment-context-banner__vendor">{ &props.partner.trade_name.clone().unwrap_or(props.partner.legal_name.clone()) } </h3>
                        <button class="btn-close" type="button" onclick={on_close.clone()}>
                            <img src="/images/x.svg" alt={i18n.t("common-close")} />
                        </button>
                </header>

                <div class="payment-context-banner">
                    // Metadata & Financial Reconciliation Badges
                    <div class="payment-context-banner__details">
                        <span>{ i18n.t_args("vendor-invoice-drawer-inv-number", &fluent_args!["number" => invoice.invoice_number.clone()]) }</span>
                        <span style="color: var(--border-color, #cbd5e1);">{"|"}</span>

                        // Always display the true historical original invoice gross liability
                        <span class="amount-badge amount-badge--gross">
                            { i18n.t_args("vendor-invoice-drawer-gross", &fluent_args!["amount" => i18n.format_currency(total_gross)]) }
                        </span>

                        // Conditionally mount outstanding balances if a partial pay variance exists
                        { if balance_remaining != total_gross {
                            html! {
                                <span class="amount-badge amount-badge--outstanding">
                                    { i18n.t_args("vendor-invoice-drawer-outstanding-balance", &fluent_args!["amount" => i18n.format_currency(balance_remaining)]) }
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
                        { i18n.t("common-general") }
                    </button>
                    <button
                        class={classes!("tab-trigger", (*active_tab == InvoiceDrawerTab::Items).then_some("tab-trigger--active"))}
                        onclick={set_tab.reform(|_| InvoiceDrawerTab::Items)}
                    >
                        { i18n.t("common-items") }
                    </button>
                    <button
                        class={classes!("tab-trigger", (*active_tab == InvoiceDrawerTab::Payments).then_some("tab-trigger--active"))}
                        onclick={set_tab.reform(|_| InvoiceDrawerTab::Payments)}
                    >
                        { i18n.t("common-payments") }
                    </button>
                </div>
                <div class="drawer__content">
                    {
                        match *active_tab {
                            InvoiceDrawerTab::General => html! { <DetailsView invoice={invoice.clone()} on_change={props.on_change.clone()} /> },
                            InvoiceDrawerTab::Items => html! { <ItemsView invoice={props.invoice.clone()} on_change={props.on_change.clone()} /> },
                            InvoiceDrawerTab::Payments => html! { <PaymentsView invoice={invoice.clone()} on_change={props.on_change.clone()} /> },
                        }
                    }
                </div>
                <footer class="drawer__footer">
                    <button class="button-secondary" onclick={on_close.clone()}>{ i18n.t("common-close") }</button>
                </footer>
            </div>
        </div>
    }
}
