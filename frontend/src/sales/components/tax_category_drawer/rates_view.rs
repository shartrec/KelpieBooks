/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use fluent::fluent_args;
use shared_core::{
    ledger::models::account_category::AccountCategory,
    sales::models::tax::{
        TaxCategory,
        TaxRate,
    },
    TaxRateId,
};
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{
    api::Api,
    contexts::{
        auth_context::use_user_context,
        locale_context::use_locale,
    },
    core::components::delete_confirmation_modal::DeleteConfirmationModal,
    ledger::util::get_accounts_by_category,
    sales::components::tax_category_drawer::tax_rate_edit_card::TaxRateEditCard,
};

#[derive(Properties, PartialEq, Clone)]
pub struct RatesViewProps {
    pub category: TaxCategory,
    pub on_change: Callback<()>,
}

#[function_component(RatesView)]
pub fn rates_view(props: &RatesViewProps) -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();
    let rates = use_state(Vec::new);
    let accounts = use_state(Vec::new);
    let error = use_state(|| None::<String>);
    let category_id = props.category.id;
    let rate_to_edit = use_state(|| None::<TaxRate>);
    let rate_to_delete = use_state(|| None::<TaxRate>);

    let fetch_rates = {
        let rates = rates.clone();
        let error = error.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        Callback::from(move |()| {
            let rates = rates.clone();
            let error = error.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("/api/sales/tax-categories/{}/rates", category_id);
                let fetched_rates = Api::get(&url, user_ctx, navigator).await;
                match fetched_rates {
                    Ok(response) if response.ok() => match response.json::<Vec<TaxRate>>().await {
                        Ok(data) => rates.set(data),
                        Err(e) => error.set(Some(i18n.t_args(
                            "tax-rate-drawer-error-parse-rates",
                            &fluent_args!["error" => e.to_string()],
                        ))),
                    },
                    Ok(response) => error.set(Some(i18n.t_args(
                        "tax-rate-drawer-error-fetch-rates",
                        &fluent_args!["status" => response.status()],
                    ))),
                    Err(e) => error.set(Some(i18n.t_args(
                        "common-network-error",
                        &fluent_args!["error" => e.to_string()],
                    ))),
                }
            });
        })
    };

    let fetch_accounts = {
        let accounts = accounts.clone();
        let error = error.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        Callback::from(move |()| {
            let accounts = accounts.clone();
            let error = error.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_accounts = get_accounts_by_category(
                    AccountCategory::Liability,
                    user_ctx,
                    navigator,
                    &i18n,
                    false,
                )
                .await;
                match fetched_accounts {
                    Ok(postable_accounts) => {
                        accounts.set(postable_accounts);
                    }
                    Err(e) => error.set(Some(e)),
                }
            });
        })
    };

    use_effect_with((), move |()| {
        fetch_rates.emit(());
        fetch_accounts.emit(());
        || ()
    });

    let on_edit_click = {
        let rate_to_edit = rate_to_edit.clone();
        Callback::from(move |rate: TaxRate| {
            rate_to_edit.set(Some(rate));
        })
    };

    let on_save_rate = {
        let rates = rates.clone();
        let rate_to_edit = rate_to_edit.clone();
        Callback::from(move |rate: TaxRate| {
            let mut current_rates = (*rates).clone();
            if let Some(pos) = current_rates.iter().position(|r| r.id == rate.id) {
                current_rates[pos] = rate;
            } else {
                current_rates.push(rate);
            }
            rates.set(current_rates);
            rate_to_edit.set(None);
        })
    };

    let on_cancel_edit = {
        let rate_to_edit = rate_to_edit.clone();
        Callback::from(move |()| {
            rate_to_edit.set(None);
        })
    };

    let on_delete_click = {
        let rate_to_delete = rate_to_delete.clone();
        Callback::from(move |rate: TaxRate| {
            rate_to_delete.set(Some(rate));
        })
    };

    let on_delete_confirm = {
        let rates = rates.clone();
        let rate_to_delete = rate_to_delete.clone();
        Callback::from(move |()| {
            if let Some(rate_to_delete) = &*rate_to_delete {
                let mut current_rates = (*rates).clone();
                current_rates.retain(|r| r.id != rate_to_delete.id);
                rates.set(current_rates);
            }
            rate_to_delete.set(None);
        })
    };

    let on_delete_cancel = {
        let rate_to_delete = rate_to_delete.clone();
        Callback::from(move |()| {
            rate_to_delete.set(None);
        })
    };

    let add_rate = {
        let rate_to_edit = rate_to_edit.clone();
        Callback::from(move |_| {
            rate_to_edit.set(Some(TaxRate {
                id: TaxRateId::default(),
                tax_category_id: category_id,
                ..TaxRate::default()
            }));
        })
    };

    let on_submit = {
        let rates = rates.clone();
        let on_change = props.on_change.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        let error = error.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let rates = rates.clone();
            let on_change = on_change.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Api::put(
                    &format!("/api/sales/tax-categories/{}/rates", category_id),
                    &*rates,
                    user_ctx,
                    navigator,
                )
                .await;
                match resp {
                    Ok(r) if r.ok() => {
                        on_change.emit(());
                    }
                    Ok(r) => error.set(Some(i18n.t_args(
                        "tax-rate-drawer-error-update-rates",
                        &fluent_args!["status" => r.status()],
                    ))),
                    Err(e) => error.set(Some(i18n.t_args(
                        "common-network-error",
                        &fluent_args!["error" => e.to_string()],
                    ))),
                }
            });
        })
    };

    html! {
        <div class="drawer-content">
            if let Some(rate) = &*rate_to_edit {
                <TaxRateEditCard
                    rate={rate.clone()}
                    accounts={(*accounts).clone()}
                    on_save={on_save_rate}
                    on_cancel={on_cancel_edit}
                />
            }
            <form onsubmit={on_submit}>
                { for (*rates).iter().map(|rate| {
                    let on_edit = {
                        let on_edit_click = on_edit_click.clone();
                        let rate = rate.clone();
                        Callback::from(move |_| on_edit_click.emit(rate.clone()))
                    };
                    let on_delete = {
                        let on_delete_click = on_delete_click.clone();
                        let rate = rate.clone();
                        Callback::from(move |_| on_delete_click.emit(rate.clone()))
                    };

                    let from_date_str = i18n.format_date(rate.valid_from);
                    let to_date_str = rate.valid_to.map(|d| i18n.format_date(d)).unwrap_or_else(|| i18n.t("common-present"));

                    html! {
                        <div class="card-item-compact">
                            <div class="card-item-compact__meta">
                                <span class="card-item-compact__account-badge">{ &rate.name }</span>
                                <div class="card__actions" style="display: flex; gap: 4px;">
                                    <button type="button" class="icon-button" onclick={on_edit}>
                                        <img src="/images/edit.svg" alt={i18n.t("common-edit")} style="width: 13px; height: 13px;" />
                                    </button>
                                    <button type="button" class="icon-button" onclick={on_delete}>
                                        <img src="/images/delete.svg" alt={i18n.t("common-delete")} style="width: 13px; height: 13px;" />
                                    </button>
                                </div>
                            </div>
                            <div class="card-item-compact__body">

                                <p class="card-item-compact__desc">{ i18n.t_args("tax-rate-drawer-validity", &fluent_args!["from" => from_date_str, "to" => to_date_str]) }</p>
                                <div class="card-item-compact__financials">
                                    <p class="card-item-compact__total">{ i18n.format_percentage(rate.rate) }</p>
                                </div>
                            </div>
                        </div>
                    }
                })}
                <div class="table-actions">
                    <button type="button" class="button-primary" onclick={add_rate}>{ i18n.t("tax-rate-drawer-add-rate-button") }</button>
                </div>
                <div class="voucher-footer">
                    if let Some(e) = &*error {
                        <div class="error">{e}</div>
                    }
                    <button type="submit" class="button-primary">{ i18n.t("common-save") }</button>
                </div>
            </form>
            if let Some(rate) = &*rate_to_delete {
                <DeleteConfirmationModal
                    title={i18n.t("tax-rate-drawer-delete-rate-title")}
                    message={i18n.t_args("tax-rate-drawer-delete-rate-message", &fluent_args!["name" => rate.name.clone()])}
                    on_confirm={on_delete_confirm}
                    on_cancel={on_delete_cancel}
                />
            }
        </div>
    }
}
