/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

pub mod about_modal;
pub mod add_role_modal;
pub mod add_user_modal;
pub mod currency_input;
pub mod delete_confirmation_modal;
pub mod edit_role_modal;
pub mod edit_user_modal;
pub mod header;
pub mod layout;
pub mod progressive_search;
pub mod report_options;
pub mod sidebar;

pub trait SearchableItem: Clone + PartialEq + 'static {
    /// The strongly-typed identifier for this item (e.g., AccountId, RoleId).
    type Id: Copy + Eq + std::hash::Hash + std::fmt::Display + 'static;

    fn id(&self) -> Self::Id;
    fn display_label(&self) -> String;
    fn subtitle(&self) -> Option<String> {
        None
    }
}