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
    partners::components::partner_list_table::PartnerListTable,
};

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
