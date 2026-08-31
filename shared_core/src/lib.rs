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

/// Generates a strongly-typed wrapper around a [`uuid::Uuid`] (Newtype pattern).
///
/// This macro creates a type-safe identifier to prevent parameter-ordering bugs
/// (e.g., accidentally passing an `OrderId` into an `OrgId` parameter) at compile time,
/// while maintaining zero runtime overhead.
///
/// # Derived Traits & Features
/// Each generated type automatically implements:
/// - Common identity/comparison traits: `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`, `Ord`, `PartialOrd`.
/// - `Serialize` and `Deserialize` (via `serde`).
/// - `sqlx::Type` with `#[sqlx(transparent)]` when the `backend` feature is active, allowing
///   the type to bind directly to PostgreSQL `UUID` database columns.
/// - `Deref<Target = Uuid>` for seamless access to underlying [`uuid::Uuid`] methods.
/// - `From<Uuid>` and `From<$name> for Uuid` for easy conversion.
/// - [`Display`](std::fmt::Display) for direct string formatting.
///
/// # Examples
///
/// Defining a new entity identifier:
/// ```rust
/// use shared_core::define_id;
/// define_id!(OrgId);
/// define_id!(OrderId);
///
/// let org_id = OrgId::from(uuid::Uuid::new_v4());
/// let order_id = OrderId::from(uuid::Uuid::new_v4());
///
/// // Transparently dereferences to Uuid:
/// println!("Organization UUID string: {}", org_id.to_string());
///
/// // Type safety in function signatures:
/// fn process_order(org_id: OrgId, order_id: OrderId) {
///     // ...
/// }
///
/// // process_order(order_id, org_id);
/// // ❌ Fails at compile-time: expected `OrgId`, found `OrderId`
/// ```
#[macro_export]
macro_rules! define_id {
    ($name:ident) => {
        #[doc = concat!("A strongly-typed wrapper around [`uuid::Uuid`] representing a unique `", stringify!($name), "` identifier.")]
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
            fn default() -> Self {
                Self(uuid::Uuid::default())
            }
        }

        impl std::str::FromStr for $name {
            type Err = ::uuid::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                ::uuid::Uuid::parse_str(s).map(Self)
            }
        }

        #[cfg(feature = "backend")]
        impl<'r> rocket::request::FromParam<'r> for $name {
            type Error = uuid::Error;

            fn from_param(param: &'r str) -> Result<Self, Self::Error> {
                uuid::Uuid::parse_str(param).map(Self)
            }
        }

        #[cfg(feature = "backend")]
        impl<'r> rocket::form::FromFormField<'r> for $name {
            fn from_value(field: rocket::form::ValueField<'r>) -> rocket::form::Result<'r, Self> {
                match uuid::Uuid::parse_str(field.value) {
                    Ok(uuid) => Ok(Self(uuid)),
                    Err(e) => Err(rocket::form::Error::validation(format!("{}", e)).into()),
                }
            }
        }

    };
}

// Instantiate all entity keys in 1 line each:
// These are all just well typed Uuids. Some, e.g. addresses may be reused in different contexts
// referring to different database tables.
// They all implement Deref, so use *org_id for instance would be a direct reference to the Uuid.

define_id!(OrgId);
define_id!(RoleId);
define_id!(UserId);

define_id!(AccountId);
define_id!(AddressId);
define_id!(AllocationId);
define_id!(ContactId);
define_id!(InvoiceId);
define_id!(InvoiceItemId);
define_id!(ItemId);
define_id!(JournalEntryId);
define_id!(LocationEntryId);
define_id!(OrderId);
define_id!(OrderItemId);
define_id!(PartnerId);
define_id!(PaymentId);
define_id!(TaxCategoryId);
define_id!(TaxRateId);
define_id!(TransactionId);
define_id!(WarehouseId);
define_id!(WarehouseBalId);
define_id!(UomId);
