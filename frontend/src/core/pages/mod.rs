/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use fluent::fluent_args;
use gloo_net::http::Response;
use shared_core::core::dtos::ApiErrorMessage;
use yew::UseStateHandle;

use crate::contexts::locale_context::LocaleContext;

pub mod configuration;
pub mod dashboard;
pub mod login;
pub mod forgot_password;
pub mod reset_password;
pub mod profile;
pub mod register;
pub mod roles;
pub mod style_guide;
pub mod users;

fn set_error(
    error: UseStateHandle<Option<String>>,
    i18n: LocaleContext,
    r: Response,
    msg_key: &str,
) {
    let error = error.clone();
    let i18n = i18n.clone();
    let status = r.status();
    let msg_key = msg_key.to_string().clone();

    wasm_bindgen_futures::spawn_local(async move {
        // Attempt to parse the structured error body from the backend JSON payload
        if let Ok(error_payload) = r.json::<ApiErrorMessage>().await {
            error.set(Some(
                i18n.t_args(&msg_key, &fluent_args!["error" => error_payload.error]),
            ));
        } else {
            // Fallback: If the body isn't standard JSON, drop back to the HTTP code number
            error.set(Some(
                i18n.t_args(&msg_key, &fluent_args!["error" => status]),
            ));
        }
    });
}
