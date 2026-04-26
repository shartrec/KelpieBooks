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

use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route; // Assuming your router's Route enum is in lib.rs or main.rs

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    html! {
        <aside class="sidebar">
            <div class="sidebar-header">
                <h2>{ "KelpieBooks" }</h2>
            </div>
            <nav class="sidebar-nav">
                <ul>
                    <li><Link<Route> to={Route::Dashboard}>{ "Dashboard" }</Link<Route>></li>
                    <li><Link<Route> to={Route::Ledger}>{ "Ledger" }</Link<Route>></li>
                    // Add more links as pages are created
                    // <li><Link<Route> to={Route::Accounts}>{ "Accounts" }</Link<Route>></li>
                    // <li><Link<Route> to={Route::Transactions}>{ "Transactions" }</Link<Route>></li>
                    // <li><Link<Route> to={Route::Reports}>{ "Reports" }</Link<Route>></li>
                </ul>
            </nav>
        </aside>
    }
}
