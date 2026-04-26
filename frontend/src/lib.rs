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
use yew_router::Routable;

pub mod pages;
pub mod components;
pub mod auth;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/register")]
    Register,
    #[at("/login")]
    Login,
    #[at("/onboard")]
    Onboard,
    #[at("/dashboard")]
    Dashboard,
    #[at("/profile")]
    Profile,
    #[at("/ledger")]
    Ledger,
    #[at("/")]
    Home,
}
