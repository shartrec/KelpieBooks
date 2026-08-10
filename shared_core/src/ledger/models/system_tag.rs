/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use serde::{
    Deserialize,
    Serialize,
};
use strum::{
    Display,
    EnumIter,
    EnumString,
    IntoEnumIterator,
};

#[derive(
    Debug, Display, EnumString, EnumIter, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Hash,
)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(
    feature = "backend",
    sqlx(type_name = "system_tag", rename_all = "snake_case")
)]
#[strum(serialize_all = "snake_case")]
pub enum SystemTag {
    CashAtBank,
    AccountsReceivable,
    AccountsPayable,
    RetainedEarnings,
    SalesTaxPayable,
    SalesTaxClearing,
    Revenue,
    Expense,
    CostOfGoodsSold,
    InventoryAsset,
    ReceivedNotInvoiced,
    InventoryAdjustment,
}

impl SystemTag {
    pub fn iterator() -> impl Iterator<Item = Self> {
        Self::iter()
    }

    /// Returns the Fluent translation key corresponding to the system tag.
    pub fn translation_key(&self) -> String {
        format!("system-tag-{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translation_key() {
        let tag = SystemTag::CashAtBank;
        assert_eq!(tag.translation_key(), "system-tag-cash_at_bank");
    }
}

