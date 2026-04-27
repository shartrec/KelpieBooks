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

use frontend::pages::register::{RegisterPage};
use yew::prelude::*;
use yew_router::prelude::*;
use frontend::pages::login::LoginPage;
use frontend::pages::dashboard::DashboardPage;
use frontend::pages::profile::ProfilePage;
use frontend::pages::ledger::LedgerPage;
use frontend::pages::account_ledger::AccountLedgerPage;
use frontend::pages::new_transaction::NewTransactionPage;
use frontend::Route;
use frontend::auth::{UserContext, UserContextHandle};
use gloo_net::http::Request;
use shared_core::dtos::user_detail::UserDetail;

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

    {
        let user_ctx = user_ctx.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(resp) = Request::get("/api/auth/me").send().await {
                    if resp.ok() {
                        if let Ok(user) = resp.json::<UserDetail>().await {
                            user_ctx.dispatch(Some(user));
                        }
                    }
                }
            });
            || ()
        });
    }

    html! {
        <ContextProvider<UserContextHandle> context={user_ctx}>
            <AppRouter />
        </ContextProvider<UserContextHandle>>
    }
}

/// The switch function to render the correct page based on the route.
fn switch(routes: Route) -> Html {
    match routes {
        Route::Register => html! { <RegisterPage /> },
        Route::Login => html! { <LoginPage /> },
        Route::Onboard => html! { <h1>{ "Onboarding" }</h1> },
        Route::Dashboard => html! { <DashboardPage /> },
        Route::Profile => html! { <ProfilePage /> },
        Route::Ledger => html! { <LedgerPage /> },
        Route::AccountLedger { id } => html! { <AccountLedgerPage account_id={id} /> },
        Route::NewTransaction => html! { <NewTransactionPage /> },
        Route::Home => html! { <LoginPage /> },
    }
}

fn main() {
    console_log::init_with_level(log::Level::Debug).expect("error initializing logger");
    yew::Renderer::<App>::new().render();
}
