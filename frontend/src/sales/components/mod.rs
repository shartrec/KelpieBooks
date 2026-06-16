/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
pub mod edit_item_modal;
pub mod item_filter;
pub mod item_list_table;
pub mod item_row;
pub mod add_item_modal;

use shared_core::core::models::auth::SystemPrivilege;
use crate::core::components::sidebar::SidebarModuleContribution;
use crate::router::Route;

#[cfg(feature = "sales")]
pub fn get_sidebar_contribution() -> Option<SidebarModuleContribution> {
    Some(SidebarModuleContribution {
        id: "sidebar-sales",
        label_key: "sidebar-sales",
        privilege: Some(SystemPrivilege::UseSales),
        target_route: None,
        children: vec![
            SidebarModuleContribution {
                id: "sales-item-list",
                label_key: "item-list-title",
                privilege: Some(SystemPrivilege::UseVendorInvoices),
                target_route: Some(Route::ItemList),
                children: vec![],
            },
            SidebarModuleContribution {
                id: "sidebar-sales-reports",
                label_key: "sidebar-reports",
                privilege: Some(SystemPrivilege::UseVendorInvoices),
                target_route: None,
                children: vec![],
            },
        ],
    })
}
