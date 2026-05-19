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

pub(crate) mod accounts;
pub(crate) mod configurations;
pub(crate) mod dashboard;
pub(crate) mod onboarding;
pub(crate) mod organization;
pub(crate) mod partners;
pub(crate) mod period_end;
pub(crate) mod reports;
pub(crate) mod security;
pub(crate) mod transactions;
pub(crate) mod users;

#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Admin,
    User,
    Guest,
}

impl Role {
    pub fn is_admin(&self) -> bool {
        matches!(self, Role::Admin)
    }

    pub fn is_user(&self) -> bool {
        matches!(self, Role::User)
    }

    pub fn is_guest(&self) -> bool {
        matches!(self, Role::Guest)
    }

    pub(crate) fn from(role_str: &str) -> Option<Role> {
        match role_str.to_lowercase().as_str() {
            "admin" => Some(Role::Admin),
            "user" => Some(Role::User),
            _ => Some(Role::Guest), // Default to Guest if not recognized
        }
    }
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Admin => write!(f, "Admin"),
            Role::User => write!(f, "User"),
            Role::Guest => write!(f, "Guest"),
        }
    }
}
