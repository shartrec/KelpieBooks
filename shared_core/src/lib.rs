/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

#[cfg(feature = "inventory")]
pub mod inventory;
#[cfg(feature = "ledger")]
pub mod ledger;
#[cfg(feature = "partners")]
pub mod partners;
#[cfg(feature = "payables")]
pub mod payables;
#[cfg(feature = "sales")]
pub mod sales;

pub mod core;
pub mod i18n;
pub mod util;


macro_rules! define_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
        #[cfg_attr(feature = "backend", derive(sqlx::Type))]
        #[cfg_attr(feature = "backend", sqlx(transparent))]
        pub struct $name(pub uuid::Uuid);

        impl From<uuid::Uuid> for $name {
            fn from(id: uuid::Uuid) -> Self {
                Self(id)
            }
        }

        impl std::ops::Deref for $name {
            type Target = uuid::Uuid;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl Default for $name {
            Self(Uuid::new_v4())
        }
    };
}

// Instantiate all entity keys in 1 line each:
// These are all just well typed Uuids. Some, e.g. addresses may be reused in different contexts
// referring to different database tables.

define_id!(OrgId);
define_id!(RoleId);
define_id!(UserId);

define_id!(AccountId);
define_id!(AddressId);
define_id!(AllocationId);
define_id!(ContactId);
define_id!(InvoiceOrderId);
define_id!(ItemId);
define_id!(JournalEntryId);
define_id!(LocationEntryId);
define_id!(OrderId);
define_id!(PartnerId);
define_id!(PaymentId);
define_id!(TaxCategoryId);
define_id!(TaxRateId);
define_id!(TransactionId);
define_id!(WarehouseId);
define_id!(UomId);
