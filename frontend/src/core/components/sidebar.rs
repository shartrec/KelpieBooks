/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use shared_core::core::models::auth::SystemPrivilege;
use yew::prelude::*;
use yew_router::prelude::*;

#[cfg(feature = "ledger")]
use crate::ledger;
#[cfg(feature = "partners")]
use crate::partners;
#[cfg(feature = "payables")]
use crate::payables;
use crate::{
    contexts::{
        auth_context::use_user_context,
        locale_context::use_locale,
    },
    inventory,
    core::components::about_modal::AboutModal,
    router::Route,
    sales,
};

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let show_about = use_state(|| false);

    let mut registry: Vec<SidebarModuleContribution> = vec![];

    // Core Ledger Module
    #[cfg(feature = "ledger")]
    if user_ctx.has_privilege(&SystemPrivilege::UseAccounts) {
        if let Some(contrib) = ledger::components::get_sidebar_contribution() {
            registry.push(contrib);
        }
    }

    #[cfg(feature = "partners")]
    if user_ctx.has_privilege(&SystemPrivilege::UseAccounts) {
        if let Some(contrib) = partners::components::get_sidebar_contribution() {
            registry.push(contrib);
        }
    }

    #[cfg(feature = "inventory")]
    if user_ctx.has_privilege(&SystemPrivilege::UseInventory) {
        if let Some(contrib) = inventory::components::get_sidebar_contribution() {
            registry.push(contrib);
        }
    }

    #[cfg(all(feature = "sales", not(feature = "inventory")))]
    if user_ctx.has_privilege(&SystemPrivilege::UseSales) {
        if let Some(contrib) = sales::components::get_sidebar_item_contribution() {
            registry.push(contrib);
        }
    }

    #[cfg(feature = "payables")]
    if user_ctx.has_privilege(&SystemPrivilege::UseVendorInvoices) {
        if let Some(contrib) = payables::components::get_sidebar_contribution() {
            registry.push(contrib);
        }
    }

    #[cfg(feature = "sales")]
    if user_ctx.has_privilege(&SystemPrivilege::UseVendorInvoices) {
        if let Some(contrib) = sales::components::get_sidebar_contribution() {
            registry.push(contrib);
        }
    }

    if user_ctx.has_privilege(&SystemPrivilege::ManageUsers) {
        if let Some(contrib) = get_core_contribution() {
            registry.push(contrib);
        }
    }

    let on_about_open = {
        let show_about = show_about.clone();
        Callback::from(move |_| show_about.set(true))
    };
    let on_about_close = {
        let show_about = show_about.clone();
        Callback::from(move |()| show_about.set(false))
    };

    if let Some(contrib) = get_help_contribution(on_about_open) {
        registry.push(contrib);
    }

    html! {
        <>
            if *show_about {
                <AboutModal on_close={on_about_close} />
            }
            <aside class="sidebar">
                <div class="sidebar__header">
                    <img src="/images/kelpiedog_120x120_transparent.png" alt={i18n.t("sidebar-logo-alt")} class="sidebar__logo" />
                    <h2>{ i18n.t("branding-app-name") }</h2>
                </div>
                <nav class="sidebar__nav">
                    <ul>
                        <li><Link<Route> to={Route::Dashboard}>{ i18n.t("sidebar-dashboard") }</Link<Route>></li>

                        // 💡 Iterate through the discovered structural modules dynamically
                        { for registry.into_iter().map(|item| html! {
                            <SidebarGroupNode item={item} depth={0}/>
                        })}
                    </ul>
                </nav>
            </aside>
        </>
    }
}

/// Represents a single link or an entire nested module block in the navigation sidebar
#[derive(Clone, PartialEq)]
pub struct SidebarModuleContribution {
    pub id: &'static str,
    pub label_key: &'static str, // The translation key for fluent i18n
    pub privilege: Option<SystemPrivilege>, // Mandatory clearance flag if applicable
    pub target_route: Option<Route>, // Target destination if it's a leaf node
    pub on_click: Option<Callback<()>>, // Callback action, use route or callback not both
    pub children: Vec<SidebarModuleContribution>, // Submenu arrays (e.g., Reports)
}

