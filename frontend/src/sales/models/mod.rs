/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use shared_core::{partners::dtos::partner_list_item::PartnerListItem, sales::models::item::Item, ItemId, PartnerId};

use crate::core::components::SearchableItem;

impl SearchableItem for Item {
    type Id = ItemId;

    fn id(&self) -> ItemId {
        self.id
    }
    fn display_label(&self) -> String {
        self.name.clone()
    }
    fn subtitle(&self) -> Option<String> {
        Some(self.code.clone())
    }
}

impl SearchableItem for PartnerListItem {
    type Id = PartnerId;

    fn id(&self) -> PartnerId {
        self.id
    }

    fn display_label(&self) -> String {
        self.trade_name.clone().unwrap_or(self.legal_name.clone())
    }
}
