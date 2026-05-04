use crate::Route;
use crate::pages::new_transaction::NewTransactionQuery;
use gloo_net::http::Request;
use shared_core::dtos::journal_entry_with_balance::JournalEntryWithBalance;
use shared_core::dtos::transaction_detail::TransactionDetail;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;
use shared_core::util::format_currency;

#[derive(Clone, Debug, PartialEq)]
pub struct TransactionGroup {
    pub transaction_id: Uuid,
    pub date: chrono::NaiveDate,
    pub description: Option<String>,
    pub primary_entry: JournalEntryWithBalance,
}

#[derive(Properties, PartialEq)]
pub struct TransactionRowProps {
    pub transaction_group: TransactionGroup,
    pub on_reverse: Callback<JournalEntryWithBalance>,
}

#[function_component(TransactionRow)]
pub fn transaction_row(props: &TransactionRowProps) -> Html {
    let expanded = use_state(|| false);
    let transaction_detail = use_state(|| None::<TransactionDetail>);
    let loading_details = use_state(|| false);
    let dropdown_open = use_state(|| false);

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

    let on_reverse_click = {
        let on_reverse = props.on_reverse.clone();
        let transaction_detail = props.transaction_group.primary_entry.clone();
        let dropdown_open = dropdown_open.clone();
        Callback::from(move |_| {
            dropdown_open.set(false);
            on_reverse.emit(transaction_detail.clone());
        })
    };

    let on_toggle_dropdown = {
        let dropdown_open = dropdown_open.clone();
        Callback::from(move |_| {
            dropdown_open.set(!*dropdown_open);
        })
    };

    let primary_entry = &props.transaction_group.primary_entry;
    let duplicate_query = NewTransactionQuery {
        duplicate_from: Some(props.transaction_group.transaction_id),
        ..Default::default()
    };

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
                <td class="amount">{ format_currency(&primary_entry.debit) }</td>
                <td class="amount">{ format_currency(&primary_entry.credit) }</td>
                <td class="amount">{ format_currency(&primary_entry.running_balance) }</td>
                <td class="actions-cell">
                    <div class="actions-dropdown">
                        <button class="icon-button" onclick={on_toggle_dropdown} title="Actions">
                            <img src="/images/more-vertical.svg" alt="Actions" />
                        </button>
                        if *dropdown_open {
                            <div class="actions-dropdown-content">
                                <button class="dropdown-item" onclick={on_reverse_click}>
                                    <img src="/images/reverse.svg" alt="Reverse" />
                                    <span>{ "Reverse" }</span>
                                </button>
                                <Link<Route, NewTransactionQuery>
                                    to={Route::NewTransaction}
                                    query={duplicate_query}
                                    classes="dropdown-item"
                                >
                                    <img src="/images/edit.svg" alt="Duplicate" />
                                    <span>{ "Duplicate" }</span>
                                </Link<Route, NewTransactionQuery>>
                            </div>
                        }
                    </div>
                </td>
            </tr>
            if *expanded {
                <tr class="transaction-detail-row">
                    <td colspan="6">
                        <div class="transaction-detail-content">
                            if *loading_details {
                                <p>{ "Loading details..." }</p>
                            } else if let Some(detail) = &*transaction_detail {
                                <div class="journal-entry-header">
                                    <span>{ "Details for trans" }</span>
                                    <span>{ &detail.transaction.id.to_string()[0..8] }</span>
                                    <span class="amount">{ "Debit" }</span>
                                    <span class="amount">{ "Credit" }</span>
                                </div>
                                { for detail.entries.iter().map(|entry| html! {
                                    <div class="journal-entry-line">
                                        <span>{ &entry.account_name }</span>
                                        <span>{ entry.description.clone().unwrap_or_default() }</span>
                                        <span class="amount">{ format_currency(&entry.debit) }</span>
                                        <span class="amount">{ format_currency(&entry.credit) }</span>
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
