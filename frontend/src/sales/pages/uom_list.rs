/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use yew::prelude::*;
use crate::core::components::layout::Layout;
use crate::contexts::locale_context::use_locale;
use crate::sales::components::uom_list_table::UomListTable;

#[function_component(UomListPage)]
pub fn uom_list_page() -> Html {
    let i18n = use_locale();

    html! {
        <Layout>
            <div class="uom-list-container">
                <header class="uom-list-header-flex">
                    <h1>{ i18n.t("uom-list-title") }</h1>
                </header>
                <p>{ i18n.t("uom-list-description") }</p>
                <UomListTable />
            </div>
        </Layout>
    }
}
