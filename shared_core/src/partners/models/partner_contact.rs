/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::{
    DateTime,
    Utc,
};
use serde::{
    Deserialize,
    Serialize,
};
use crate::{ContactId, OrgId, PartnerId};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PartnerContact {
    pub id: ContactId,
    pub organization_id: OrgId,
    pub partner_id: PartnerId,
    pub is_primary: bool,
    pub full_name: String,
    pub preferred_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub role_title: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
