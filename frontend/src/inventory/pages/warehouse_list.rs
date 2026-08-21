/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 * (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use yew::prelude::*;

use crate::{
    contexts::locale_context::use_locale,
    core::components::layout::Layout,
    inventory::components::warehouse_list_table::WarehouseListTable,
};

#[function_component(WarehouseListPage)]
pub fn warehouse_list_page() -> Html {
    let i18n = use_locale();

    html! {
        <Layout>
            <div class="warehouse-list-container">
                <header class="warehouse-list-header-flex">
                    <h1>{ i18n.t("warehouse-list-title") }</h1>
                </header>
                <p>{ i18n.t("warehouse-list-description") }</p>
                <WarehouseListTable />
            </div>
        </Layout>
    }
}
