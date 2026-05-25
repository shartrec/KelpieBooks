/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
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
pub(crate) mod vendor_invoices;
pub(crate) mod vendor_payments;

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
