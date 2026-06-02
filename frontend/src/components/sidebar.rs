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
    let payables_reports_open = use_state(|| false);
    let partners_open = use_state(|| false);
    let tasks_open = use_state(|| false);
    let admin_open = use_state(|| false);

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

                    // Accounts Group
                    <li class="sidebar__group">
                        <div class="sidebar__group-header" onclick={toggle_state(accounts_open.clone())}>
                            <span>{ i18n.t("sidebar-accounts") }</span>
                            <img src="/images/chevron-right.svg" alt={i18n.t("common-toggle")} class={if *accounts_open { "is-rotated" } else { "" }} />
                        </div>
                        if *accounts_open {
                            <ul class="sidebar__sub-nav" style="--depth: 1;">
                                <li><Link<Route> to={Route::Ledger}>{ i18n.t("coa-title") }</Link<Route>></li>
                                <li class="sidebar__group">
                                    <div class="sidebar__group-header" onclick={toggle_state(accounts_reports_open.clone())}>
                                        <span>{ i18n.t("sidebar-reports") }</span>
                                        <img src="/images/chevron-right.svg" alt={i18n.t("common-toggle")} class={if *accounts_reports_open { "is-rotated" } else { "" }} />
                                    </div>
                                    if *accounts_reports_open {
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
                                <li class="sidebar__group">
                                    <div class="sidebar__group-header" onclick={toggle_state(payables_reports_open.clone())}>
                                        <span>{ i18n.t("sidebar-reports") }</span>
                                        <img src="/images/chevron-right.svg" alt={i18n.t("common-toggle")} class={if *payables_reports_open { "is-rotated" } else { "" }} />
                                    </div>
                                    if *payables_reports_open {
                                        <ul class="sidebar__sub-nav" style="--depth: 2;">
                                            <li><Link<Route> to={Route::AgedPayables}>{ i18n.t("sidebar-aged-payables") }</Link<Route>></li>
                                        </ul>
                                    }
                                </li>
                            </ul>
                        }
                    </li>

                    // Partners Group
                    <li class="sidebar__group">
                        <div class="sidebar__group-header" onclick={toggle_state(partners_open.clone())}>
                            <span>{ i18n.t("sidebar-partners") }</span>
                            <img src="/images/chevron-right.svg" alt={i18n.t("common-toggle")} class={if *partners_open { "is-rotated" } else { "" }} />
                        </div>
                        if *partners_open {
                            <ul class="sidebar__sub-nav" style="--depth: 1;">
                                <li><Link<Route> to={Route::PartnerList}>{ i18n.t("partner-list-title") }</Link<Route>></li>
                            </ul>
                        }
                    </li>

                    // Tasks Group
                    <li class="sidebar__group">
                        <div class="sidebar__group-header" onclick={toggle_state(tasks_open.clone())}>
                            <span>{ i18n.t("sidebar-tasks") }</span>
                            <img src="/images/chevron-right.svg" alt={i18n.t("common-toggle")} class={if *tasks_open { "is-rotated" } else { "" }} />
                        </div>
                        if *tasks_open {
                            <ul class="sidebar__sub-nav" style="--depth: 1;">
                                <li><Link<Route> to={Route::CloseYear}>{ i18n.t("sidebar-close-year") }</Link<Route>></li>
                                <li><Link<Route> to={Route::PeriodSettings}>{ i18n.t("sidebar-period-settings") }</Link<Route>></li>
                                <li><Link<Route> to={Route::Configuration}>{ i18n.t("sidebar-configuration") }</Link<Route>></li>
                            </ul>
                        }
                    </li>

                    // Admin Group
                    <li class="sidebar__group">
                        <div class="sidebar__group-header" onclick={toggle_state(admin_open.clone())}>
                            <span>{ i18n.t("sidebar-admin") }</span>
                            <img src="/images/chevron-right.svg" alt={i18n.t("common-toggle")} class={if *admin_open { "is-rotated" } else { "" }} />
                        </div>
                        if *admin_open {
                            <ul class="sidebar__sub-nav" style="--depth: 1;">
                                <li><Link<Route> to={Route::Users}>{ i18n.t("sidebar-users") }</Link<Route>></li>
                            </ul>
                        }
                    </li>
                </ul>
            </nav>
        </aside>
    }
}
