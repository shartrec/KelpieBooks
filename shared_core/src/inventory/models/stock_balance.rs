/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 * (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use chrono::{
    DateTime,
    Utc,
};
use rust_decimal::Decimal;
use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

use crate::OrgId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(
    feature = "backend",
    sqlx(type_name = "stock_transaction_type", rename_all = "snake_case")
)]
pub enum TransactionType {
    Receipt,
    Adjustment,
    Allocation,
    Pick,
    Shipment,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(
    feature = "backend",
    sqlx(type_name = "reference_type", rename_all = "snake_case")
)]
pub enum ReferenceType {
    PurchaseOrder,
    SalesOrder,
    ManualAdjustment,
    CycleCount,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockTransaction {
    pub id: Uuid,
    pub organization_id: OrgId,
    pub warehouse_id: Uuid,
    pub location_id: Uuid,
    pub item_id: Uuid,
    pub transaction_type: TransactionType,
    pub quantity_change: Decimal,
    pub reference_type: Option<ReferenceType>,
    pub reference_id: Option<Uuid>,
    pub notes: Option<String>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
}
