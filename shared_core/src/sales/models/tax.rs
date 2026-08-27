/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;
use crate::OrgId;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct TaxCategory {
    pub id: Uuid,
    #[cfg_attr(feature = "backend", sqlx(rename = "organization_id"))]
    pub org_id: OrgId,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct TaxRate {
    pub id: Uuid,
    #[cfg_attr(feature = "backend", sqlx(rename = "organization_id"))]
    pub org_id: OrgId,
    pub tax_category_id: Uuid,
    pub name: String,
    pub rate: Decimal,
    pub liability_account_id: Uuid,
    pub valid_from: NaiveDate,
    pub valid_to: Option<NaiveDate>,
}

impl Default for TaxRate {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            org_id: OrgId::default(),
            tax_category_id: Uuid::nil(),
            name: String::new(),
            rate: Decimal::new(0, 4),
            liability_account_id: Uuid::nil(),
            valid_from: chrono::Local::now().naive_local().date(),
            valid_to: None,
        }
    }
}
