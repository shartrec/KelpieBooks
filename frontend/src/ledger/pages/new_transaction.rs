/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use chrono::{
    Duration,
    NaiveDate,
};
use fluent::fluent_args;
use rust_decimal::{dec, Decimal};
use serde::{
    Deserialize,
    Serialize,
};
use shared_core::ledger::{
    dtos::transaction_detail::TransactionDetail,
    models::account::Account,
    requests::transaction::{
        CreateTransactionRequest,
        JournalEntryLine,
        UpdateTransactionRequest,
    },
};
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    api::Api,
    core::components::layout::Layout,
    contexts::{
        auth_context::use_user_context,
        locale_context::use_locale,
        org_context::OrgContextHandle,
    },
    ledger::components::journal_entry_row::JournalEntryRow,
    router::Route,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct NewTransactionQuery {
    #[serde(default)]
    #[serde(rename = "from_account")]
    pub from_account: Option<Uuid>,
    #[serde(default)]
    #[serde(rename = "duplicate_from")]
    pub duplicate_from: Option<Uuid>,
    #[serde(default)]
    #[serde(rename = "edit_id")]
    pub edit_id: Option<Uuid>,
}

#[function_component(NewTransactionPage)]
pub fn new_transaction_page() -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let request = use_state(CreateTransactionRequest::default);
    let edit_id = use_state(|| None::<Uuid>);
    let focus_index = use_state(|| None::<usize>);
    let postable_accounts = use_state(Vec::new);
    let from_account = use_state(|| None::<Account>);
    let org_ctx = use_context::<OrgContextHandle>().expect("OrgContext not found");
    let navigator = use_navigator().unwrap();
    let location = use_location().unwrap();

    {
        let request = request.clone();
        let edit_id_state = edit_id.clone();
        let postable_accounts = postable_accounts.clone();
        let from_account = from_account.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();

        use_effect_with((), move |_| {
            let query = location.query::<NewTransactionQuery>().ok();
            let from_account_id = query.as_ref().and_then(|q| q.from_account);
            let duplicate_from_id = query.as_ref().and_then(|q| q.duplicate_from);
            let edit_id = query.as_ref().and_then(|q| q.edit_id);
            edit_id_state.set(edit_id);
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(response) =
                    Api::get("/api/accounts", user_ctx.clone(), navigator.clone()).await
                {
                    if let Ok(accounts) = response.json::<Vec<Account>>().await {
                        let postable = accounts
                            .into_iter()
                            .filter(|a| !a.is_group)
                            .map(|a| (a.id, a.name))
                            .collect();
                        postable_accounts.set(postable);
                    }
                }

                if let Some(id) = from_account_id {
                    if let Ok(response) = Api::get(
                        &format!("/api/accounts/{}", id),
                        user_ctx.clone(),
                        navigator.clone(),
                    )
                    .await
                    {
                        if let Ok(acc) = response.json::<Account>().await {
                            from_account.set(Some(acc));
                        }
                    }
                }

                let mut new_req = CreateTransactionRequest::default();
                let transaction_id_to_load = edit_id.or(duplicate_from_id);

                if let Some(id) = transaction_id_to_load {
                    if let Ok(response) =
                        Api::get(&format!("/api/transactions/{}", id), user_ctx, navigator).await
                    {
                        if let Ok(detail) = response.json::<TransactionDetail>().await {
                            new_req.date = detail.transaction.date;
                            new_req.reference = detail.transaction.reference;
                            new_req.entries = detail
                                .entries
                                .into_iter()
                                .map(|e| JournalEntryLine {
                                    line_id: Uuid::new_v4(),
                                    account_id: e.account_id,
                                    debit: e.debit,
                                    credit: e.credit,
                                    description: e.description,
                                })
                                .collect();
                        }
                    }
                } else {
                    let mut entries =
                        vec![JournalEntryLine::default(), JournalEntryLine::default()];
                    if let Some(id) = from_account_id {
                        entries[0].account_id = id;
                    }
                    new_req.entries = entries;
                }
                request.set(new_req);
            });
            || ()
        });
    }

    let on_entry_change = {
        let request = request.clone();
        Callback::from(move |(index, updated_entry): (usize, JournalEntryLine)| {
            let mut new_req = (*request).clone();
            if let Some(entry) = new_req.entries.get_mut(index) {
                *entry = updated_entry;
            }
            request.set(new_req);
        })
    };

    let on_delete_line = {
        let request = request.clone();
        Callback::from(move |index: usize| {
            if request.entries.len() > 2 {
                let mut new_req = (*request).clone();
                new_req.entries.remove(index);
                request.set(new_req);
            }
        })
    };

    let on_date_change = {
        let request = request.clone();
        Callback::from(move |e: Event| {
            let value = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            if let Ok(date) = NaiveDate::parse_from_str(&value, "%Y-%m-%d") {
                let mut new_req = (*request).clone();
                new_req.date = date;
                request.set(new_req);
            }
        })
    };

    let total_debits: Decimal = request.entries.iter().map(|e| e.debit).sum();
    let total_credits: Decimal = request.entries.iter().map(|e| e.credit).sum();
    let is_balanced = total_debits > dec!(0.00) && total_debits == total_credits;
    let earliest_date = org_ctx.locked_until.unwrap_or(NaiveDate::default()) + Duration::days(1);

    let is_period_locked = org_ctx
        .locked_until
        .map(|lock| request.date <= lock)
        .unwrap_or(false);

    let on_submit = {
        let request = request.clone();
        let navigator = navigator.clone();
        let edit_id = *edit_id;
        let user_ctx = user_ctx.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if is_balanced {
                let mut req = (*request).clone();
                req.entries.retain(|entry| {
                    !entry.account_id.is_nil() && (entry.debit != dec!(0.00) || entry.credit != dec!(0.00))
                });
                let navigator = navigator.clone();
                let user_ctx = user_ctx.clone();

                if let Some(id) = edit_id {
                    // Update existing transaction
                    let update_req = UpdateTransactionRequest {
                        date: req.date,
                        description: req.description,
                        reference: req.reference,
                        entries: req.entries.into_iter().map(|e| e.into()).collect(),
                    };
                    wasm_bindgen_futures::spawn_local(async move {
                        let resp = Api::put(
                            &format!("/api/transactions/{}", id),
                            &update_req,
                            user_ctx,
                            navigator.clone(),
                        )
                        .await;
                        if resp.is_ok() {
                            navigator.back();
                        } else {
                            // TODO: Handle error
                        }
                    });
                } else {
                    // Create new transaction
                    wasm_bindgen_futures::spawn_local(async move {
                        let resp =
                            Api::post("/api/transactions", &req, user_ctx, navigator.clone()).await;
                        if resp.is_ok() {
                            navigator.back();
                        } else {
                            // TODO: Handle error
                        }
                    });
                }
            }
        })
    };

    let on_cancel = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.back();
        })
    };

    let is_edit_mode = edit_id.is_some();
    let page_title = if is_edit_mode {
        i18n.t("new-transaction-edit-title")
    } else {
        i18n.t("new-transaction-new-title")
    };
    let save_button_text = if is_edit_mode {
        i18n.t("new-transaction-update-button")
    } else {
        i18n.t("new-transaction-save-button")
    };

    let page_header = if let Some(acc) = &*from_account {
        html! {
            <div class="page-subheader">
                <h3>{ i18n.t("new-transaction-for-label") }<Link<Route> to={Route::AccountLedger { id: acc.id }}>{ &acc.name }</Link<Route>></h3>
            </div>
        }
    } else {
        html! {}
    };

    let value = focus_index.clone();
    let add_line = {
        let request = request.clone();
        Callback::from(move |_| {
            let mut new_req = (*request).clone();
            let new_idx = new_req.entries.len();
            new_req.entries.push(JournalEntryLine::default());
            request.set(new_req);
            value.set(Some(new_idx));
        })
    };

    html! {
       <Layout>
           <h1>{ page_title }</h1>
           { page_header }
           <form onsubmit={on_submit} class="transaction__form">
               <div class="transaction__form__header">
                   <label>
                       { i18n.t("new-transaction-date-label") }
                   </label>
                       <input type="date" value={request.date.format("%Y-%m-%d").to_string()} onchange={on_date_change}
                           min={ earliest_date.format("%Y-%m-%d").to_string() }
                       />
               </div>

               <div class="journal__entries">
                   <div class="journal__entry-header">
                       <span>{ i18n.t("common-account") }</span>
                       <span>{ i18n.t("common-description") }</span>
                       <span>{ i18n.t("common-debit") }</span>
                       <span>{ i18n.t("common-credit") }</span>
                       <span></span>
                   </div>
                   { for request.entries.iter().enumerate().map(|(i, entry)| {
                       let on_change = { let on_entry_change = on_entry_change.clone(); Callback::from(move |updated_entry| { on_entry_change.emit((i, updated_entry)); }) };
                       let on_delete = { let on_delete_line = on_delete_line.clone(); Callback::from(move |_| { on_delete_line.emit(i); }) };
                       html!{
                           <JournalEntryRow
                               key={entry.line_id.to_string()}
                               entry={entry.clone()}
                               on_change={on_change}
                               on_delete={on_delete}
                               accounts={(*postable_accounts).clone()}
                               should_focus={*focus_index == Some(i)}
                           />
                       }
                   })}
               </div>
               <div class="modal__form__actions">
                   <button type="button" onclick={add_line} class="button-add-row">{ i18n.t("new-transaction-add-line-button") }</button>
               </div>
               <div class="transaction__form__totals">
                   <div>{ i18n.t_args("new-transaction-debits-total", &fluent_args!["amount" => i18n.format_currency(total_debits)]) }</div>
                   <div>{ i18n.t_args("new-transaction-credits-total", &fluent_args!["amount" => i18n.format_currency(total_credits)]) }</div>
                   <div class={if is_balanced { "transaction__form__balanced" } else { "transaction__form__unbalanced" }}>
                       { if is_balanced { i18n.t("new-transaction-balanced") } else { i18n.t("new-transaction-unbalanced") } }
                   </div>
               </div>

               <div class="modal__form__actions">
                   <button type="button" onclick={on_cancel} class="button-secondary">{ i18n.t("common-cancel") }</button>
                   <button type="submit" disabled={!is_balanced || is_period_locked}>{
                          if is_period_locked { i18n.t("new-transaction-period-locked") }
                          else { save_button_text }
                       }</button>
               </div>
           </form>
       </Layout>
    }
}
