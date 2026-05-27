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
// Assuming your router's Route enum is in lib.rs or main.rs

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let i18n = use_locale();

    let reports_open = use_state(|| false);
    let tasks_open = use_state(|| false);

    let toggle_reports = {
        let reports_open = reports_open.clone();
        Callback::from(move |_| {
            reports_open.set(!*reports_open);
        })
    };

    let toggle_tasks = {
        let tasks_open = tasks_open.clone();
        Callback::from(move |_| {
            tasks_open.set(!*tasks_open);
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
                    <li><Link<Route> to={Route::Ledger}>{ i18n.t("sidebar-accounts") }</Link<Route>></li>
                    <li><Link<Route> to={Route::Payables}>{ i18n.t("sidebar-payables") }</Link<Route>></li>
                    <li><Link<Route> to={Route::PartnerList}>{ i18n.t("sidebar-partners") }</Link<Route>></li>
                    <li class="sidebar__group">
                        <div class="sidebar__group-header" onclick={toggle_reports}>
                            <span>{ i18n.t("sidebar-reports") }</span>
                            <img
                                src="/images/chevron-right.svg"
                                alt={i18n.t("common-toggle")}
                                class={if *reports_open { "rotated" } else { "" }}
                            />
                        </div>
                        if *reports_open {
                            <ul class="sidebar__sub-nav">
                                <li><Link<Route> to={Route::TrialBalance}>{ i18n.t("sidebar-trial-balance") }</Link<Route>></li>
                                <li><Link<Route> to={Route::ProfitLoss}>{ i18n.t("sidebar-profit-loss") }</Link<Route>></li>
                                <li><Link<Route> to={Route::BalanceSheet}>{ i18n.t("sidebar-balance-sheet") }</Link<Route>></li>
                                <li><Link<Route> to={Route::GeneralLedger}>{ i18n.t("sidebar-general-ledger") }</Link<Route>></li>
                            </ul>
                        }
                    </li>
                    <li class="sidebar__group">
                        <div class="sidebar__group-header" onclick={toggle_tasks}>
                            <span>{ i18n.t("sidebar-tasks") }</span>
                            <img
                                src="/images/chevron-right.svg"
                                alt={i18n.t("common-toggle")}
                                class={if *tasks_open { "rotated" } else { "" }}
                            />
                        </div>
                        if *tasks_open {
                            <ul class="sidebar__sub-nav">
                                <li><Link<Route> to={Route::CloseYear}>{ i18n.t("sidebar-close-year") }</Link<Route>></li>
                                <li><Link<Route> to={Route::PeriodSettings}>{ i18n.t("sidebar-period-settings") }</Link<Route>></li>
                                <li><Link<Route> to={Route::Configuration}>{ i18n.t("sidebar-configuration") }</Link<Route>></li>
                            </ul>
                        }
                    </li>
                </ul>
            </nav>
        </aside>
    }
}
