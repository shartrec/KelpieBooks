/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::contexts::locale_context::use_locale;
use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let i18n = use_locale();

    let accounts_open = use_state(|| false);
    let accounts_reports_open = use_state(|| false);
    let payables_open = use_state(|| false);
    let partners_open = use_state(|| false);
    let tasks_open = use_state(|| false);

    let toggle_state = |state: UseStateHandle<bool>| {
        Callback::from(move |_| {
            state.set(!*state);
        })
    };

    html! {
        <aside class="sidebar">
            <div class="sidebar__header">
                <img src="/images/kelpiedog_120x120_transparent.png" alt={i18n.t("sidebar-logo-alt")} class="sidebar__logo" />
                <h2>{ i18n.t("branding-app-name") }</h2>
            </div>
            <nav class="sidebar__nav">
                <ul>
                    <li><Link<Route> to={Route::Dashboard}>{ i18n.t("sidebar-dashboard") }</Link<Route>></li>

                    // Accounts Group (Level 0)
                    <li class="sidebar__group">
                        <div class="sidebar__group-header" onclick={toggle_state(accounts_open.clone())}>
                            <span>{ i18n.t("sidebar-accounts") }</span>
                            <img src="/images/chevron-right.svg" alt={i18n.t("common-toggle")} class={if *accounts_open { "is-rotated" } else { "" }} />
                        </div>
                        if *accounts_open {
                            // Level 1 Sub-nav
                            <ul class="sidebar__sub-nav" style="--depth: 1;">
                                <li><Link<Route> to={Route::Ledger}>{ i18n.t("coa-title") }</Link<Route>></li>

                                <li class="sidebar__group">
                                    <div class="sidebar__group-header" onclick={toggle_state(accounts_reports_open.clone())}>
                                        <span>{ i18n.t("sidebar-reports") }</span>
                                        <img src="/images/chevron-right.svg" alt={i18n.t("common-toggle")} class={if *accounts_reports_open { "is-rotated" } else { "" }} />
                                    </div>
                                    if *accounts_reports_open {
                                        // Level 2 Sub-nav -> This evaluates to 15px + (2 * 20px) = 55px padding-left!
                                        <ul class="sidebar__sub-nav" style="--depth: 2;">
                                            <li><Link<Route> to={Route::TrialBalance}>{ i18n.t("sidebar-trial-balance") }</Link<Route>></li>
                                            <li><Link<Route> to={Route::ProfitLoss}>{ i18n.t("sidebar-profit-loss") }</Link<Route>></li>
                                            <li><Link<Route> to={Route::BalanceSheet}>{ i18n.t("sidebar-balance-sheet") }</Link<Route>></li>
                                            <li><Link<Route> to={Route::GeneralLedger}>{ i18n.t("sidebar-general-ledger") }</Link<Route>></li>
                                        </ul>
                                    }
                                </li>
                            </ul>
                        }
                    </li>

                    // Payables Group
                    <li class="sidebar__group">
                        <div class="sidebar__group-header" onclick={toggle_state(payables_open.clone())}>
                            <span>{ i18n.t("sidebar-payables") }</span>
                            <img src="/images/chevron-right.svg" alt={i18n.t("common-toggle")} class={if *payables_open { "is-rotated" } else { "" }} />
                        </div>
                        if *payables_open {
                            <ul class="sidebar__sub-nav" style="--depth: 1;">
                                <li><Link<Route> to={Route::Payables}>{ i18n.t("payables-ledger-title") }</Link<Route>></li>
                            </ul>
                        }
                    </li>

                    // ... Repeat style="--depth: 1;" on remaining primary dropdown ul elements!
                </ul>
            </nav>
        </aside>
    }
}
