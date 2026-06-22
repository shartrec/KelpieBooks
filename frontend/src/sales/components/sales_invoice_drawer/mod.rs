/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

pub mod details_view;

use fluent::fluent_args;
use shared_core::{
    partners::models::partner::Partner,
    sales::models::sales_invoice::SalesInvoice,
};
use yew::prelude::*;

use crate::contexts::locale_context::use_locale;
use crate::sales::components::sales_invoice_drawer::details_view::DetailsView;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SalesInvoiceDrawerTab {
    General,
    Addresses,
    Items,
    Payments,
}

#[derive(Properties, PartialEq, Clone)]
pub struct SalesInvoiceDrawerProps {
    pub invoice: SalesInvoice,
    pub partner: Partner,
    pub on_close: Callback<()>,
    pub on_change: Callback<()>,
    #[prop_or(SalesInvoiceDrawerTab::General)]
    pub initial_tab: SalesInvoiceDrawerTab,
}

#[function_component(SalesInvoiceDrawer)]
pub fn sales_invoice_drawer(props: &SalesInvoiceDrawerProps) -> Html {
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
    // Todo Fix balance remaining
    let balance_remaining = props.invoice.subtotal;

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
                            SalesInvoiceDrawerTab::General => html! { <DetailsView invoice={props.invoice.clone()} on_change={props.on_change.clone()} /> },
                            SalesInvoiceDrawerTab::Addresses => html! { <p>{"Addresses go here"} </p> },
                            SalesInvoiceDrawerTab::Items => html! { <p>{"Items go here"} </p> },
                            SalesInvoiceDrawerTab::Payments => html! { <p>{"Payments go here"} </p> },
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
