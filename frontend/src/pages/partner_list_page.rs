/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use crate::components::layout::Layout;
use crate::components::partner_list_table::PartnerListTable;
use crate::contexts::locale_context::use_locale;
use yew::prelude::*;

#[function_component(PartnerListPage)]
pub fn partner_list_page() -> Html {
    let i18n = use_locale();

    html! {
        <Layout>
            <div class="partner-list-container">
                <header class="partner-list-header-flex">
                    <h1>{ i18n.t("partner-list-title") }</h1>
                </header>
                <p>{ i18n.t("partner-list-description") }</p>
                <PartnerListTable />
            </div>
        </Layout>
    }
}
