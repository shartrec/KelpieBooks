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

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum InvoiceStatus {
    Draft,
    Open,
    Paid,
    PartiallyPaid,
    Void,
}

impl Default for InvoiceStatus {
    fn default() -> Self {
        Self::Draft
    }
}

impl InvoiceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Draft => "Draft",
            Self::Open => "Open",
            Self::Paid => "Paid",
            Self::PartiallyPaid => "PartiallyPaid",
            Self::Void => "Void",
        }
    }
}

impl TryFrom<&str> for InvoiceStatus {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Draft" => Ok(Self::Draft),
            "Open" => Ok(Self::Open),
            "Paid" => Ok(Self::Paid),
            "PartiallyPaid" => Ok(Self::PartiallyPaid),
            "Void" => Ok(Self::Void),
            _ => Err(format!("Unknown invoice status: {}", value)),
        }
    }
}
