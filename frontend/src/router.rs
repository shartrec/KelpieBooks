/*
 * Copyright (c) 2026. Trevor Campbell and others.
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

use yew_router::Routable;
use uuid::Uuid;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/register")]
    Register,
    #[at("/login")]
    Login,
    #[at("/dashboard")]
    Dashboard,
    #[at("/profile")]
    Profile,
    #[at("/ledger")]
    Ledger,
    #[at("/reports/trial-balance")]
    TrialBalance,
    #[at("/reports/profit-loss")]
    ProfitLoss,
    #[at("/reports/balance-sheet")]
    BalanceSheet,
    #[at("/accounts/:id")]
    AccountLedger { id: Uuid },
    #[at("/transactions/new")]
    NewTransaction,
    #[at("/tasks/close-year")]
    CloseYear,
    #[at("/tasks/period-settings")]
    PeriodSettings,
    #[at("/tasks/configuration")]
    Configuration,
    #[at("/")]
    Home,

    // Not really a page, but it's a good example of a page that doesn't have a route
    #[at("/style-guide")]
    StyleGuide,
}
