/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::partners::models::address_type::AddressType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PartnerAddress {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub partner_id: Uuid,
    pub address_type: AddressType,
    pub is_primary: bool,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub city: String,
    pub state_province: Option<String>,
    pub postal_code: Option<String>,
    pub country: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
