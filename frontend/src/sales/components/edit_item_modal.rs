/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use rust_decimal::Decimal;
use shared_core::{
    ledger::models::account_category::AccountCategory,
    sales::models::item::{
        Item,
        ItemType,
        UnitOfMeasure,
    },
};
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{
    api::Api,
    contexts::{
        auth_context::use_user_context,
        locale_context::use_locale,
    },
    core::components::currency_input::DecimalInput,
    ledger::util::get_accounts_by_category,
};

#[derive(Properties, PartialEq)]
pub struct EditItemModalProps {
    #[prop_or_default]
    pub item: Option<Item>,
    pub on_close: Callback<()>,
    pub on_submit: Callback<()>,
}

#[function_component(EditItemModal)]
pub fn edit_item_modal(props: &EditItemModalProps) -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();
    let request = use_state(|| props.item.clone().unwrap());
    let uoms = use_state(Vec::new);
    let income_accounts = use_state(Vec::new);
    let error = use_state(|| None::<String>);

    {
        let uoms = uoms.clone();
        let income_accounts = income_accounts.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        let error = error.clone();
        let i18n = i18n.clone();
        use_effect_with((), move |_| {
            let uoms = uoms.clone();
            let income_accounts = income_accounts.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            let error = error.clone();
            let i18n = i18n.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(response) =
                    Api::get("/api/sales/uoms", user_ctx.clone(), navigator.clone()).await
                {
                    if let Ok(data) = response.json::<Vec<UnitOfMeasure>>().await {
                        uoms.set(data);
                    }
                }
                let fetched_accounts = get_accounts_by_category(
                    AccountCategory::Revenue,
                    user_ctx,
                    navigator,
                    &i18n,
                    false,
                )
                .await;
                match fetched_accounts {
                    Ok(postable_accounts) => {
                        income_accounts.set(postable_accounts);
                    }
                    Err(e) => error.set(Some(e)),
                }
            });
            || ()
        });
    }

    let on_input = |field_updater: fn(&mut Item, String)| {
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

    let on_select = |field_updater: fn(&mut Item, Uuid)| {
        let state = request.clone();
        Callback::from(move |e: Event| {
            let mut info = (*state).clone();
            let value = e
                .target_unchecked_into::<web_sys::HtmlSelectElement>()
                .value();
            if let Ok(id) = Uuid::parse_str(&value) {
                field_updater(&mut info, id);
                state.set(info);
            }
        })
    };

    let on_price_change = {
        let state = request.clone();
        Callback::from(move |value: Decimal| {
            let mut info = (*state).clone();
            info.unit_price = value;
            state.set(info);
        })
    };

    let on_cost_change = {
        let state = request.clone();
        Callback::from(move |value: Decimal| {
            let mut info = (*state).clone();
            info.unit_cost = value;
            state.set(info);
        })
    };

    let on_item_type_change = {
        let state = request.clone();
        Callback::from(move |e: Event| {
            let mut info = (*state).clone();
            let value = e
                .target_unchecked_into::<web_sys::HtmlSelectElement>()
                .value();
            info.item_type = match value.as_str() {
                "Stocked" => ItemType::Stocked,
                "NonStocked" => ItemType::NonStocked,
                "Service" => ItemType::Service,
                _ => ItemType::Service,
            };
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
        let on_submit = props.on_submit.clone();
        let request = request.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        let is_edit = props.item.is_some();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let on_submit = on_submit.clone();
            let request = request.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = if is_edit {
                    Api::put(
                        &format!("/api/sales/items/{}", request.id),
                        &*request,
                        user_ctx,
                        navigator,
                    )
                    .await
                } else {
                    Api::post("/api/sales/items", &*request, user_ctx, navigator).await
                };
                if resp.is_ok() {
                    on_submit.emit(());
                }
            });
        })
    };

    let on_cancel = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| {
            on_close.emit(());
        })
    };

    let title = if props.item.is_some() {
        "item-edit-title"
    } else {
        "item-add-title"
    };

    html! {
        <div class="modal-overlay" onclick={on_cancel.clone()}>
            <div class="modal-content" onclick={|e: MouseEvent| e.stop_propagation()}>
                <h2>{ i18n.t(title) }</h2>
                <form onsubmit={on_form_submit} class="modal__form">
                    <label>{i18n.t("item-code-label")}</label>
                    <input type="text" value={request.code.clone()} oninput={on_input(|r, v| r.code = v)} required=true />

                    <label>{i18n.t("item-name-label")}</label>
                    <input type="text" value={request.name.clone()} oninput={on_input(|r, v| r.name = v)} required=true />

                    <label>{i18n.t("item-description-label")}</label>
                    <input type="text" value={request.description.clone().unwrap_or_default()} oninput={on_input(|r, v| r.description = Some(v))} />

                    <label>{i18n.t("item-type-label")}</label>
                    <select onchange={on_item_type_change}>
                        <option value="Service" selected={request.item_type == ItemType::Service}>{"Service"}</option>
                        <option value="Stocked" selected={request.item_type == ItemType::Stocked}>{"Stocked"}</option>
                        <option value="NonStocked" selected={request.item_type == ItemType::NonStocked}>{"Non-Stocked"}</option>
                    </select>

                    <label>{i18n.t("item-uom-label")}</label>
                    <select onchange={on_select(|r, v| r.uom_id = v)}>
                        <option value="" disabled=true selected={request.uom_id.is_nil()}>{i18n.t("item-select-uom")}</option>
                        { for (*uoms).iter().map(|uom| html! {
                            <option value={uom.id.to_string()} selected={request.uom_id == uom.id}>{&uom.name}</option>
                        })}
                    </select>

                    <label>{i18n.t("item-price-label")}</label>
                    <DecimalInput
                        value={request.unit_price}
                        decimal_places = 4
                        on_change={on_price_change}
                    />

                    <label>{i18n.t("item-cost-label")}</label>
                    <DecimalInput
                        value={request.unit_cost}
                        decimal_places = 4
                        on_change={on_cost_change}
                    />

                    <label>{i18n.t("item-income-account-label")}</label>
                    <select onchange={on_select(|r, v| r.income_account_id = v)}>
                        <option value="" disabled=true selected={request.income_account_id.is_nil()}>{i18n.t("item-select-income-account")}</option>
                        { for (*income_accounts).iter().map(|acc| html! {
                            <option value={acc.id.to_string()} selected={request.income_account_id == acc.id}>{&acc.name}</option>
                        })}
                    </select>

                    <label>{i18n.t("item-is-active-label")}</label>
                    <input type="checkbox" checked={request.is_active} onchange={on_is_active_change} />

                    <div class="modal__form__actions">
                        <button type="button" onclick={on_cancel} class="button-secondary">{ i18n.t("common-cancel") }</button>
                        <button type="submit">{ i18n.t("common-save") }</button>
                    </div>
                    if let Some(err) = &*error {
                        <div class="error">{ err }</div>
                    }
                </form>
            </div>
        </div>
    }
}
