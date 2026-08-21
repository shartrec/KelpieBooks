/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

// Import pages conditionally based on features
#[cfg(feature = "ledger")]
use frontend::core::pages::configuration::ConfigurationPage;
#[cfg(feature = "ledger")]
use frontend::ledger::pages::account_ledger::AccountLedgerPage;
#[cfg(feature = "ledger")]
use frontend::ledger::pages::balance_sheet::BalanceSheetPage;
#[cfg(feature = "ledger")]
use frontend::ledger::pages::close_year::CloseYearPage;
#[cfg(feature = "ledger")]
use frontend::ledger::pages::general_ledger_report::GeneralLedgerReportPage;
#[cfg(feature = "ledger")]
use frontend::ledger::pages::ledger::LedgerPage;
#[cfg(feature = "ledger")]
use frontend::ledger::pages::new_transaction::NewTransactionPage;
#[cfg(feature = "ledger")]
use frontend::ledger::pages::period_settings::PeriodSettings;
#[cfg(feature = "ledger")]
use frontend::ledger::pages::profit_loss::ProfitLossPage;
#[cfg(feature = "ledger")]
use frontend::ledger::pages::trial_balance::TrialBalancePage;
#[cfg(feature = "partners")]
use frontend::partners::pages::partner_list_page::PartnerListPage;
#[cfg(feature = "payables")]
use frontend::payables::pages::aged_payables::AgedPayablesPage;
#[cfg(feature = "payables")]
use frontend::payables::pages::new_vendor_invoice::NewVendorInvoicePage;
#[cfg(feature = "payables")]
use frontend::payables::pages::payables_ledger::PayablesLedgerPage;
#[cfg(feature = "sales")]
use frontend::sales::pages::aged_receivables::AgedReceivablesPage;
use frontend::{
    contexts::{
        auth_context::{
            UserContext,
            UserContextHandle,
        },
        locale_context::LocaleProvider,
        org_context::{
            OrgAction,
            OrgContextHandle,
            OrgState,
        },
        report_context::ReportContextProvider,
    },
    core::pages::{
        dashboard::DashboardPage,
        forgot_password::ForgotPasswordPage,
        login::LoginPage,
        profile::ProfilePage,
        register::RegisterPage,
        reset_password::ResetPasswordPage,
        roles::RolesPage,
        style_guide::StyleGuide,
        users::UsersPage,
    },
    inventory::pages::{
        warehouse_list::WarehouseListPage,
        warehouse_locations::WarehouseLocationsPage,
    },
    router::Route,
    sales::pages::{
        item_list::ItemListPage,
        new_sales_order::NewSalesOrderPage,
        sales_orders::SalesOrdersPage,
        tax_category_list::TaxCategoryListPage,
        uom_list::UomListPage,
    },
};
use gloo_net::http::Request;
use log::info;
use shared_core::core::{
    dtos::user_detail::AuthUserDetail,
    models::organization::Organization,
};
use yew::prelude::*;
use yew_router::prelude::*;

/// The component that contains the router and switches between pages.
#[function_component(AppRouter)]
fn app_router() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

/// The main App component, which provides the user context.
#[function_component(App)]
fn app() -> Html {
    let user_ctx = use_reducer(UserContext::default);
    let org_ctx = use_reducer(OrgState::default);

    {
        let user_ctx = user_ctx.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(resp) = Request::get("/api/auth/me").send().await {
                    if resp.ok() {
                        if let Ok(user) = resp.json::<AuthUserDetail>().await {
                            user_ctx.dispatch(Some(user));
                        } else {
                            return;
                        }
                    }
                }
            });
            || ()
        });
    }

    {
        let org_ctx = org_ctx.clone();
        // let user_org_id = user_ctx.organisation_id.clone();
        use_effect_with(user_ctx.clone(), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(resp) = Request::get("/api/organization").send().await {
                    if resp.ok() {
                        if let Ok(org) = resp.json::<Organization>().await {
                            info!("Org: {:?}", org);
                            org_ctx.dispatch(OrgAction::SetOrg(OrgState {
                                id: org.id,
                                name: org.name,
                                strict_audit_mode: org.strict_audit_mode,
                                locked_until: org.locked_until,
                            }));
                        } else {
                            return;
                        }
                    }
                }
            });
            || ()
        });
    }

    html! {
        <LocaleProvider>
            <ContextProvider<UserContextHandle> context={user_ctx}>
                <ContextProvider<OrgContextHandle> context={org_ctx}>
                    <ReportContextProvider>
                        <AppRouter />
                    </ReportContextProvider>
                </ContextProvider<OrgContextHandle>>
            </ContextProvider<UserContextHandle>>
        </LocaleProvider>
    }
}

