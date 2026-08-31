/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use std::rc::Rc;

use chrono::NaiveDate;
use shared_core::OrgId;
use yew::prelude::*;

pub type OrgContextHandle = UseReducerHandle<OrgState>;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct OrgState {
    pub id: OrgId,
    pub name: String,
    pub strict_audit_mode: bool,
    pub locked_until: Option<NaiveDate>,
}

pub enum OrgAction {
    SetOrg(OrgState),
    UpdateLockDate(Option<NaiveDate>),
    UpdateAuditMode(bool),
}

impl Reducible for OrgState {
    type Action = OrgAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            OrgAction::SetOrg(new_state) => Rc::new(new_state),
            OrgAction::UpdateLockDate(new_date) => {
                let mut current = (*self).clone();
                current.locked_until = new_date;
                Rc::new(current)
            }
            OrgAction::UpdateAuditMode(new_mode) => {
                let mut current = (*self).clone();
                current.strict_audit_mode = new_mode;
                Rc::new(current)
            }
        }
    }
}

#[hook]
pub fn use_org_context() -> OrgContextHandle {
    use_context::<OrgContextHandle>().unwrap()
}
