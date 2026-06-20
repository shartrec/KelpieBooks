/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use fluent::fluent_args;
use shared_core::sales::dtos::sales_invoice_list_item::SalesInvoiceListItem;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{
    api::Api,
    contexts::{
        auth_context::use_user_context,
        locale_context::use_locale,
    },
    sales::contexts::sales_invoice_filter_context::{
        use_sales_invoice_filter,
        PaymentStatusFilter,
    },
};

#[function_component(SalesInvoiceTable)]
pub fn sales_invoice_table() -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();
    let filter_ctx = use_sales_invoice_filter();
    let invoices = use_state(Vec::new);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);

    let fetch_invoices = {
        let invoices = invoices.clone();
        let error = error.clone();
        let loading = loading.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        let filter_ctx = filter_ctx.clone();
        Callback::from(move |_: ()| {
            let invoices = invoices.clone();
            let error = error.clone();
            let loading = loading.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            let filter_ctx = filter_ctx.clone();
            loading.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                let mut url = format!(
                    "/api/sales-invoices?start_date={}&end_date={}",
                    filter_ctx.start_date, filter_ctx.end_date
                );
                if let Some(partner_id) = filter_ctx.partner_id {
                    url.push_str(&format!("&partner_id={}", partner_id));
                }
                if let Some(min_amount) = filter_ctx.min_amount {
                    url.push_str(&format!("&min_amount={}", min_amount));
                }
                match filter_ctx.status {
                    PaymentStatusFilter::Outstanding => {
                        // For now, treat Outstanding as status=open
                        url.push_str("&status=open");
                    }
                    PaymentStatusFilter::Paid => {
                        url.push_str("&status=paid");
                    }
                    PaymentStatusFilter::All => {}
                }
                let fetched_invoices = Api::get(&url, user_ctx, navigator).await;
                loading.set(false);
                match fetched_invoices {
                    Ok(response) if response.ok() => {
                        match response.json::<Vec<SalesInvoiceListItem>>().await {
                            Ok(data) => {
                                invoices.set(data);
                            }
                            Err(e) => error.set(Some(i18n.t_args(
                                "sales-invoice-table-error-parse-invoices",
                                &fluent_args!["error" => e.to_string()],
                            ))),
                        }
                    }
                    Ok(response) => error.set(Some(i18n.t_args(
                        "sales-invoice-table-error-fetch-invoices",
                        &fluent_args!["status" => response.status()],
                    ))),
                    Err(e) => error.set(Some(i18n.t_args(
                        "common-network-error",
                        &fluent_args!["error" => e.to_string()],
                    ))),
                }
            });
        })
    };

    let fetch_invoices_clone = fetch_invoices.clone();
    use_effect_with(
        (
            filter_ctx.start_date,
            filter_ctx.end_date,
            filter_ctx.partner_id,
            filter_ctx.min_amount,
            filter_ctx.status,
        ),
        move |_| {
            fetch_invoices_clone.emit(());
            || ()
        },
    );

    if *loading {
        return html! { <p>{ i18n.t("common-loading") }</p> };
    }
    if let Some(err) = &*error {
        return html! { <div class="error">{ err }</div> };
    }

    html! {
        <table class="table">
            <thead>
                <tr>
                    <th class="table__text-col">{ i18n.t("common-customer") }</th>
                    <th class="table__text-col">{ i18n.t("sales-invoice-table-invoice-number") }</th>
                    <th class="table__value-col">{ i18n.t("sales-invoice-table-invoice-date") }</th>
                    <th class="table__value-col">{ i18n.t("common-due-date") }</th>
                    <th class="table__value-col">{ i18n.t("common-net") }</th>
                    <th class="table__value-col">{ i18n.t("common-tax") }</th>
                    <th class="table__value-col">{ i18n.t("common-gross") }</th>
                </tr>
            </thead>
            <tbody>
                { for (*invoices).iter().map(|inv| html!{
                    <tr>
                        <td class="table__text-col">{ &inv.partner_name }</td>
                        <td class="table__text-col">{ &inv.invoice_number }</td>
                        <td class="table__value-col">{ i18n.format_date(inv.issue_date) }</td>
                        <td class="table__value-col">{ i18n.format_date(inv.due_date) }</td>
                        <td class="table__value-col">{ i18n.format_currency(inv.net_amount) }</td>
                        <td class="table__value-col">{ i18n.format_currency(inv.tax_amount) }</td>
                        <td class="table__value-col">{ i18n.format_currency(inv.gross_amount) }</td>
                    </tr>
                })}
            </tbody>
        </table>
    }
}
