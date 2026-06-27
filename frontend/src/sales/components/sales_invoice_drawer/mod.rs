/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

pub(crate) mod address_edit_card;
pub(crate) mod addresses_view;
pub(crate) mod details_view;
pub(crate) mod items_view;
pub(crate) mod item_edit_card;
pub(crate) mod payments_view;

use fluent::fluent_args;
use shared_core::{
    partners::models::partner::Partner,
    sales::models::{
        invoice_address::AddressType,
        sales_invoice::SalesInvoice,
    },
};
use yew::prelude::*;

use crate::{
    contexts::locale_context::use_locale,
    sales::components::sales_invoice_drawer::{
        addresses_view::AddressesView,
        details_view::DetailsView,
    },
};
use crate::sales::components::sales_invoice_drawer::items_view::ItemsView;
use crate::sales::components::sales_invoice_drawer::payments_view::PaymentsView;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SalesInvoiceDrawerTab {
    General,
    Addresses,
    Items,
    Payments,
}

#[derive(Properties, PartialEq, Clone)]
pub(crate) struct SalesInvoiceDrawerProps {
    pub(crate) invoice: SalesInvoice,
    pub(crate) partner: Partner,
    pub(crate) on_close: Callback<()>,
    pub(crate) on_change: Callback<()>,
    #[prop_or(SalesInvoiceDrawerTab::Items)]
    pub(crate) initial_tab: SalesInvoiceDrawerTab,
}

#[function_component(SalesInvoiceDrawer)]
pub(crate) fn sales_invoice_drawer(props: &SalesInvoiceDrawerProps) -> Html {
    let active_tab = use_state(|| props.initial_tab);
    let i18n = use_locale();

    let set_tab = {
        let active_tab = active_tab.clone();
        Callback::from(move |tab: SalesInvoiceDrawerTab| {
            active_tab.set(tab);
        })
    };
    let on_close = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| {
            on_close.emit(());
        })
    };

    let total_gross = props.invoice.total_amount;
    let balance_remaining = props.invoice.amount_due;

    // Build the address objects
    let addresses = vec![
        (AddressType::Billing, props.invoice.bill_to.clone()),
        (AddressType::Shipping, props.invoice.ship_to.clone()),
    ];

    html! {
        <div class="drawer-overlay" onclick={on_close.clone()}>
            <div class="drawer" onclick={|e: MouseEvent| e.stop_propagation()}>
                <header class="drawer__header">
                    <h3 class="payment-context-banner__vendor">{ &props.partner.trade_name.clone().unwrap_or(props.partner.legal_name.clone()) }</h3>
                    <button class="btn-close" type="button" onclick={on_close.clone()}>
                        <img src="/images/x.svg" alt={i18n.t("common-close")} />
                    </button>
                </header>

                <div class="payment-context-banner">
                    <div class="payment-context-banner__details">
                        <span>{ i18n.t_args("sales-invoice-drawer-inv-number", &fluent_args!["number" => props.invoice.invoice_number.clone()]) }</span>
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
                        class={classes!("tab-trigger", (*active_tab == SalesInvoiceDrawerTab::General).then_some("tab-trigger--active"))}
                        onclick={set_tab.reform(|_| SalesInvoiceDrawerTab::General)}
                    >
                        { i18n.t("common-general") }
                    </button>
                    <button
                        class={classes!("tab-trigger", (*active_tab == SalesInvoiceDrawerTab::Addresses).then_some("tab-trigger--active"))}
                        onclick={set_tab.reform(|_| SalesInvoiceDrawerTab::Addresses)}
                    >
                        { i18n.t("common-addresses") }
                    </button>
                    <button
                        class={classes!("tab-trigger", (*active_tab == SalesInvoiceDrawerTab::Items).then_some("tab-trigger--active"))}
                        onclick={set_tab.reform(|_| SalesInvoiceDrawerTab::Items)}
                    >
                        { i18n.t("common-items") }
                    </button>
                    <button
                        class={classes!("tab-trigger", (*active_tab == SalesInvoiceDrawerTab::Payments).then_some("tab-trigger--active"))}
                        onclick={set_tab.reform(|_| SalesInvoiceDrawerTab::Payments)}
                    >
                        { i18n.t("common-payments") }
                    </button>
                </div>
                <div class="drawer__content">
                    {
                        match *active_tab {
                            SalesInvoiceDrawerTab::General => html! {
                                <DetailsView
                                    invoice={props.invoice.clone()}
                                    on_change={props.on_change.clone()}
                                />
                            },
                            SalesInvoiceDrawerTab::Addresses => html! {
                                <AddressesView
                                    addresses={addresses}
                                    invoice_id={props.invoice.id.clone()}
                                    on_change={props.on_change.clone()}
                                />
                            },
                            SalesInvoiceDrawerTab::Items => html! {
                                <ItemsView
                                    invoice={props.invoice.clone()}
                                    on_change={props.on_change.clone()}
                                />
                            },
                            SalesInvoiceDrawerTab::Payments => html! {
                                <PaymentsView
                                    invoice={props.invoice.clone()}
                                    on_change={props.on_change.clone()}
                                />
                            },
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
