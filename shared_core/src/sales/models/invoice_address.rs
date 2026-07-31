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
use strum::Display;

#[derive(Clone, Debug, Display, PartialEq)]
pub enum AddressType {
    Billing,
    Shipping,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct InvoiceAddress {
    pub name: Option<String>,
    pub attention: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub state_province: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}
