use gloo_net::http::Request;
use shared_core::dtos::dashboard::FinancialHealth;
use shared_core::dtos::recent_transaction::RecentTransaction;
use shared_core::dtos::expense_breakdown::ExpenseBreakdown;
use crate::services::handle_response;

pub async fn get_financial_health() -> Result<FinancialHealth, String> {
    let response = Request::get("/api/dashboard/financial-health").send().await;
    handle_response(response).await
}

pub async fn get_recent_transactions() -> Result<Vec<RecentTransaction>, String> {
    let response = Request::get("/api/dashboard/recent-transactions").send().await;
    handle_response(response).await
}

pub async fn get_expense_breakdown() -> Result<Vec<ExpenseBreakdown>, String> {
    let response = Request::get("/api/dashboard/expense-breakdown").send().await;
    handle_response(response).await
}