#[derive(Properties, PartialEq)]
struct GroupNodeProps {
    item: SidebarModuleContribution,
    pub depth: usize, // 💡 Tracks recursive rendering depth for CSS indentations
}

/// A reusable component that manages its own toggle state and nested link maps
#[function_component(SidebarGroupNode)]
fn sidebar_group_node(props: &GroupNodeProps) -> Html {
    let i18n = use_locale();
    let item = &props.item;
    let current_depth = props.depth;

    // 💡 Create a deterministic session storage identifier using the unique localization translation token
    let storage_key = format!("kb_nav_open_{}", item.id);

    // Initialize toggle state directly from browser session history cache if it exists
    let is_open = use_state(|| {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.session_storage() {
                if let Ok(Some(val)) = storage.get_item(&storage_key) {
                    return val == "true";
                }
            }
        }
        false
    });

    let toggle = {
        let is_open = is_open.clone();
        let storage_key = storage_key.clone();
        Callback::from(move |_| {
            let next_state = !*is_open;

            // Persist the UI configuration string state cleanly to disk storage context loops
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.session_storage() {
                    let _ =
                        storage.set_item(&storage_key, if next_state { "true" } else { "false" });
                }
            }
            is_open.set(next_state);
        })
    };

    let item = &props.item;

    if let Some(target_route) = &item.target_route {
        // Simple direct leaf link node
        html! {
            <li><Link<Route> to={target_route.clone()}>{ i18n.t(item.label_key) }</Link<Route>></li>
        }
    } else if let Some(on_click) = &item.on_click {
        // 2. Action Button Node (Matches standard link styling)
        let on_click = on_click.clone();
        let handle_click = Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            on_click.emit(());
        });

        html! {
            <li>
                <a href="#" class="sidebar__action-item" onclick={handle_click}>
                    { i18n.t(item.label_key) }
                </a>
            </li>
        }
    } else {
        // Group Dropdown Node with child nodes
        html! {
            <li class="sidebar__group">
                <div class="sidebar__group-header" onclick={toggle}>
                    <span>{ i18n.t(item.label_key) }</span>
                    <img src="/images/chevron-right.svg" alt={i18n.t("common-toggle")} class={if *is_open { "is-rotated" } else { "" }} />
                </div>
                if *is_open {
                    <ul class="sidebar__sub-nav" style={format!("--depth: {};", current_depth + 1)}>
                        { for item.children.iter().map(|child| html! {
                            <SidebarGroupNode item={child.clone()} depth={current_depth + 1}/>
                        })}
                    </ul>
                }
            </li>
        }
    }
}

pub fn get_core_contribution() -> Option<SidebarModuleContribution> {
    Some(SidebarModuleContribution {
        id: "sidebar-admin",
        label_key: "sidebar-admin",
        privilege: Some(SystemPrivilege::ManageUsers),
        target_route: None,
        on_click: None,
        children: vec![
            SidebarModuleContribution {
                id: "sidebar-users",
                label_key: "sidebar-users",
                privilege: Some(SystemPrivilege::ManageUsers),
                target_route: Some(Route::Users),
                on_click: None,
                children: vec![],
            },
            SidebarModuleContribution {
                id: "sidebar-roles",
                label_key: "sidebar-roles",
                privilege: Some(SystemPrivilege::ManageUsers),
                target_route: Some(Route::Roles),
                on_click: None,
                children: vec![],
            },
        ],
    })
}

pub fn get_help_contribution(on_about_open: Callback<()>) -> Option<SidebarModuleContribution> {
    Some(SidebarModuleContribution {
        id: "sidebar-help",
        label_key: "sidebar-help",
        privilege: Some(SystemPrivilege::ManageUsers),
        target_route: None,
        on_click: None,
        children: vec![
            SidebarModuleContribution {
                id: "sidebar-about",
                label_key: "sidebar-about",
                privilege: Some(SystemPrivilege::ManageUsers),
                target_route: None,
                on_click: Some(on_about_open),
                children: vec![],
            },
        ],
    })
}
