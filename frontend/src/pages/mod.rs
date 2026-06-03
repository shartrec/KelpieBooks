/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use yew::UseStateHandle;
use gloo_net::http::Response;
use fluent::fluent_args;
use shared_core::dtos::ApiErrorMessage;
use crate::contexts::locale_context::LocaleContext;

pub mod account_ledger;
pub mod aged_payables;
pub mod balance_sheet;
pub mod close_year;
pub mod configuration;
pub mod dashboard;
pub mod general_ledger_report;
pub mod ledger;
pub mod login;
pub mod new_transaction;
pub mod new_vendor_invoice;
pub mod partner_list_page;
pub mod payables_ledger;
pub mod period_settings;
pub mod profile;
pub mod profit_loss;
pub mod register;
pub mod roles;
pub mod style_guide;
pub mod trial_balance;
pub mod users;

fn set_error(error: UseStateHandle<Option<String>>, i18n: LocaleContext, r: Response, msg_key: &str) {

    let error = error.clone();
    let i18n = i18n.clone();
    let status = r.status();
    let msg_key = msg_key.to_string().clone();

    wasm_bindgen_futures::spawn_local(async move {
        // Attempt to parse the structured error body from the backend JSON payload
        if let Ok(error_payload) = r.json::<ApiErrorMessage>().await {
            error.set(Some(i18n.t_args(&msg_key, &fluent_args!["error" => error_payload.error]
            )));
        } else {
            // Fallback: If the body isn't standard JSON, drop back to the HTTP code number
            error.set(Some(i18n.t_args(&msg_key,&fluent_args!["error" => status],
            )));
        }
    });
}