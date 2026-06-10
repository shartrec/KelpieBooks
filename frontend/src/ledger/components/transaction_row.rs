/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use gloo_net::http::Request;
use shared_core::ledger::dtos::{
    journal_entry_with_balance::JournalEntryWithBalance,
    transaction_detail::TransactionDetail,
};
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    contexts::{
        locale_context::use_locale,
        org_context::OrgContextHandle,
    },
    ledger::pages::new_transaction::NewTransactionQuery,
    router::Route,
};

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
    let org_state = use_context::<OrgContextHandle>();
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

    let is_locked = props
        .org_ctx
        .locked_until
        .map_or(false, |lock_date| primary_entry.date <= lock_date);

    let i18n = use_locale();

    let strict = if let Some(org_state) = &org_state {
        org_state.strict_audit_mode
    } else {
        true
    };
    let locked_until = if let Some(org_state) = &org_state {
        org_state.locked_until
    } else {
        None
    };
    let current = if let Some(locked_date) = locked_until {
        primary_entry.date > locked_date
    } else {
        true
    };
    let can_update = !strict && current;

    html! {
        <>
            <tr class="transaction-summary-row">
                <td>
                    <button onclick={on_toggle_expand} class="collapse-toggle">
                        if *expanded {
                            <img src="/images/chevron-down.svg" alt={i18n.t("common-collapse")} />
                        } else {
                            <img src="/images/chevron-right.svg" alt={i18n.t("common-expand")} />
                        }
                    </button>
                    { i18n.format_date(primary_entry.date) }
                </td>
                <td class="table__text-col">{ props.transaction_group.description.clone().unwrap_or_default() }</td>
                <td class="table__value-col">{ i18n.format_currency(primary_entry.debit) }</td>
                <td class="table__value-col">{ i18n.format_currency(primary_entry.credit) }</td>
                <td class="table__value-col">{ i18n.format_currency(primary_entry.running_balance) }</td>
                <td class="actions-cell">
                    <div class="actions-dropdown">
                        <button class="icon-button" onclick={on_toggle_dropdown} title={i18n.t("common-actions")}>
                            <img src="/images/more-vertical.svg" alt={i18n.t("common-actions")} class="dropdown-trigger-icon" />
                        </button>
                        if *dropdown_open {
                            <div class="actions-dropdown__content">

                                <button class="dropdown-item" onclick={on_reverse_click} disabled={is_locked}>
                                    <img src="/images/reverse.svg" alt={i18n.t("transaction-row-reverse")} />
                                    <span>{ i18n.t("transaction-row-reverse") }</span>
                                </button>
                                if can_update {
                                    <button class="dropdown-item" onclick={on_edit_click} disabled={is_locked}>
                                        <img src="/images/edit.svg" alt={i18n.t("common-edit")} />
                                        <span>{ i18n.t("common-edit") }</span>
                                    </button>
                                    <button class="dropdown-item" onclick={on_delete_click} disabled={is_locked}>
                                        <img src="/images/delete.svg" alt={i18n.t("common-delete")} />
                                        <span>{ i18n.t("common-delete") }</span>
                                    </button>
                                }
                                <Link<Route, NewTransactionQuery>
                                    to={Route::NewTransaction}
                                    query={duplicate_query}
                                    classes="dropdown-item"
                                >
                                    <img src="/images/edit.svg" alt={i18n.t("transaction-row-duplicate")} />
                                    <span>{ i18n.t("transaction-row-duplicate") }</span>
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
                                <p>{ i18n.t("transaction-row-loading-details") }</p>
                            } else if let Some(detail) = &*transaction_detail {
                                <div class="journal-entry__header">
                                    <span class="table__text-col">{ i18n.t("transaction-row-details-for") }</span>
                                    <span class="table__text-col">{ &detail.transaction.id.to_string()[0..8] }</span>
                                    <span class="table__value-col">{ i18n.t("common-debit") }</span>
                                    <span class="table__value-col">{ i18n.t("common-credit") }</span>
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
                                        <span class="table__value-col">{ i18n.format_currency(entry.debit) }</span>
                                        <span class="table__value-col">{ i18n.format_currency(entry.credit) }</span>
                                    </div>
                                })}
                            } else {
                                <p class="message__error">{ i18n.t("transaction-row-error-load-details") }</p>
                            }
                        </div>
                    </td>
                </tr>
            }
        </>
    }
}
