/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use fluent::fluent_args;
use shared_core::sales::models::sales_order::SalesOrder;
use yew::prelude::*;

use crate::contexts::locale_context::use_locale;

#[derive(Properties, PartialEq, Clone)]
pub struct LinesViewProps {
    pub order: SalesOrder,
}

#[function_component(LinesView)]
pub fn lines_view(props: &LinesViewProps) -> Html {
    let i18n = use_locale();
    let order = &props.order;

    html! {
        <div class="items-view">
            { for order.lines.iter().map(|line| {
                let gross = line.net_amount + line.tax_amount;

                let avail_html = match line.quantity_available {
                    Some(qty) if qty < line.quantity => html! {
                        <span class="status-badge status-badge--warning">
                            { i18n.t_args("sales-order-item-insufficient-stock", &fluent_args!["qty" => i18n.format_decimal(qty)]) }
                        </span>
                    },
                    Some(qty) => html! {
                        <span class="status-badge status-badge--confirmed">
                            { i18n.t_args("sales-order-item-available", &fluent_args!["qty" => i18n.format_decimal(qty)]) }
                        </span>
                    },
                    None => html! { <span class="status-badge status-badge--neutral">{ "—" }</span> },
                };

                html! {
                    <div class="card-item-compact">
                        // Meta row: qty × name on the left, availability badge on the right
                        <div class="card-item-compact__meta">
                            <span class="card-item-compact__account-badge">
                                { format!("{} \u{00d7} {} ({})",
                                    i18n.format_decimal(line.quantity),
                                    line.name,
                                    line.code) }
                            </span>
                            { avail_html }
                        </div>

                        // Body row: description on the left, financials on the right
                        <div class="card-item-compact__body">
                            <p class="card-item-compact__desc">
                                { if line.description.is_empty() { &line.name } else { &line.description } }
                            </p>
                            <div class="card-item-compact__financials">
                                <p class="card-item-compact__total">
                                    { i18n.format_currency(gross) }
                                </p>
                                <p class="card-item-compact__sub-breakdown">
                                    { i18n.t_args("items-view-net-tax-breakdown",
                                        &fluent_args![
                                            "net" => i18n.format_currency(line.net_amount),
                                            "tax" => i18n.format_currency(line.tax_amount)
                                        ]) }
                                </p>
                            </div>
                        </div>
                    </div>
                }
            }) }

            <div class="voucher-footer">
                <span class="amount-badge amount-badge--gross">
                    { i18n.t_args("vendor-invoice-drawer-gross",
                        &fluent_args!["amount" => i18n.format_currency(order.total_amount)]) }
                </span>
            </div>
        </div>
    }
}
