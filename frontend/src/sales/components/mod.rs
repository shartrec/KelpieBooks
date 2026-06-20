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
pub mod uom_list_table;
pub mod uom_row;
pub mod add_uom_modal;
pub mod edit_uom_modal;
pub mod delete_uom_confirmation_modal;
pub mod tax_category_list_table;
pub mod tax_category_row;
pub mod add_tax_category_modal;
pub mod edit_tax_category_modal;
pub mod tax_category_drawer;
pub mod sales_invoice_item_row;
pub mod sales_invoice_filter;
pub mod sales_invoice_table;

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
                id: "sales-invoice-list",
                label_key: "sales-invoice-list",
                privilege: Some(SystemPrivilege::ManageSales),
                target_route: Some(Route::SalesLedger),
                children: vec![],
            },
            SidebarModuleContribution {
                id: "sales-new-invoice",
                label_key: "new-sales-invoice-title",
                privilege: Some(SystemPrivilege::ManageSales),
                target_route: Some(Route::NewSalesInvoice),
                children: vec![],
            },
            SidebarModuleContribution {
                id: "sales-item-list",
                label_key: "item-list-title",
                privilege: Some(SystemPrivilege::UseSales),
                target_route: Some(Route::ItemList),
                children: vec![],
            },
            SidebarModuleContribution {
                id: "sales-uom-list",
                label_key: "uom-list-title",
                privilege: Some(SystemPrivilege::UseSales),
                target_route: Some(Route::UomList),
                children: vec![],
            },
            SidebarModuleContribution {
                id: "sales-tax-category-list",
                label_key: "tax-category-list-title",
                privilege: Some(SystemPrivilege::UseSales),
                target_route: Some(Route::TaxCategoryList),
                children: vec![],
            },
            SidebarModuleContribution {
                id: "sidebar-sales-reports",
                label_key: "sidebar-reports",
                privilege: Some(SystemPrivilege::UseSales),
                target_route: None,
                children: vec![],
            },
        ],
    })
}