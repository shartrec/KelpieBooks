/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::api::Api;
use crate::contexts::auth_context::{UserContext, UserContextHandle};
use shared_core::dtos::dashboard::FinancialHealth;
use shared_core::dtos::expense_breakdown::ExpenseBreakdown;
use shared_core::dtos::recent_transaction::RecentTransaction;
use shared_core::dtos::top_payable::TopPayable;
use yew_router::prelude::Navigator;

pub async fn get_financial_health(
    user_ctx: UserContextHandle,
    navigator: Navigator,
) -> Result<FinancialHealth, String> {
    let response = Api::get("/api/dashboard/financial-health", user_ctx, navigator).await;
    match response {
        Ok(response) => {
            if response.ok() {
                response.json::<FinancialHealth>().await.map_err(|e| e.to_string())
            } else {
                Err(format!("Failed to fetch financial health: {}", response.status()))
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn get_recent_transactions(
    user_ctx: UserContextHandle,
    navigator: Navigator,
) -> Result<Vec<RecentTransaction>, String> {
    let response = Api::get("/api/dashboard/recent-transactions", user_ctx, navigator).await;
    match response {
        Ok(response) => {
            if response.ok() {
                response.json::<Vec<RecentTransaction>>().await.map_err(|e| e.to_string())
            } else {
                Err(format!("Failed to fetch recent transactions: {}", response.status()))
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn get_expense_breakdown(
    user_ctx: UserContextHandle,
    navigator: Navigator,
) -> Result<Vec<ExpenseBreakdown>, String> {
    let response = Api::get("/api/dashboard/expense-breakdown", user_ctx, navigator).await;
    match response {
        Ok(response) => {
            if response.ok() {
                response.json::<Vec<ExpenseBreakdown>>().await.map_err(|e| e.to_string())
            } else {
                Err(format!("Failed to fetch expense breakdown: {}", response.status()))
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn get_top_payables(
    user_ctx: UserContextHandle,
    navigator: Navigator,
) -> Result<Vec<TopPayable>, String> {
    let response = Api::get("/api/dashboard/top-payables", user_ctx, navigator).await;
    match response {
        Ok(response) => {
            if response.ok() {
                response.json::<Vec<TopPayable>>().await.map_err(|e| e.to_string())
            } else {
                Err(format!("Failed to fetch top payables: {}", response.status()))
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
