/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use frontend::contexts::auth_context::{UserContext, UserContextHandle};
use frontend::contexts::org_context::{OrgAction, OrgContextHandle, OrgState};
use frontend::contexts::report_context::ReportContextProvider;
use frontend::pages::account_ledger::AccountLedgerPage;
use frontend::pages::balance_sheet::BalanceSheetPage;
use frontend::pages::close_year::CloseYearPage;
use frontend::pages::configuration::ConfigurationPage;
use frontend::pages::dashboard::DashboardPage;
use frontend::pages::general_ledger_report::GeneralLedgerReportPage;
use frontend::pages::ledger::LedgerPage;
use frontend::pages::login::LoginPage;
use frontend::pages::new_transaction::NewTransactionPage;
use frontend::pages::new_vendor_invoice::NewVendorInvoicePage;
use frontend::pages::partner_list_page::PartnerListPage;
use frontend::pages::payables_ledger::PayablesLedgerPage;
use frontend::pages::period_settings::PeriodSettings;
use frontend::pages::profile::ProfilePage;
use frontend::pages::profit_loss::ProfitLossPage;
use frontend::pages::register::RegisterPage;
use frontend::pages::style_guide::StyleGuide;
use frontend::pages::trial_balance::TrialBalancePage;
use frontend::router::Route;
use gloo_net::http::Request;
use log::info;
use shared_core::dtos::user_detail::UserDetail;
use shared_core::models::organization::Organization;
use yew::prelude::*;
use yew_router::prelude::*;
use frontend::contexts::locale_context::LocaleProvider;

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
                        if let Ok(user) = resp.json::<UserDetail>().await {
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
        Route::Register => html! { <RegisterPage /> },
        Route::Login => html! { <LoginPage /> },
        Route::Dashboard => html! { <DashboardPage /> },
        Route::Profile => html! { <ProfilePage /> },
        Route::Ledger => html! { <LedgerPage /> },
        Route::PartnerList => html! { <PartnerListPage /> },
        Route::Payables => html! { <PayablesLedgerPage /> },
        Route::NewVendorInvoice => html! { <NewVendorInvoicePage /> },
        Route::TrialBalance => html! { <TrialBalancePage /> },
        Route::ProfitLoss => html! { <ProfitLossPage /> },
        Route::BalanceSheet => html! { <BalanceSheetPage /> },
        Route::GeneralLedger => html! { <GeneralLedgerReportPage /> },
        Route::AccountLedger { id } => html! { <AccountLedgerPage account_id={id} /> },
        Route::NewTransaction => html! { <NewTransactionPage /> },
        Route::CloseYear => html! { <CloseYearPage /> },
        Route::PeriodSettings => html! { <PeriodSettings /> },
        Route::Configuration => html! { <ConfigurationPage /> },
        Route::Home => html! { <LoginPage /> },
        Route::StyleGuide => html! {<StyleGuide />},
    }
}

fn main() {
    console_log::init_with_level(log::Level::Debug).expect("error initializing logger");
    yew::Renderer::<App>::new().render();
}
