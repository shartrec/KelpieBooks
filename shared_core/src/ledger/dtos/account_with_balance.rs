/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use rust_decimal::{
    dec,
    Decimal,
};
use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

use crate::ledger::models::{
    account_category::AccountCategory,
    system_tag::SystemTag,
};
use crate::OrgId;

/// A DTO that combines account data with its calculated balance.
/// This is the structure that will be sent to the frontend.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountWithBalance {
    pub id: Uuid,
    pub organization_id: OrgId,
    pub parent_id: Option<Uuid>,
    pub code: String,
    pub name: String,
    pub category: AccountCategory,
    pub is_group: bool,
    pub is_bank_account: bool,
    pub system_tag: Option<SystemTag>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub balance: Decimal,
}

impl AccountWithBalance {
    pub fn calculate_totals(accounts: &[Self]) -> (Decimal, Decimal) {
        let mut debit_sum = dec!(0.00);
        let mut credit_sum = dec!(0.00);

        for acc in accounts.iter() {
            if !acc.is_group {
                match acc.category {
                    AccountCategory::Asset | AccountCategory::Expense => {
                        if acc.balance >= dec!(0.00) {
                            debit_sum += acc.balance;
                        } else {
                            credit_sum += acc.balance.abs();
                        }
                    }
                    AccountCategory::Liability
                    | AccountCategory::Equity
                    | AccountCategory::Revenue => {
                        if acc.balance <= dec!(0.00) {
                            credit_sum += acc.balance.abs();
                        } else {
                            debit_sum += acc.balance;
                        }
                    }
                }
            }
        }
        (debit_sum, credit_sum)
    }
}
