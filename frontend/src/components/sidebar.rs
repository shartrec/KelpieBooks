/*
 * Copyright (c) 2026-2026. Trevor Campbell and others.
 *
 * This file is part of KelpieBooks.
 *
 * KelpieBooks is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License,or
 * (at your option) any later version.
 *
 * KelpieBooks is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with KelpieBooks; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 *
 * Contributors:
 *      Trevor Campbell
 *
 */

use crate::router::Route;
use shared_core::i18n::t;
use yew::prelude::*;
use yew_router::prelude::*;
// Assuming your router's Route enum is in lib.rs or main.rs

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
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
                <img src="/images/kelpiedog_120x120_transparent.png" alt={t("sidebar-logo-alt")} class="sidebar__logo" />
                <h2>{ t("branding-app-name") }</h2>
            </div>
            <nav class="sidebar__nav">
                <ul>
                    <li><Link<Route> to={Route::Dashboard}>{ t("sidebar-dashboard") }</Link<Route>></li>
                    <li><Link<Route> to={Route::Ledger}>{ t("sidebar-accounts") }</Link<Route>></li>
                    <li><Link<Route> to={Route::Payables}>{ t("sidebar-payables") }</Link<Route>></li>
                    <li><Link<Route> to={Route::PartnerList}>{ t("sidebar-partners") }</Link<Route>></li>
                    <li class="sidebar__group">
                        <div class="sidebar__group-header" onclick={toggle_reports}>
                            <span>{ t("sidebar-reports") }</span>
                            <img
                                src="/images/chevron-right.svg"
                                alt={t("common-toggle")}
                                class={if *reports_open { "rotated" } else { "" }}
                            />
                        </div>
                        if *reports_open {
                            <ul class="sidebar__sub-nav">
                                <li><Link<Route> to={Route::TrialBalance}>{ t("sidebar-trial-balance") }</Link<Route>></li>
                                <li><Link<Route> to={Route::ProfitLoss}>{ t("sidebar-profit-loss") }</Link<Route>></li>
                                <li><Link<Route> to={Route::BalanceSheet}>{ t("sidebar-balance-sheet") }</Link<Route>></li>
                                <li><Link<Route> to={Route::GeneralLedger}>{ t("sidebar-general-ledger") }</Link<Route>></li>
                            </ul>
                        }
                    </li>
                    <li class="sidebar__group">
                        <div class="sidebar__group-header" onclick={toggle_tasks}>
                            <span>{ t("sidebar-tasks") }</span>
                            <img
                                src="/images/chevron-right.svg"
                                alt={t("common-toggle")}
                                class={if *tasks_open { "rotated" } else { "" }}
                            />
                        </div>
                        if *tasks_open {
                            <ul class="sidebar__sub-nav">
                                <li><Link<Route> to={Route::CloseYear}>{ t("sidebar-close-year") }</Link<Route>></li>
                                <li><Link<Route> to={Route::PeriodSettings}>{ t("sidebar-period-settings") }</Link<Route>></li>
                                <li><Link<Route> to={Route::Configuration}>{ t("sidebar-configuration") }</Link<Route>></li>
                            </ul>
                        }
                    </li>
                </ul>
            </nav>
        </aside>
    }
}
