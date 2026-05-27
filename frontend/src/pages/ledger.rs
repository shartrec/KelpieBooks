/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::components::chart_of_accounts_table::ChartOfAccountsTable;
use crate::components::layout::Layout;
use crate::contexts::locale_context::use_locale;
use yew::prelude::*;

#[function_component(LedgerPage)]
pub fn ledger_page() -> Html {
    let i18n = use_locale();

    html! {
        <Layout>
            <h1>{ i18n.t("coa-title") }</h1>
            <p>{ i18n.t("coa-description") }</p>
            <ChartOfAccountsTable />
        </Layout>
    }
}
