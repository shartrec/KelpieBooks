/*
 * Copyright (c) 2026-2026. Trevor Campbell and others.
 *
 * This file is part of KelpieBooks.
 *
 * KelpieBooks is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License,or
 * (at your option) any later version.
 *
 * KelpieBooks is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with KelpieBooks; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 *
 * Contributors:
 *      Trevor Campbell
 *
 */

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::dtos::vendor_invoice_list_item::VendorInvoiceListItem;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgedPayableSummary {
    pub partner_id: Uuid,
    pub partner_name: String,
    pub current: i64,
    pub days_30: i64,
    pub days_60: i64,
    pub days_90: i64,
    pub days_90_plus: i64,
    pub total: i64,
    pub invoices: Vec<VendorInvoiceListItem>,
}
