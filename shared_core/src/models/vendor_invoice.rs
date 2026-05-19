/*
 * Copyright (c) 2026. Trevor Campbell and others.
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

use crate::models::invoice_status::InvoiceStatus;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VendorInvoice {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub partner_id: Uuid,
    pub transaction_id: Option<Uuid>,
    pub invoice_number: String,
    pub status: InvoiceStatus,
    pub issue_date: NaiveDate,
    pub due_date: NaiveDate,
    pub amount_due: i64,
    pub amount_remaining: i64,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl VendorInvoice {
    pub fn is_overdue(&self, today: NaiveDate) -> bool {
        self.status != InvoiceStatus::Paid
            && self.status != InvoiceStatus::Void
            && self.due_date < today
    }
}
