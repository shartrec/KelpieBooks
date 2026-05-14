use yew::prelude::*;
use crate::components::layout::Layout;
use crate::services::dashboard::{get_financial_health, get_recent_transactions, get_expense_breakdown};
use shared_core::dtos::dashboard::FinancialHealth;
use shared_core::dtos::recent_transaction::RecentTransaction;
use shared_core::dtos::expense_breakdown::ExpenseBreakdown;
use yew_router::prelude::*;
use crate::router::Route;
use crate::contexts::org_context::use_org_context;
use shared_core::util::format_currency;
use crate::contexts::auth_context::use_user_context;

#[function_component(DashboardPage)]
pub fn dashboard_page() -> Html {
    let user_ctx = use_user_context();
    let org_context = use_org_context();
    let navigator = use_navigator().unwrap();

    let financial_health_state = use_state(|| None::<FinancialHealth>);
    let recent_transactions_state = use_state(|| None::<Vec<RecentTransaction>>);
    let expense_breakdown_state = use_state(|| None::<Vec<ExpenseBreakdown>>);
    let error_state = use_state(|| None::<String>);

    {
        let financial_health_state = financial_health_state.clone();
        let recent_transactions_state = recent_transactions_state.clone();
        let expense_breakdown_state = expense_breakdown_state.clone();
        let error_state = error_state.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();

        use_effect_with((), move |_| {
            let error_state1 = error_state.clone();
            let user_ctx1 = user_ctx.clone();
            let navigator1 = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match get_financial_health(user_ctx1, navigator1).await {
                    Ok(data) => financial_health_state.set(Some(data)),
                    Err(e) => error_state1.set(Some(e.clone())),
                }
            });
            let error_state2 = error_state.clone();
            let user_ctx2 = user_ctx.clone();
            let navigator2 = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match get_recent_transactions(user_ctx2, navigator2).await {
                    Ok(data) => recent_transactions_state.set(Some(data)),
                    Err(e) => error_state2.set(Some(e.clone())),
                }
            });
            let error_state3 = error_state.clone();
            let user_ctx3 = user_ctx.clone();
            let navigator3 = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match get_expense_breakdown(user_ctx3, navigator3).await {
                    Ok(data) => expense_breakdown_state.set(Some(data)),
                    Err(e) => error_state3.set(Some(e)),
                }
            });
            || ()
        });
    }

    html! {
        <Layout>
            <div class="dashboard-container">
                <header class="dashboard-header-flex">
                    <h1>{ "Dashboard" }</h1>
                    if let Some(lock_date) = org_context.locked_until {
                        <span class="period-badge">
                            { format!("🔒 Period Locked Until: {}", lock_date.format("%d %b %Y")) }
                        </span>
                    } else {
                        <span class="period-badge warning">{ "🔓 Period Open" }</span>
                    }
                </header>

                <section class="dashboard-grid">
                    if let Some(health) = &*financial_health_state {
                        <FinancialCard title="Net Profit (YTD)" value={&health.net_profit_ytd} />
                        <FinancialCard title="Operating Bank" value={&health.bank_balance} />
                        <FinancialCard title="Receivables" value={&health.accounts_receivable} />
                        <FinancialCard title="Payables" value={&health.accounts_payable} />
                    }
                </section>

                <section class="card shadow-sm p-4">
                    <h3>{ "Recent Activity" }</h3>
                    <table class="audit-table">
                        <thead>
                            <tr>
                                <th>{ "Date" }</th>
                                <th>{ "Description" }</th>
                                <th style="text-align: right">{ "Amount" }</th>
                            </tr>
                        </thead>
                        <tbody>
                            if let Some(transactions) = &*recent_transactions_state {
                                { for transactions.iter().map(|tx| html! {
                                    <tr>
                                        <td>
                                            <Link<Route>
                                                to={Route::AccountLedger { id: tx.account_id }}
                                                classes={classes!("account-link")}
                                            >
                                                {tx.date.format("%d %b %Y").to_string() }
                                            </Link<Route>>
                                        </td>
                                        <td>{ tx.description.clone() }</td>
                                        <td class="stat-value-small">{ format_currency(&tx.amount) }</td>
                                    </tr>
                                })}
                            }
                        </tbody>
                    </table>
                </section>
            </div>
        </Layout>
    }
}

#[derive(Properties, PartialEq)]
struct FinancialCardProps {
    title: AttrValue,
    value: i64,
}

#[function_component(FinancialCard)]
    fn financial_card(props: &FinancialCardProps) -> Html {
        html! {
        <div class="stat-card">
            <span class="stat-label">{ &props.title }</span>
            <div class="stat-value">
                { format_currency(&props.value) }
            </div>
        </div>
    }
}
