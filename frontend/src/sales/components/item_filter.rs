/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use shared_core::sales::models::item::ItemType;
use yew::prelude::*;

use crate::{
    contexts::locale_context::use_locale,
    sales::contexts::item_filter_context::{
        use_item_filter,
        ItemFilterAction,
    },
};

#[function_component(ItemFilter)]
pub fn item_filter() -> Html {
    let filter_ctx = use_item_filter();
    let i18n = use_locale();

    let on_search_change = {
        let filter_ctx = filter_ctx.clone();
        Callback::from(move |e: InputEvent| {
            let value = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            filter_ctx.dispatch(ItemFilterAction::SetSearchTerm(value));
        })
    };

    let on_item_type_change = {
        let filter_ctx = filter_ctx.clone();
        Callback::from(move |e: Event| {
            let value = e
                .target_unchecked_into::<web_sys::HtmlSelectElement>()
                .value();
            let item_type = match value.as_str() {
                "Stocked" => Some(ItemType::Stocked),
                "NonStocked" => Some(ItemType::NonStocked),
                "Service" => Some(ItemType::Service),
                _ => None,
            };
            filter_ctx.dispatch(ItemFilterAction::SetItemType(item_type));
        })
    };

    let on_include_inactive_change = {
        let filter_ctx = filter_ctx.clone();
        Callback::from(move |e: Event| {
            let checked = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .checked();
            filter_ctx.dispatch(ItemFilterAction::SetIncludeInactive(checked));
        })
    };

    let on_limit_change = {
        let filter_ctx = filter_ctx.clone();
        Callback::from(move |e: Event| {
            let value = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            if let Ok(limit) = value.parse::<u32>() {
                filter_ctx.dispatch(ItemFilterAction::SetLimit(limit));
            }
        })
    };

    let on_more_click = {
        let filter_ctx = filter_ctx.clone();
        Callback::from(move |_| {
            filter_ctx.dispatch(ItemFilterAction::IncrementLimit);
        })
    };

    html! {
        <div class="report__options">
            <div class="filter-bar-row" style="display: flex; align-items: center; gap: 0.75rem; margin-bottom: 0.5rem;">
                <input type="text" class="report__advanced-filters" placeholder={i18n.t("item-filter-search-placeholder")} oninput={on_search_change} />
                <select onchange={on_item_type_change}>
                    <option value="">{i18n.t("item-filter-all-types")}</option>
                    <option value="Service">{i18n.t("item-type-service")}</option>
                    <option value="Stocked">{i18n.t("item-type-stocked")}</option>
                    <option value="NonStocked">{i18n.t("item-type-non-stocked")}</option>
                </select>
                <label class="report__advanced-filters">
                    <input type="checkbox" onchange={on_include_inactive_change} />
                    {i18n.t("item-filter-include-inactive")}
                </label>
                <input type="number" class="report__rows-per-page" value={filter_ctx.limit.to_string()} onchange={on_limit_change} />
                <button  class="report__advanced-filters" onclick={on_more_click}>{i18n.t("common-more")}</button>
            </div>
        </div>
    }
}
