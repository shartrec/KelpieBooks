/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use shared_core::{
    AccountId,
    WarehouseId,
};
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
    AccountLedger { id: AccountId },
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

    #[cfg(any(feature = "sales", feature = "inventory"))]
    #[at("/items")]
    ItemList,
    #[cfg(any(feature = "sales", feature = "inventory"))]
    #[at("/uoms")]
    UomList,
    #[cfg(feature = "sales")]
    #[at("/tax-categories")]
    TaxCategoryList,
    #[cfg(feature = "sales")]
    #[at("/sales/orders")]
    SalesOrders,
    #[cfg(feature = "sales")]
    #[at("/sales/orders/new")]
    NewSalesOrder,
    #[cfg(feature = "sales")]
    #[at("/sales/reports/aged-receivables")]
    AgedReceivables,

    #[cfg(feature = "inventory")]
    #[at("/warehouses")]
    WarehouseList,
    #[cfg(feature = "inventory")]
    #[at("/warehouse-locations/:id")]
    WarehouseLocations { id: WarehouseId },

    // Not really a page, but it's a good example of a page that doesn't have a route
    #[at("/style-guide")]
    StyleGuide,
}
