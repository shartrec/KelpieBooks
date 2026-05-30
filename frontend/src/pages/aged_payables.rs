/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::components::aged_trial_balance_matrix::AgedTrialBalanceMatrix;
use crate::components::layout::Layout;
use crate::contexts::locale_context::use_locale;
use yew::prelude::*;

#[function_component(AgedPayablesPage)]
pub fn aged_payables_page() -> Html {
    let i18n = use_locale();

    html! {
        <Layout>
            <div class="report-header">
                <h3>{ i18n.t("sidebar-aged-payables") }</h3>
            </div>
            <AgedTrialBalanceMatrix />
        </Layout>
    }
}
