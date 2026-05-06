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

use crate::db;
use crate::util::ApiError;
use chrono::NaiveDate;
use rocket_db_pools::sqlx::PgConnection;
use shared_core::models::AccountCategory;
use uuid::Uuid;

pub async fn close_financial_year(
    pool: &mut PgConnection,
    organization_id: Uuid,
    year_end: NaiveDate,
) -> Result<(), ApiError> {
    // 1. Calculate Net Income
    let revenue_balance = db::account::get_balance_for_category(pool, organization_id, AccountCategory::Revenue, year_end).await?;
    let expense_balance = db::account::get_balance_for_category(pool, organization_id, AccountCategory::Expense, year_end).await?;
    let net_income = revenue_balance + expense_balance; // Revenue is credit (negative), expense is debit (positive)

    // 2. Create the Closing Journal Transaction
    let closing_tx = db::transaction::insert(pool, organization_id, year_end, Some("Closing Entry".to_string()), None).await?;

    // 3. Create Journal Entries for all Revenue and Expense accounts
    let income_accounts = db::account::get_all_by_category(pool, organization_id, vec![AccountCategory::Revenue, AccountCategory::Expense]).await?;
    for account in income_accounts {
        let balance = db::journal_entry::get_balance_up_to_date(pool, account.id, year_end).await?;
        if balance != 0 {
            let (debit, credit) = if balance > 0 { (0, balance) } else { (-balance, 0) };
            db::journal_entry::insert(pool, closing_tx, account.id, debit, credit, Some("Closing Entry".to_string())).await?;
        }
    }

    // 4. Post the Net Income to Retained Earnings
    let retained_earnings_account = db::account::get_by_system_tag(pool, organization_id, "RetainedEarnings").await?
        .ok_or_else(|| ApiError::NotFound("Retained Earnings account not found".to_string()))?;

    let (debit, credit) = if net_income > 0 { (net_income, 0) } else { (0, -net_income) };
    db::journal_entry::insert(pool, closing_tx, retained_earnings_account.id, debit, credit, Some("Closing Net Income".to_string())).await?;

    // 5. Update Organization Record to lock the period
    db::organization::set_locked_until(pool, organization_id, year_end).await?;

    Ok(())
}
