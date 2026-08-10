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
        id: "sidebar-accounts",
        label_key: "sidebar-accounts",
        privilege: Some(SystemPrivilege::UseAccounts),
        target_route: None,
        on_click: None,
        children: vec![
            SidebarModuleContribution {
                id: "coa-title",
                label_key: "coa-title",
                privilege: Some(SystemPrivilege::UseAccounts),
                target_route: Some(Route::Ledger),
                on_click: None,
                children: vec![],
            },
            SidebarModuleContribution {
                id: "sidebar-tasks",
                label_key: "sidebar-tasks",
                privilege: Some(SystemPrivilege::ManageAccounts),
                target_route: None,
                on_click: None,
                children: vec![
                    SidebarModuleContribution {
                        id: "sidebar-close-year",
                        label_key: "sidebar-close-year",
                        privilege: Some(SystemPrivilege::ManageAccounts),
                        target_route: Some(Route::CloseYear),
                        on_click: None,
                        children: vec![],
                    },
                    SidebarModuleContribution {
                        id: "sidebar-period-settings",
                        label_key: "sidebar-period-settings",
                        privilege: Some(SystemPrivilege::ManageAccounts),
                        target_route: Some(Route::PeriodSettings),
                        on_click: None,
                        children: vec![],
                    },
                    SidebarModuleContribution {
                        id: "sidebar-configuration",
                        label_key: "sidebar-configuration",
                        privilege: Some(SystemPrivilege::ManageAccounts),
                        target_route: Some(Route::Configuration),
                        on_click: None,
                        children: vec![],
                    },
                ],
            },
            SidebarModuleContribution {
                id: "sidebar-ledger-reports",
                label_key: "sidebar-reports",
                privilege: Some(SystemPrivilege::UseTransactions),
                target_route: None,
                on_click: None,
                children: vec![
                    SidebarModuleContribution {
                        id: "sidebar-trial-balance",
                        label_key: "sidebar-trial-balance",
                        privilege: Some(SystemPrivilege::UseTransactions),
                        target_route: Some(Route::TrialBalance),
                        on_click: None,
                        children: vec![],
                    },
                    SidebarModuleContribution {
                        id: "sidebar-profit-loss",
                        label_key: "sidebar-profit-loss",
                        privilege: Some(SystemPrivilege::UseTransactions),
                        target_route: Some(Route::ProfitLoss),
                        on_click: None,
                        children: vec![],
                    },
                    SidebarModuleContribution {
                        id: "sidebar-balance-sheet",
                        label_key: "sidebar-balance-sheet",
                        privilege: Some(SystemPrivilege::UseTransactions),
                        target_route: Some(Route::BalanceSheet),
                        on_click: None,
                        children: vec![],
                    },
                    SidebarModuleContribution {
                        id: "sidebar-general-ledger",
                        label_key: "sidebar-general-ledger",
                        privilege: Some(SystemPrivilege::UseTransactions),
                        target_route: Some(Route::GeneralLedger),
                        on_click: None,
                        children: vec![],
                    },
                ],
            },
        ],
    })
}
