/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use uuid::Uuid;
use yew_router::Routable;

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
    #[at("/partners")]
    PartnerList,
    #[at("/payables")]
    Payables,
    #[at("/payables/new")]
    NewVendorInvoice,
    #[at("/payables/reports/aged-payables")]
    AgedPayables,
    #[at("/reports/trial-balance")]
    TrialBalance,
    #[at("/reports/profit-loss")]
    ProfitLoss,
    #[at("/reports/balance-sheet")]
    BalanceSheet,
    #[at("/reports/general-ledger")]
    GeneralLedger,
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
    #[at("/users")]
    Users,
    #[at("/")]
    Home,

    // Not really a page, but it's a good example of a page that doesn't have a route
    #[at("/style-guide")]
    StyleGuide,
}
