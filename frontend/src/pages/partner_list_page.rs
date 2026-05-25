/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use crate::components::layout::Layout;
use crate::components::partner_list_table::PartnerListTable;
use shared_core::i18n::t;
use yew::prelude::*;

#[function_component(PartnerListPage)]
pub fn partner_list_page() -> Html {
    html! {
        <Layout>
            <div class="partner-list-container">
                <header class="partner-list-header-flex">
                    <h1>{ t("partner-list-title") }</h1>
                </header>
                <p>{ t("partner-list-description") }</p>
                <PartnerListTable />
            </div>
        </Layout>
    }
}
