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
    #[at("/")]
    Home,

    #[at("/register")]
    Register,
    #[at("/login")]
    Login,
    #[at("/forgot-password")]
    ForgotPassword,
    #[at("/reset-password")]
    ResetPassword,
    #[at("/profile")]
    Profile,

    #[at("/users")]
    Users,
    #[at("/roles")]
    Roles,

    #[at("/dashboard")]
    Dashboard,

    #[cfg(feature = "ledger")]
    #[at("/ledger")]
    Ledger,
    #[cfg(feature = "ledger")]
    #[at("/reports/trial-balance")]
    TrialBalance,
    #[cfg(feature = "ledger")]
    #[at("/reports/profit-loss")]
    ProfitLoss,
    #[at("/reports/balance-sheet")]
    #[cfg(feature = "ledger")]
    BalanceSheet,
    #[at("/reports/general-ledger")]
    #[cfg(feature = "ledger")]
    GeneralLedger,
    #[at("/accounts/:id")]
    #[cfg(feature = "ledger")]
    AccountLedger { id: Uuid },
    #[at("/transactions/new")]
    #[cfg(feature = "ledger")]
    NewTransaction,
    #[at("/tasks/close-year")]
    #[cfg(feature = "ledger")]
    CloseYear,
    #[at("/tasks/period-settings")]
    #[cfg(feature = "ledger")]
    PeriodSettings,
    #[at("/tasks/configuration")]
    Configuration,

    #[cfg(feature = "partners")]
    #[at("/partners")]
    PartnerList,

    #[cfg(feature = "payables")]
    #[at("/payables")]
    Payables,
    #[cfg(feature = "payables")]
    #[at("/payables/new")]
    NewVendorInvoice,
    #[cfg(feature = "payables")]
    #[at("/payables/reports/aged-payables")]
    AgedPayables,

    #[cfg(feature = "sales")]
    #[at("/items")]
    ItemList,
    #[cfg(feature = "sales")]
    #[at("/uoms")]
    UomList,
    #[cfg(feature = "sales")]
    #[at("/tax-categories")]
    TaxCategoryList,
    #[cfg(feature = "sales")]
    #[at("/sales/new")]
    NewSalesInvoice,

    // Not really a page, but it's a good example of a page that doesn't have a route
    #[at("/style-guide")]
    StyleGuide,
}