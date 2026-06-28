/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use std::{
    ops::Deref,
    str::FromStr,
};

use chrono::NaiveDate;
use rocket::{
    form::{
        self,
        FromFormField,
        ValueField,
    },
    request::FromParam,
};
use shared_core::{
    payables::models::invoice_status::InvoiceStatus,
    sales::models::{
        invoice_status::InvoiceStatus as SalesInvoiceStatus,
        item::ItemType,
    },
};
use uuid::Uuid;

/// A newtype wrapper for `Uuid` to implement `FromParam` and satisfy the orphan rule.
/// This allows Rocket to parse `Uuid` values from URL path segments.
#[derive(Clone, Copy)]
pub(crate) struct PathUuid(pub(crate) Uuid);

/// Allows `PathUuid` to be used as a `Uuid` via dereferencing (e.g., `*id`).
impl Deref for PathUuid {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Synchronous implementation of `FromParam` for our `PathUuid` newtype.
impl<'r> FromParam<'r> for PathUuid {
    type Error = uuid::Error;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        Uuid::parse_str(param).map(PathUuid)
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for PathUuid {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        match Uuid::parse_str(field.value) {
            Ok(uuid) => Ok(PathUuid(uuid)),
            Err(e) => Err(form::Error::validation(format!("{}", e)).into()),
        }
    }
}

/// A newtype wrapper for `InvoiceStatus` to implement `FromFormField` and satisfy the orphan rule.
#[derive(Clone, Copy)]
pub(crate) struct FormInvoiceStatus(pub(crate) InvoiceStatus);

/// Allows `FormInvoiceStatus` to be used as a `InvoiceStatus` via dereferencing.
impl Deref for FormInvoiceStatus {
    type Target = InvoiceStatus;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for FormInvoiceStatus {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        // The `from_str` comes from `strum::EnumString` on the InvoiceStatus enum
        match InvoiceStatus::from_str(field.value) {
            Ok(status) => Ok(FormInvoiceStatus(status)),
            Err(_) => Err(form::Error::validation("Invalid invoice status.").into()),
        }
    }
}

/// A newtype wrapper for Sales `InvoiceStatus` to implement `FromFormField` and satisfy the orphan rule.
#[derive(Clone, Copy)]
pub(crate) struct FormSalesInvoiceStatus(pub(crate) SalesInvoiceStatus);

/// Allows `FormSalesInvoiceStatus` to be used as a `InvoiceStatus` via dereferencing.
impl Deref for FormSalesInvoiceStatus {
    type Target = SalesInvoiceStatus;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for FormSalesInvoiceStatus {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        // The `from_str` comes from `strum::EnumString` on the InvoiceStatus enum
        match SalesInvoiceStatus::from_str(field.value) {
            Ok(status) => Ok(FormSalesInvoiceStatus(status)),
            Err(_) => Err(form::Error::validation("Invalid invoice status.").into()),
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct FormItemType(pub(crate) ItemType);
/// Allows `FormItemType` to be used as a `ItemType` via dereferencing.
impl Deref for FormItemType {
    type Target = ItemType;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for FormItemType {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        // The `from_str` comes from `strum::EnumString` on the ItemType enum
        match ItemType::from_str(field.value) {
            Ok(status) => Ok(FormItemType(status)),
            Err(_) => Err(form::Error::validation("Invalid invoice status.").into()),
        }
    }
}

/// A newtype wrapper for `NaiveDate` to implement `FromParam` and satisfy the orphan rule.
/// This allows Rocket to parse `NaiveDate` values from URL path segments.
#[allow(unused)] // It isn't unused, but is only used by some things behind a feature flag for now
#[derive(Clone, Copy)]
pub(crate) struct PathDate(pub(crate) NaiveDate);

/// Allows `PathDate` to be used as a `NaiveDate` via dereferencing (e.g., `*date`).
impl Deref for PathDate {
    type Target = NaiveDate;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Synchronous implementation of `FromParam` for our `PathDate` newtype.
impl<'r> FromParam<'r> for PathDate {
    type Error = chrono::ParseError;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        NaiveDate::parse_from_str(param, "%Y-%m-%d").map(PathDate)
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for PathDate {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        match NaiveDate::parse_from_str(field.value, "%Y-%m-%d") {
            Ok(date) => Ok(PathDate(date)),
            Err(e) => Err(form::Error::validation(format!("{}", e)).into()),
        }
    }
}
