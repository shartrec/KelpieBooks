/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use shared_core::core::models::auth::SystemPrivilege;

use crate::{
    core::components::sidebar::SidebarModuleContribution,
    router::Route,
};

pub mod add_partner_modal;
pub mod delete_partner_confirmation_modal;
pub mod partner_drawer;
pub mod partner_list_table;
pub mod partner_row;

#[cfg(feature = "payables")]
pub fn get_sidebar_contribution() -> Option<SidebarModuleContribution> {
    Some(SidebarModuleContribution {
        id: "sidebar-partners",
        label_key: "sidebar-partners",
        privilege: Some(SystemPrivilege::UsePartners),
        target_route: None,
        children: vec![SidebarModuleContribution {
            id: "partner-list",
            label_key: "partner-list-title",
            privilege: Some(SystemPrivilege::UsePartners),
            target_route: Some(Route::PartnerList),
            children: vec![],
        }],
    })
}
