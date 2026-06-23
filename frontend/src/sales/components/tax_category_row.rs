/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use shared_core::{
    core::models::auth::SystemPrivilege,
    sales::models::tax::TaxCategory,
};
use yew::prelude::*;

use crate::contexts::{
    auth_context::use_user_context,
    locale_context::use_locale,
};

#[derive(Properties, PartialEq, Clone)]
pub struct TaxCategoryRowProps {
    pub tax_category: TaxCategory,
    pub on_view: Callback<TaxCategory>,
    pub on_delete: Callback<TaxCategory>,
    pub on_select: Callback<TaxCategory>,
}

#[function_component(TaxCategoryRow)]
pub fn tax_category_row(props: &TaxCategoryRowProps) -> Html {
    let i18n = use_locale();
    let user_ctx = use_user_context();

    let on_view = {
        let on_view = props.on_view.clone();
        let tax_category = props.tax_category.clone();
        Callback::from(move |_| {
            on_view.emit(tax_category.clone());
        })
    };

    let on_delete = {
        let on_delete = props.on_delete.clone();
        let tax_category = props.tax_category.clone();
        Callback::from(move |_| {
            on_delete.emit(tax_category.clone());
        })
    };

    html! {
        <tr>
            <td class="table__text-col">{ &props.tax_category.name }</td>
            <td class="table__text-col">{ &props.tax_category.description }</td>
            <td class="table__text-col"><input type="checkbox" checked={props.tax_category.is_active} disabled=true /></td>
            <td class="table__col-actions">
                <div class="actions-wrapper">
                    { if user_ctx.has_privilege(&SystemPrivilege::ManageSales) {
                        html! {
                            <>
                                <button class="icon-button btn-action" onclick={on_view}>
                                    <img src="/images/view.svg" alt={i18n.t("common-view")} />
                                </button>
                                <button class="icon-button btn-action" onclick={on_delete}>
                                    <img src="/images/delete.svg" alt={i18n.t("common-delete")} />
                                </button>
                            </>
                        }
                    } else {
                        html!{}
                    }}
                </div>
            </td>
        </tr>
    }
}
