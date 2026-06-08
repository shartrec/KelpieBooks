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

pub mod aged_trial_balance_matrix;
pub mod vendor_invoice_drawer;
pub mod vendor_invoice_filter;
pub mod vendor_invoice_item_row;
pub mod vendor_invoice_table;

#[cfg(feature = "payables")]
pub fn get_sidebar_contribution() -> Option<SidebarModuleContribution> {
    Some(SidebarModuleContribution {
        id: "sidebar-payables",
        label_key: "sidebar-payables",
        privilege: Some(SystemPrivilege::use_vendor_invoices),
        target_route: None,
        children: vec![
            SidebarModuleContribution {
                id: "payables-ledger",
                label_key: "payables-ledger-title",
                privilege: Some(SystemPrivilege::use_vendor_invoices),
                target_route: Some(Route::Payables),
                children: vec![],
            },
            SidebarModuleContribution {
                id: "sidebar-payables-reports",
                label_key: "sidebar-reports",
                privilege: Some(SystemPrivilege::use_vendor_invoices),
                target_route: None,
                children: vec![SidebarModuleContribution {
                    id: "sidebar-aged-payables",
                    label_key: "sidebar-aged-payables",
                    privilege: Some(SystemPrivilege::use_vendor_invoices),
                    target_route: Some(Route::AgedPayables),
                    children: vec![],
                }],
            },
        ],
    })
}
