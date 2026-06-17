/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use yew::prelude::*;
use shared_core::sales::models::tax::TaxCategory;
use crate::core::components::layout::Layout;
use crate::contexts::locale_context::use_locale;
use crate::sales::components::tax_category_list_table::TaxCategoryListTable;
use crate::sales::components::tax_category_drawer::tax_category_drawer::{TaxCategoryDrawer, Tab};

#[function_component(TaxCategoryListPage)]
pub fn tax_category_list_page() -> Html {
    let i18n = use_locale();
    let selected_category = use_state(|| None::<(TaxCategory, Tab)>);

    let on_category_select = {
        let selected_category = selected_category.clone();
        Callback::from(move |(category, tab)| {
            selected_category.set(Some((category, tab)));
        })
    };

    let on_drawer_close = {
        let selected_category = selected_category.clone();
        Callback::from(move |()| {
            selected_category.set(None);
        })
    };

    html! {
        <Layout>
            <div class="tax-category-list-container">
                <header class="tax-category-list-header-flex">
                    <h1>{ i18n.t("tax-category-list-title") }</h1>
                </header>
                <p>{ i18n.t("tax-category-list-description") }</p>
                <TaxCategoryListTable on_category_select={on_category_select} />
            </div>
            if let Some((category, tab)) = &*selected_category {
                <div class="drawer-overlay" onclick={on_drawer_close.reform(|_: MouseEvent| ())}>
                    <div class="drawer" onclick={|e: MouseEvent| e.stop_propagation()}>
                        <TaxCategoryDrawer
                            category={category.clone()}
                            on_change={on_drawer_close.clone()}
                            on_close={on_drawer_close.clone()}
                            initial_tab={*tab}
                        />
                    </div>
                </div>
            }
        </Layout>
    }
}