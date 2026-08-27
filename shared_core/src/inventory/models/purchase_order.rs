/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use chrono::{
    DateTime,
    NaiveDate,
    Utc,
};
use rust_decimal::Decimal;
use serde::{
    Deserialize,
    Serialize,
};
use strum::{
    Display,
    EnumString,
};
use uuid::Uuid;
use crate::OrgId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, Display, Copy)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(
    feature = "backend",
    sqlx(type_name = "purchase_order_status", rename_all = "snake_case")
)]
pub enum PurchaseOrderStatus {
    Draft,
    Approved,
    Sent,
    PartiallyReceived,
    Received,
    Cancelled,
}

impl Default for PurchaseOrderStatus {
    fn default() -> Self {
        PurchaseOrderStatus::Draft
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct PurchaseOrder {
    pub id: Uuid,
    #[cfg_attr(feature = "backend", sqlx(rename = "organization_id"))]
    pub org_id: OrgId,
    pub vendor_id: Uuid, // References partner
    pub destination_warehouse_id: Uuid,
    pub po_number: String,
    pub status: PurchaseOrderStatus,
    pub order_date: NaiveDate,
    pub expected_delivery_date: Option<NaiveDate>,
    pub notes: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct PurchaseOrderLine {
    pub id: Uuid,
    #[cfg_attr(feature = "backend", sqlx(rename = "organization_id"))]
    pub org_id: OrgId,
    pub purchase_order_id: Uuid,
    pub item_id: Uuid,
    pub description: Option<String>,
    pub quantity_ordered: Decimal,
    pub quantity_received: Decimal,
    pub unit_cost: Decimal, // Scaled to 4 decimal places
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct InventoryReceiptLog {
    pub id: Uuid,
    #[cfg_attr(feature = "backend", sqlx(rename = "organization_id"))]
    pub org_id: OrgId,
    pub purchase_order_line_id: Uuid,
    pub received_at_location_id: Uuid,
    pub quantity_received: Decimal,
    pub received_date: NaiveDate,
    pub received_by_user_id: Option<Uuid>,
    pub created_at: Option<DateTime<Utc>>,
}
