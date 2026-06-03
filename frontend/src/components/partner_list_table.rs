/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::api::Api;
use crate::components::add_partner_modal::AddPartnerModal;
use crate::components::delete_partner_confirmation_modal::DeletePartnerConfirmationModal;
use crate::components::partner_drawer::PartnerDrawer;
use crate::components::partner_row::PartnerRow;
use crate::contexts::auth_context::use_user_context;
use crate::contexts::locale_context::use_locale;
use fluent::fluent_args;
use shared_core::dtos::partner_list_item::PartnerListItem;
use shared_core::models::account::Account;
use shared_core::models::account_category::AccountCategory;
use shared_core::models::auth::SystemPrivilege;
use shared_core::models::partner::Partner;
use shared_core::models::partner_address::PartnerAddress;
use shared_core::models::partner_contact::PartnerContact;
use shared_core::requests::partner::CreatePartnerRequest;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[function_component(PartnerListTable)]
pub fn partner_list_table() -> Html {
    let user_ctx = use_user_context();
    let i18n = use_locale();
    let navigator = use_navigator().unwrap();
    let partners = use_state(Vec::new);
    let accounts = use_state(Vec::new);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);
    let show_add_modal = use_state(|| false);
    let partner_to_edit = use_state(|| None::<Partner>);
    let partner_addresses = use_state(Vec::new);
    let partner_contacts = use_state(Vec::new);
    let partner_to_delete = use_state(|| None::<PartnerListItem>);

    let fetch_data = {
        let partners = partners.clone();
        let accounts = accounts.clone();
        let error = error.clone();
        let loading = loading.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        Callback::from(move |_: ()| {
            let partners = partners.clone();
            let accounts = accounts.clone();
            let error = error.clone();
            let loading = loading.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            loading.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_partners =
                    Api::get("/api/partners", user_ctx.clone(), navigator.clone()).await;
                match fetched_partners {
                    Ok(response) if response.ok() => {
                        match response.json::<Vec<PartnerListItem>>().await {
                            Ok(data) => partners.set(data),
                            Err(e) => error.set(Some(i18n.t_args(
                                "partner-list-error-parse-partners",
                                &fluent_args!["error" => e.to_string()],
                            ))),
                        }
                    }
                    Ok(response) => error.set(Some(i18n.t_args(
                        "partner-list-error-fetch-partners",
                        &fluent_args!["status" => response.status()],
                    ))),
                    Err(e) => error.set(Some(i18n.t_args(
                        "common-network-error",
                        &fluent_args!["error" => e.to_string()],
                    ))),
                }

                let fetched_accounts =
                    Api::get("/api/accounts", user_ctx, navigator).await;
                match fetched_accounts {
                    Ok(response) if response.ok() => {
                        match response.json::<Vec<Account>>().await {
                            Ok(data) => accounts.set(data),
                            Err(e) => error.set(Some(i18n.t_args(
                                "partner-list-error-parse-accounts",
                                &fluent_args!["error" => e.to_string()],
                            ))),
                        }
                    }
                    Ok(response) => error.set(Some(i18n.t_args(
                        "partner-list-error-fetch-accounts",
                        &fluent_args!["status" => response.status()],
                    ))),
                    Err(e) => error.set(Some(i18n.t_args(
                        "common-network-error",
                        &fluent_args!["error" => e.to_string()],
                    ))),
                }
                loading.set(false);
            });
        })
    };

    let fetch_data_clone = fetch_data.clone();
    use_effect_with((), move |()| {
        fetch_data_clone.emit(());
        || ()
    });

    let on_modal_close = {
        let show_add_modal = show_add_modal.clone();
        let partner_to_edit = partner_to_edit.clone();
        let partner_to_delete = partner_to_delete.clone();
        Callback::from(move |_: ()| {
            show_add_modal.set(false);
            partner_to_edit.set(None);
            partner_to_delete.set(None);
        })
    };

    let on_add_click = {
        let show_add_modal = show_add_modal.clone();
        Callback::from(move |_| show_add_modal.set(true))
    };

    let on_edit_click = {
        let partner_to_edit = partner_to_edit.clone();
        let partner_addresses = partner_addresses.clone();
        let partner_contacts = partner_contacts.clone();
        let error = error.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        Callback::from(move |id: Uuid| {
            let partner_to_edit = partner_to_edit.clone();
            let partner_addresses = partner_addresses.clone();
            let partner_contacts = partner_contacts.clone();
            let error = error.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Api::get(
                    &format!("/api/partners/{}", id),
                    user_ctx.clone(),
                    navigator.clone(),
                )
                .await;
                match resp {
                    Ok(r) if r.ok() => match r.json::<Partner>().await {
                        Ok(partner) => partner_to_edit.set(Some(partner)),
                        Err(e) => error.set(Some(i18n.t_args(
                            "partner-list-error-parse-partner",
                            &fluent_args!["error" => e.to_string()],
                        ))),
                    },
                    Ok(r) => error.set(Some(i18n.t_args(
                        "partner-list-error-fetch-partner",
                        &fluent_args!["status" => r.status()],
                    ))),
                    Err(e) => error.set(Some(i18n.t_args(
                        "common-network-error",
                        &fluent_args!["error" => e.to_string()],
                    ))),
                }

                let resp = Api::get(
                    &format!("/api/partners/{}/addresses", id),
                    user_ctx.clone(),
                    navigator.clone(),
                )
                .await;
                match resp {
                    Ok(r) if r.ok() => match r.json::<Vec<PartnerAddress>>().await {
                        Ok(addresses) => partner_addresses.set(addresses),
                        Err(e) => error.set(Some(i18n.t_args(
                            "partner-list-error-parse-addresses",
                            &fluent_args!["error" => e.to_string()],
                        ))),
                    },
                    Ok(r) => error.set(Some(i18n.t_args(
                        "partner-list-error-fetch-addresses",
                        &fluent_args!["status" => r.status()],
                    ))),
                    Err(e) => error.set(Some(i18n.t_args(
                        "common-network-error",
                        &fluent_args!["error" => e.to_string()],
                    ))),
                }

                let resp = Api::get(
                    &format!("/api/partners/{}/contacts", id),
                    user_ctx,
                    navigator,
                )
                .await;
                match resp {
                    Ok(r) if r.ok() => match r.json::<Vec<PartnerContact>>().await {
                        Ok(contacts) => partner_contacts.set(contacts),
                        Err(e) => error.set(Some(i18n.t_args(
                            "partner-list-error-parse-contacts",
                            &fluent_args!["error" => e.to_string()],
                        ))),
                    },
                    Ok(r) => error.set(Some(i18n.t_args(
                        "partner-list-error-fetch-contacts",
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

    let on_delete_click = {
        let partner_to_delete = partner_to_delete.clone();
        Callback::from(move |partner: PartnerListItem| {
            if partner.can_delete {
                partner_to_delete.set(Some(partner));
            }
        })
    };

    let on_add_submit = {
        let on_modal_close = on_modal_close.clone();
        let error = error.clone();
        let fetch_data = fetch_data.clone();
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        Callback::from(move |req: CreatePartnerRequest| {
            let on_modal_close = on_modal_close.clone();
            let error = error.clone();
            let fetch_data = fetch_data.clone();
            let user_ctx = user_ctx.clone();
            let i18n = i18n.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Api::post("/api/partners", &req, user_ctx, navigator).await;
                match resp {
                    Ok(r) if r.ok() => {
                        on_modal_close.emit(());
                        fetch_data.emit(());
                    }
                    Ok(r) => error.set(Some(i18n.t_args(
                        "partner-list-error-add-partner",
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

    let on_partner_change = {
        let partner_id = partner_to_edit.as_ref().map(|p| p.id);
        let on_edit_click = on_edit_click.clone();
        Callback::from(move |()| {
            if let Some(id) = partner_id {
                on_edit_click.emit(id);
            }
        })
    };

    let on_delete_confirm = {
        let on_modal_close = on_modal_close.clone();
        let error = error.clone();
        let fetch_data = fetch_data.clone();
        let partner_id = partner_to_delete.as_ref().map(|p| p.id);
        let user_ctx = user_ctx.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        Callback::from(move |_: ()| {
            if let Some(id) = partner_id {
                let on_modal_close = on_modal_close.clone();
                let error = error.clone();
                let fetch_data = fetch_data.clone();
                let user_ctx = user_ctx.clone();
                let i18n = i18n.clone();
                let navigator = navigator.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let resp =
                        Api::delete(&format!("/api/partners/{}", id), user_ctx, navigator).await;
                    match resp {
                        Ok(r) if r.ok() => {
                            on_modal_close.emit(());
                            fetch_data.emit(());
                        }
                        Ok(r) => {
                            error.set(Some(i18n.t_args(
                                "partner-list-error-delete-partner",
                                &fluent_args!["status" => r.status()],
                            )))
                        }
                        Err(e) => error.set(Some(i18n.t_args(
                            "common-network-error",
                            &fluent_args!["error" => e.to_string()],
                        ))),
                    }
                });
            }
        })
    };

    let ap_accounts: Vec<(Uuid, String)> = (*accounts)
        .iter()
        .filter(|a| a.category == AccountCategory::Liability)
        .map(|a| (a.id, a.name.clone()))
        .collect();
    let ar_accounts: Vec<(Uuid, String)> = (*accounts)
        .iter()
        .filter(|a| a.category == AccountCategory::Asset)
        .map(|a| (a.id, a.name.clone()))
        .collect();

    if *loading {
        return html! { <p>{ i18n.t("common-loading") }</p> };
    }
    if let Some(err) = &*error {
        return html! { <div class="error">{ err }</div> };
    }

    html! {
        <>
            <div class="table-actions">
                { if user_ctx.has_privilege(&SystemPrivilege::manage_partners) {
                    html! {
                        <button onclick={on_add_click}>{ i18n.t("partner-list-add-partner-button") }</button>
                    }
                } else {
                    html! {}
                }}
            </div>

            if *show_add_modal {
                <AddPartnerModal
                    on_close={on_modal_close.clone()}
                    on_submit={on_add_submit}
                    ap_accounts={ap_accounts.clone()}
                    ar_accounts={ar_accounts.clone()}
                />
            }
            if let Some(partner) = &*partner_to_edit {
                <PartnerDrawer
                    partner={partner.clone()}
                    partner_addresses={(*partner_addresses).clone()}
                    partner_contacts={(*partner_contacts).clone()}
                    on_close={on_modal_close.clone()}
                    on_change={on_partner_change}
                    ap_accounts={ap_accounts.clone()}
                    ar_accounts={ar_accounts.clone()}
                />
            }
            if let Some(partner) = &*partner_to_delete {
                <DeletePartnerConfirmationModal
                    partner={partner.clone()}
                    on_close={on_modal_close.clone()}
                    on_confirm={on_delete_confirm}
                />
            }

            <table class="table">
                <thead>
                    <tr>
                        <th class="table__text-col">{ i18n.t("partner-list-legal-name") }</th>
                        <th class="table__text-col">{ i18n.t("partner-list-trade-name") }</th>
                        <th class="table__text-col">{ i18n.t("common-type") }</th>
                        <th class="table__col-actions">{ i18n.t("common-actions") }</th>
                    </tr>
                </thead>
                <tbody>
                    { for (*partners).iter().map(|partner| html! {
                        <PartnerRow
                            partner={partner.clone()}
                            on_edit={on_edit_click.clone()}
                            on_delete={on_delete_click.clone()}
                        />
                    })}
                </tbody>
            </table>
        </>
    }
}
