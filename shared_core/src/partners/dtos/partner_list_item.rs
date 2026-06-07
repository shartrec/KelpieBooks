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
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PartnerListItem {
    pub id: Uuid,
    pub legal_name: String,
    pub trade_name: Option<String>,
    pub is_vendor: bool,
    pub is_customer: bool,
    pub can_delete: bool,
}
