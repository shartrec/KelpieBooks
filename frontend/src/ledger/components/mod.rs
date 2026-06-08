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

pub mod account_row;
pub mod add_account_modal;
pub mod chart_of_accounts_table;
pub mod edit_account_modal;
pub mod je_delete_confirmation_modal;
pub mod je_reversal_confirmation_modal;
pub mod journal_entry_row;
pub mod transaction_row;

#[cfg(feature = "ledger")]
pub fn get_sidebar_contribution() -> Option<SidebarModuleContribution> {
    Some(SidebarModuleContribution {
        label_key: "sidebar-accounts",
        privilege: Some(SystemPrivilege::use_accounts),
        target_route: None,
        children: vec![
            SidebarModuleContribution {
                label_key: "coa-title",
                privilege: Some(SystemPrivilege::use_accounts),
                target_route: Some(Route::Ledger),
                children: vec![],
            },
            SidebarModuleContribution {
                label_key: "sidebar-tasks",
                privilege: Some(SystemPrivilege::manage_accounts),
                target_route: None,
                children: vec![
                    SidebarModuleContribution {
                        label_key: "sidebar-close-year",
                        privilege: Some(SystemPrivilege::manage_accounts),
                        target_route: Some(Route::CloseYear),
                        children: vec![],
                    },
                    SidebarModuleContribution {
                        label_key: "sidebar-period-settings",
                        privilege: Some(SystemPrivilege::manage_accounts),
                        target_route: Some(Route::PeriodSettings),
                        children: vec![],
                    },
                    SidebarModuleContribution {
                        label_key: "sidebar-configuration",
                        privilege: Some(SystemPrivilege::manage_accounts),
                        target_route: Some(Route::Configuration),
                        children: vec![],
                    },
                ],
            },
            SidebarModuleContribution {
                label_key: "sidebar-reports",
                privilege: Some(SystemPrivilege::use_transactions),
                target_route: None,
                children: vec![
                    SidebarModuleContribution {
                        label_key: "sidebar-trial-balance",
                        privilege: Some(SystemPrivilege::use_transactions),
                        target_route: Some(Route::TrialBalance),
                        children: vec![],
                    },
                    SidebarModuleContribution {
                        label_key: "sidebar-profit-loss",
                        privilege: Some(SystemPrivilege::use_transactions),
                        target_route: Some(Route::ProfitLoss),
                        children: vec![],
                    },
                    SidebarModuleContribution {
                        label_key: "sidebar-balance-sheet",
                        privilege: Some(SystemPrivilege::use_transactions),
                        target_route: Some(Route::BalanceSheet),
                        children: vec![],
                    },
                    SidebarModuleContribution {
                        label_key: "sidebar-general-ledger",
                        privilege: Some(SystemPrivilege::use_transactions),
                        target_route: Some(Route::GeneralLedger),
                        children: vec![],
                    },
                ],
            },
        ],
    })
}
