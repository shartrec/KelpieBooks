/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use uuid::Uuid;
use shared_core::partners::dtos::partner_list_item::PartnerListItem;
use shared_core::sales::models::item::Item;
use crate::core::components::SearchableItem;

impl SearchableItem for Item {
    fn id(&self) -> Uuid { self.id }
    fn display_label(&self) -> String { self.name.clone() }
    fn subtitle(&self) -> Option<String> { Some(self.code.clone()) }
}

impl SearchableItem for PartnerListItem {
    fn id(&self) -> Uuid {
        self.id
    }

    fn display_label(&self) -> String {
        self.trade_name.clone().unwrap_or(self.legal_name.clone())
    }
}