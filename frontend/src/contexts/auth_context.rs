/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use std::rc::Rc;

use shared_core::core::models::auth::SystemPrivilege;
use yew::prelude::*;
use shared_core::core::dtos::user_detail::AuthUserDetail;

pub type UserContextHandle = UseReducerHandle<UserContext>;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct UserContext {
    pub user: Option<AuthUserDetail>,
}

impl UserContext {
    /// Checks whether the currently authenticated user possesses a given capability string flag.
    /// Returns false immediately if the user is unauthenticated (Logged out).
    pub fn has_privilege(&self, required_privilege: &SystemPrivilege) -> bool {
        let Some(ref auth_detail) = self.user else {
            return false; // 🔒 Anonymous user -> no access to protected UI options
        };

        // Assuming your backend maps privileges into `auth_detail.user.privileges`
        // or directly on the `auth_detail` wrapper from your custom DTO:
        let privileges = &auth_detail.privileges;

        // 👑 Shortcut: Organization administrators automatically bypass individual module blocks
        if privileges
            .iter()
            .any(|p| p == SystemPrivilege::security_admin.as_str())
        {
            return true;
        }

        // Match against the exact system capability string variant requested
        privileges.iter().any(|p| p == required_privilege.as_ref())
    }
}

impl Reducible for UserContext {
    type Action = Option<AuthUserDetail>;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Self { user: action }.into()
    }
}

#[hook]
pub fn use_user_context() -> UserContextHandle {
    use_context::<UserContextHandle>().unwrap()
}
