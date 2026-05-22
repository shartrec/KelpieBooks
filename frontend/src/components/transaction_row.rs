use crate::contexts::org_context::OrgContextHandle;
use crate::pages::new_transaction::NewTransactionQuery;
use crate::router::Route;
use gloo_net::http::Request;
use shared_core::dtos::journal_entry_with_balance::JournalEntryWithBalance;
use shared_core::dtos::transaction_detail::TransactionDetail;
use shared_core::util::format_currency;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

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
    pub on_edit: Callback<Uuid>,
    pub on_delete: Callback<JournalEntryWithBalance>,
    pub org_ctx: OrgContextHandle,
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

    let on_edit_click = {
        let on_edit = props.on_edit.clone();
        let transaction_id = props.transaction_group.transaction_id;
        let dropdown_open = dropdown_open.clone();
        Callback::from(move |_| {
            dropdown_open.set(false);
            on_edit.emit(transaction_id);
        })
    };

    let on_delete_click = {
        let on_delete = props.on_delete.clone();
        let transaction_detail = props.transaction_group.primary_entry.clone();
        let dropdown_open = dropdown_open.clone();
        Callback::from(move |_| {
            dropdown_open.set(false);
            on_delete.emit(transaction_detail.clone());
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

    let strict_audit_mode = props.org_ctx.strict_audit_mode;
    let is_locked = props
        .org_ctx
        .locked_until
        .map_or(false, |lock_date| primary_entry.date <= lock_date);

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
                <td class="table__text-col">{ props.transaction_group.description.clone().unwrap_or_default() }</td>
                <td class="table__value-col">{ format_currency(&primary_entry.debit) }</td>
                <td class="table__value-col">{ format_currency(&primary_entry.credit) }</td>
                <td class="table__value-col">{ format_currency(&primary_entry.running_balance) }</td>
                <td class="actions-cell">
                    <div class="actions-dropdown">
                        <button class="icon-button" onclick={on_toggle_dropdown} title="Actions">
                            <img src="/images/more-vertical.svg" alt="Actions" class="dropdown-trigger-icon" />
                        </button>
                        if *dropdown_open {
                            <div class="actions-dropdown__content">
                                if strict_audit_mode {
                                    <button class="dropdown-item" onclick={on_reverse_click} disabled={is_locked}>
                                        <img src="/images/reverse.svg" alt="Reverse" />
                                        <span>{ "Reverse" }</span>
                                    </button>
                                } else {
                                    <button class="dropdown-item" onclick={on_edit_click} disabled={is_locked}>
                                        <img src="/images/edit.svg" alt="Edit" />
                                        <span>{ "Edit" }</span>
                                    </button>
                                    <button class="dropdown-item" onclick={on_delete_click} disabled={is_locked}>
                                        <img src="/images/delete.svg" alt="Delete" />
                                        <span>{ "Delete" }</span>
                                    </button>
                                }
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
                <tr class="transaction-detail__row">
                    <td colspan="6">
                        <div class="transaction-detail__content">
                            if *loading_details {
                                <p>{ "Loading details..." }</p>
                            } else if let Some(detail) = &*transaction_detail {
                                <div class="journal-entry__header">
                                    <span class="table__text-col">{ "Details for trans" }</span>
                                    <span class="table__text-col">{ &detail.transaction.id.to_string()[0..8] }</span>
                                    <span class="table__value-col">{ "Debit" }</span>
                                    <span class="table__value-col">{ "Credit" }</span>
                                </div>
                                { for detail.entries.iter().map(|entry| html! {
                                    <div class="journal-entry__line">
                                        <span class="table__text-col">
                                            <Link<Route>
                                                to={Route::AccountLedger { id: entry.account_id }}
                                                classes={classes!("account-link")}
                                            >
                                                { &entry.account_name }
                                            </Link<Route>>
                                        </span>
                                        <span class="table__text-col">{ entry.description.clone().unwrap_or_default() }</span>
                                        <span class="table__value-col">{ format_currency(&entry.debit) }</span>
                                        <span class="table__value-col">{ format_currency(&entry.credit) }</span>
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
