use gloo_net::http::Request;
use shared_core::dtos::dashboard::FinancialHealth;
use shared_core::dtos::recent_transaction::RecentTransaction;
use shared_core::dtos::expense_breakdown::ExpenseBreakdown;
use crate::services::handle_response;
use yew_router::prelude::Navigator;
use crate::contexts::auth_context::UserContextHandle;

pub async fn get_financial_health(user_ctx: UserContextHandle, navigator: Navigator) -> Result<FinancialHealth, String> {
    let response = Request::get("/api/dashboard/financial-health").send().await;
    handle_response(response, user_ctx, navigator).await
}

pub async fn get_recent_transactions(user_ctx: UserContextHandle, navigator: Navigator) -> Result<Vec<RecentTransaction>, String> {
    let response = Request::get("/api/dashboard/recent-transactions").send().await;
    handle_response(response, user_ctx, navigator).await
}

pub async fn get_expense_breakdown(user_ctx: UserContextHandle, navigator: Navigator) -> Result<Vec<ExpenseBreakdown>, String> {
    let response = Request::get("/api/dashboard/expense-breakdown").send().await;
    handle_response(response, user_ctx, navigator).await
}