/// The switch function to render the correct page based on the route.
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <DashboardPage /> },

        Route::Register => html! { <RegisterPage /> },
        Route::Login => html! { <LoginPage /> },
        Route::ForgotPassword => html! { <ForgotPasswordPage /> },
        Route::ResetPassword => html! { <ResetPasswordPage /> },
        Route::Dashboard => html! { <DashboardPage /> },
        Route::Profile => html! { <ProfilePage /> },
        Route::Users => html! { <UsersPage /> },
        Route::Roles => html! { <RolesPage /> },

        #[cfg(feature = "ledger")]
        Route::Ledger => html! { <LedgerPage /> },
        #[cfg(feature = "ledger")]
        Route::TrialBalance => html! { <TrialBalancePage /> },
        #[cfg(feature = "ledger")]
        Route::ProfitLoss => html! { <ProfitLossPage /> },
        #[cfg(feature = "ledger")]
        Route::BalanceSheet => html! { <BalanceSheetPage /> },
        #[cfg(feature = "ledger")]
        Route::GeneralLedger => html! { <GeneralLedgerReportPage /> },
        #[cfg(feature = "ledger")]
        Route::AccountLedger { id } => html! { <AccountLedgerPage account_id={id} /> },
        #[cfg(feature = "ledger")]
        Route::NewTransaction => html! { <NewTransactionPage /> },
        #[cfg(feature = "ledger")]
        Route::CloseYear => html! { <CloseYearPage /> },
        #[cfg(feature = "ledger")]
        Route::PeriodSettings => html! { <PeriodSettings /> },
        #[cfg(feature = "ledger")]
        Route::Configuration => html! { <ConfigurationPage /> },
        #[cfg(feature = "partners")]
        Route::PartnerList => html! { <PartnerListPage /> },
        #[cfg(feature = "payables")]
        Route::Payables => html! { <PayablesLedgerPage /> },
        #[cfg(feature = "payables")]
        Route::NewVendorInvoice => html! { <NewVendorInvoicePage /> },
        #[cfg(feature = "payables")]
        Route::AgedPayables => html! { <AgedPayablesPage /> },
        #[cfg(any(feature = "sales", feature = "inventory"))]
        Route::ItemList => html! { <ItemListPage /> },
        #[cfg(any(feature = "sales", feature = "inventory"))]
        Route::UomList => html! { <UomListPage /> },
        #[cfg(feature = "sales")]
        Route::SalesOrders => html! { <SalesOrdersPage /> },
        #[cfg(feature = "sales")]
        Route::NewSalesOrder => html! { <NewSalesOrderPage /> },
        #[cfg(feature = "sales")]
        Route::TaxCategoryList => html! { <TaxCategoryListPage /> },
        #[cfg(feature = "sales")]
        Route::AgedReceivables => html! { <AgedReceivablesPage /> },
        #[cfg(any(feature = "inventory"))]
        Route::WarehouseList => html! { <WarehouseListPage /> },
        #[cfg(any(feature = "inventory"))]
        Route::WarehouseLocations { id } => html! { <WarehouseLocationsPage warehouse_id = {id}/> },

        Route::StyleGuide => html! {<StyleGuide />},
    }
}

fn main() {
    console_log::init_with_level(log::Level::Debug).expect("error initializing logger");
    yew::Renderer::<App>::new().render();
}
