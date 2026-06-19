/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use uuid::Uuid;

pub mod add_role_modal;
pub mod add_user_modal;
pub mod currency_input;
pub mod delete_confirmation_modal;
pub mod edit_role_modal;
pub mod edit_user_modal;
pub mod generic_delete_confirmation_modal;
pub mod header;
pub mod layout;
pub mod report_options;
pub mod sidebar;
pub mod progressive_search;

pub trait SearchableItem: Clone + PartialEq + 'static {
    fn id(&self) -> Uuid;
    fn display_label(&self) -> String;
    fn subtitle(&self) -> Option<String> { None } // Optional: For item codes or business details
}

