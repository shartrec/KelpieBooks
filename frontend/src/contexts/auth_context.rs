/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use shared_core::dtos::user_detail::UserDetail;
use std::rc::Rc;
use yew::prelude::*;

pub type UserContextHandle = UseReducerHandle<UserContext>;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct UserContext {
    pub user: Option<UserDetail>,
}

impl Reducible for UserContext {
    type Action = Option<UserDetail>;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Self { user: action }.into()
    }
}

#[hook]
pub fn use_user_context() -> UserContextHandle {
    use_context::<UserContextHandle>().unwrap()
}
