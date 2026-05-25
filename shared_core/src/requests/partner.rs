/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::models::partner_address::PartnerAddress;
use crate::models::partner_contact::PartnerContact;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreatePartnerRequest {
    pub legal_name: String,
    pub trade_name: Option<String>,
    pub tax_identifier: Option<String>,
    pub is_vendor: bool,
    pub is_customer: bool,
    pub default_ap_account_id: Option<Uuid>,
    pub default_ar_account_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdatePartnerRequest {
    pub legal_name: String,
    pub trade_name: Option<String>,
    pub tax_identifier: Option<String>,
    pub is_vendor: bool,
    pub is_customer: bool,
    pub default_ap_account_id: Option<Uuid>,
    pub default_ar_account_id: Option<Uuid>,
    pub addresses: Vec<PartnerAddress>,
    pub contacts: Vec<PartnerContact>,
}
