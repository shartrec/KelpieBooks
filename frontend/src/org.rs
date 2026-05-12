/*
 * Copyright (c) 2026. Trevor Campbell and others.
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

use std::rc::Rc;
use yew::prelude::*;
use chrono::NaiveDate;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrgState {
    pub id: Uuid,
    pub name: String,
    pub strict_audit_mode: bool,
    pub locked_until: Option<NaiveDate>,
}

pub enum OrgAction {
    SetOrg(OrgState),
    UpdateLockDate(Option<NaiveDate>),
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
        }
    }
}

pub type OrgContextHandle = UseReducerHandle<OrgState>;