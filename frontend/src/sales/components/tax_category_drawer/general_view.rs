/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use fluent::fluent_args;
use shared_core::sales::models::tax::TaxCategory;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{
    api::Api,
    contexts::{
        auth_context::use_user_context,
        locale_context::use_locale,
    },
};

#[derive(Properties, PartialEq)]
pub struct GeneralViewProps {
    pub tax_category: TaxCategory,
    pub on_change: Callback<()>,
}

#[function_component(GeneralView)]
pub fn general_view(props: &GeneralViewProps) -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();
    let request = use_state(|| props.tax_category.clone());
    let error = use_state(|| None::<String>);
    let show_saved = use_state(|| false);

    let on_input = |field_updater: fn(&mut TaxCategory, String)| {
        let state = request.clone();
        Callback::from(move |e: InputEvent| {
            let mut info = (*state).clone();
            let value = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            field_updater(&mut info, value);
            state.set(info);
        })
    };

    let on_is_active_change = {
        let state = request.clone();
        Callback::from(move |e: Event| {
            let mut info = (*state).clone();
            info.is_active = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .checked();
            state.set(info);
        })
    };

    let on_form_submit = {
        let on_change = props.on_change.clone();
        let request = request.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        let show_saved = show_saved.clone();
        let i18n = i18n.clone();
        let error = error.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let on_change = on_change.clone();
            let request = request.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            let show_saved = show_saved.clone();
            let i18n = i18n.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Api::put(
                    &format!("/api/sales/tax-categories/{}", request.id),
                    &*request,
                    user_ctx,
                    navigator,
                )
                .await;
                match resp {
                    Ok(r) if r.ok() => {
                        on_change.emit(());
                        show_saved.set(true);
                    }
                    Ok(r) => error.set(Some(i18n.t_args(
                        "tax-rate-drawer-error-update-rates",
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

    html! {
        <form onsubmit={on_form_submit} class="modal__form">
            <label>{i18n.t("tax-category-name-label")}</label>
            <input type="text" value={request.name.clone()} oninput={on_input(|r, v| r.name = v)} required=true />

            <label>{i18n.t("common-description")}</label>
            <input type="text" value={request.description.clone()} oninput={on_input(|r, v| r.description = Some(v))} />

            <label>{i18n.t("tax-category-is-active-label")}</label>
            <input type="checkbox" checked={request.is_active} onchange={on_is_active_change} />

            <div class="table-actions">
                <button type="submit" class="button-primary">{ i18n.t("common-save") }</button>
            </div>
             if *show_saved {
                <span class="fade-out message__success" style="margin-left: 1rem;">{ i18n.t("common-saved") }</span>
            }
        </form>
    }
}
