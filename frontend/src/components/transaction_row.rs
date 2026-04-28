use yew::prelude::*;
use shared_core::dtos::journal_entry_with_balance::JournalEntryWithBalance;
use shared_core::dtos::transaction_detail::TransactionDetail;
use gloo_net::http::Request;

#[derive(Clone, Debug, PartialEq)]
pub struct TransactionGroup {
    pub transaction_id: uuid::Uuid,
    pub date: chrono::NaiveDate,
    pub description: Option<String>,
    pub primary_entry: JournalEntryWithBalance,
}

#[derive(Properties, PartialEq)]
pub struct TransactionRowProps {
    pub transaction_group: TransactionGroup,
}

#[function_component(TransactionRow)]
pub fn transaction_row(props: &TransactionRowProps) -> Html {
    let expanded = use_state(|| false);
    let transaction_detail = use_state(|| None::<TransactionDetail>);
    let loading_details = use_state(|| false);

    let on_toggle_expand = {
        let expanded = expanded.clone();
        let transaction_detail = transaction_detail.clone();
        let loading_details = loading_details.clone();
        let transaction_id = props.transaction_group.transaction_id;

        Callback::from(move |_| {
            let expanded = expanded.clone();
            let transaction_detail = transaction_detail.clone();
            let loading_details = loading_details.clone();

            if !*expanded && transaction_detail.is_none() {
                loading_details.set(true);
                wasm_bindgen_futures::spawn_local(async move {
                    let url = format!("/api/transactions/{}", transaction_id);
                    if let Ok(response) = Request::get(&url).send().await {
                        if let Ok(data) = response.json::<TransactionDetail>().await {
                            transaction_detail.set(Some(data));
                        }
                    }
                    loading_details.set(false);
                });
            }
            expanded.set(!*expanded);
        })
    };

    let primary_entry = &props.transaction_group.primary_entry;

    html! {
        <>
            <tr class="transaction-summary-row">
                <td>
                    <button onclick={on_toggle_expand} class="collapse-toggle">
                        if *expanded {
                            <img src="/images/chevron-down.svg" alt="Collapse" />
                        } else {
                            <img src="/images/chevron-right.svg" alt="Expand" />
                        }
                    </button>
                    { primary_entry.date.to_string() }
                </td>
                <td>{ props.transaction_group.description.clone().unwrap_or_default() }</td>
                <td class="amount">{ format!("{:.2}", (primary_entry.debit as f64) / 100.0) }</td>
                <td class="amount">{ format!("{:.2}", (primary_entry.credit as f64) / 100.0) }</td>
                <td class="amount">{ format!("{:.2}", (primary_entry.running_balance as f64) / 100.0) }</td>
            </tr>
            if *expanded {
                <tr class="transaction-detail-row">
                    <td colspan="5">
                        <div class="transaction-detail-content">
                            if *loading_details {
                                <p>{ "Loading details..." }</p>
                            } else if let Some(detail) = &*transaction_detail {
                                <div class="journal-entry-header">
                                    <span>{ "Account" }</span>
                                    <span>{ "Description" }</span>
                                    <span class="amount">{ "Debit" }</span>
                                    <span class="amount">{ "Credit" }</span>
                                </div>
                                { for detail.entries.iter().map(|entry| html! {
                                    <div class="journal-entry-line">
                                        <span>{ &entry.account_name }</span>
                                        <span>{ entry.description.clone().unwrap_or_default() }</span>
                                        <span class="amount">{ format!("{:.2}", (entry.debit as f64) / 100.0) }</span>
                                        <span class="amount">{ format!("{:.2}", (entry.credit as f64) / 100.0) }</span>
                                    </div>
                                })}
                            } else {
                                <p class="error">{ "Could not load transaction details." }</p>
                            }
                        </div>
                    </td>
                </tr>
            }
        </>
    }
}
