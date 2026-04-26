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

use serde::Deserialize;
use yew::prelude::*;

/// This struct must match the `CurrentUser` struct from the backend.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CurrentUser {
    pub username: String, // This is the email
    pub full_name: String,
    pub display_name: Option<String>,
    pub role: String,
}

/// The context that will hold the user's state.
/// `Option<CurrentUser>` is used because the user might not be logged in.
#[derive(Debug, Clone, PartialEq)]
pub struct UserContext {
    pub user: Option<CurrentUser>,
}

impl Reducible for UserContext {
    type Action = Option<CurrentUser>;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        UserContext { user: action }.into()
    }
}
impl Default for UserContext {
    fn default() -> Self {
        Self { user: None }
    }
}

pub type UserContextHandle = UseReducerHandle<UserContext>;
