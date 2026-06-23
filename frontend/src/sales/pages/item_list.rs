/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use yew::prelude::*;

use crate::{
    contexts::locale_context::use_locale,
    core::components::layout::Layout,
    sales::{
        components::{
            item_filter::ItemFilter,
            item_list_table::ItemListTable,
        },
        contexts::item_filter_context::ItemFilterProvider,
    },
};

#[function_component(ItemListPage)]
pub fn item_list_page() -> Html {
    let i18n = use_locale();

    html! {
        <Layout>
            <ItemFilterProvider>
                <div class="report-header">
                    <div>
                    <h1>{ i18n.t("item-list-title") }</h1>
                    { i18n.t("item-list-description") }
                    </div>
                    <ItemFilter />
                </div>
                <ItemListTable />
            </ItemFilterProvider>
        </Layout>
    }
}
