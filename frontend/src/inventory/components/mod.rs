/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
pub mod bulk_generator_modal;
pub mod location_grid;
pub mod location_tree;
pub mod receiving_modal;
pub mod stock_adjustment_modal;
pub mod warehouse_list_table;
pub mod warehouse_modal;
pub mod warehouse_row;

use shared_core::core::models::auth::SystemPrivilege;

use crate::{
    core::components::sidebar::SidebarModuleContribution,
    router::Route,
};

#[cfg(feature = "inventory")]
pub fn get_sidebar_contribution() -> Option<SidebarModuleContribution> {
    Some(SidebarModuleContribution {
        id: "sidebar-products",
        label_key: "sidebar-products",
        privilege: Some(SystemPrivilege::UseInventory),
        target_route: None,
        children: vec![
            SidebarModuleContribution {
                id: "inventory-warehouse-list",
                label_key: "inventory-warehouse-title",
                privilege: Some(SystemPrivilege::UseInventory),
                target_route: Some(Route::WarehouseList),
                children: vec![],
            },
            SidebarModuleContribution {
                id: "sales-item-list",
                label_key: "item-list-title",
                privilege: Some(SystemPrivilege::UseInventory),
                target_route: Some(Route::ItemList),
                children: vec![],
            },
            SidebarModuleContribution {
                id: "sales-uom-list",
                label_key: "uom-list-title",
                privilege: Some(SystemPrivilege::UseInventory),
                target_route: Some(Route::UomList),
                children: vec![],
            },
        ],
    })
}
