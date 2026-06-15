/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use rocket::{
    http::Status,
    request::{
        FromRequest,
        Outcome,
    },
    Request,
};
use shared_core::core::models::auth::SystemPrivilege;

use crate::core::routes::security::AuthenticatedUser;
#[rustfmt::skip]  // This is to stop all the markers from being reformatted.

use std::marker::PhantomData;

pub(crate) trait GuardPrivilege {
    const VALUE: SystemPrivilege;
}

// Create structural markers
pub(crate) struct SecurityAdmin;
impl GuardPrivilege for SecurityAdmin {
    const VALUE: SystemPrivilege = SystemPrivilege::SecurityAdmin;
}

pub(crate) struct UseAccounts;
impl GuardPrivilege for UseAccounts {
    const VALUE: SystemPrivilege = SystemPrivilege::UseAccounts;
}

pub(crate) struct ManageAccounts;
impl GuardPrivilege for ManageAccounts {
    const VALUE: SystemPrivilege = SystemPrivilege::ManageAccounts;
}

#[cfg(feature = "partners")]
pub(crate) struct UsePartners;
#[cfg(feature = "partners")]
impl GuardPrivilege for UsePartners {
    const VALUE: SystemPrivilege = SystemPrivilege::UsePartners;
}

#[cfg(feature = "partners")]
pub(crate) struct ManagePartners;
#[cfg(feature = "partners")]
impl GuardPrivilege for ManagePartners {
    const VALUE: SystemPrivilege = SystemPrivilege::ManagePartners;
}

#[cfg(feature = "payables")]
pub(crate) struct UseVendorInvoices;
#[cfg(feature = "payables")]
impl GuardPrivilege for UseVendorInvoices {
    const VALUE: SystemPrivilege = SystemPrivilege::UseVendorInvoices;
}
#[cfg(feature = "payables")]
pub(crate) struct ManageVendorInvoices;
#[cfg(feature = "payables")]
impl GuardPrivilege for ManageVendorInvoices {
    const VALUE: SystemPrivilege = SystemPrivilege::ManageVendorInvoices;
}

#[cfg(feature = "sales")]
pub(crate) struct UseSales;
#[cfg(feature = "sales")]
impl GuardPrivilege for UseSales {
    const VALUE: SystemPrivilege = SystemPrivilege::UseSales;
}

#[cfg(feature = "sales")]
pub(crate) struct ManageSales;
#[cfg(feature = "sales")]
impl GuardPrivilege for ManageSales {
    const VALUE: SystemPrivilege = SystemPrivilege::ManageSales;
}

pub(crate) struct UseTransactions;
impl GuardPrivilege for UseTransactions {
    const VALUE: SystemPrivilege = SystemPrivilege::UseTransactions;
}

pub(crate) struct ManageTransactions;
impl GuardPrivilege for ManageTransactions {
    const VALUE: SystemPrivilege = SystemPrivilege::ManageTransactions;
}

pub(crate) struct ManageUsers;
impl GuardPrivilege for ManageUsers {
    const VALUE: SystemPrivilege = SystemPrivilege::ManageUsers;
}

pub(crate) struct ManageOrganization;
impl GuardPrivilege for ManageOrganization {
    const VALUE: SystemPrivilege = SystemPrivilege::ManageOrganization;
}

// 💡 The Request Guard now takes the Trait Type instead of the Enum constant!
pub(crate) struct RequirePrivilege<T: GuardPrivilege>(
    pub(crate) AuthenticatedUser,
    std::marker::PhantomData<T>,
);

#[rocket::async_trait]
impl<'r, T: GuardPrivilege> FromRequest<'r> for RequirePrivilege<T> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // 1. Evaluate your existing session token guard path first
        let user_outcome = request.guard::<AuthenticatedUser>().await;

        match user_outcome {
            Outcome::Success(user) => {
                // 2. Safely verify permissions using your custom array mapping layer
                if let Some(ref role) = user.role {
                    if role.privileges.contains(&T::VALUE) {
                        return Outcome::Success(RequirePrivilege(user, PhantomData));
                    }
                }

                // Security Audit Logging
                rocket::error!(
                    "Unauthorized Access Blocked: User {} lacks required privilege {:?}",
                    user.full_name,
                    T::VALUE
                );
                Outcome::Error((Status::Forbidden, ()))
            }
            // Bubble up existing unauthenticated or parsing failures cleanly
            Outcome::Error((status, _)) => Outcome::Error((status, ())),
            Outcome::Forward(status) => Outcome::Forward(status),
        }
    }
}
