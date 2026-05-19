/*
 * Copyright (c) 2026. Trevor Campbell and others.
 *
 * This file is part of KelpieBooks.
 *
 * KelpieBooks is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License,or
 * (at your option) any later version.
 *
 * KelpieBooks is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with KelpieBooks; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 *
 * Contributors:
 *      Trevor Campbell
 *
 */

use crate::api::Api;
use crate::components::add_partner_modal::AddPartnerModal;
use crate::components::delete_partner_confirmation_modal::DeletePartnerConfirmationModal;
use crate::components::partner_drawer::PartnerDrawer;
use crate::components::partner_row::PartnerRow;
use crate::contexts::auth_context::use_user_context;
use shared_core::dtos::account_with_balance::AccountWithBalance;
use shared_core::dtos::partner_list_item::PartnerListItem;
use shared_core::models::account_category::AccountCategory;
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
        let navigator = navigator.clone();
        Callback::from(move |_: ()| {
            let partners = partners.clone();
            let accounts = accounts.clone();
            let error = error.clone();
            let loading = loading.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            loading.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_partners =
                    Api::get("/api/partners", user_ctx.clone(), navigator.clone()).await;
                match fetched_partners {
                    Ok(response) if response.ok() => {
                        match response.json::<Vec<PartnerListItem>>().await {
                            Ok(data) => partners.set(data),
                            Err(e) => error.set(Some(format!("Failed to parse partners: {}", e))),
                        }
                    }
                    Ok(response) => error.set(Some(format!(
                        "Failed to fetch partners: {}",
                        response.status()
                    ))),
                    Err(e) => error.set(Some(format!("Network error: {}", e))),
                }

                let fetched_accounts =
                    Api::get("/api/accounts_with_balances", user_ctx, navigator).await;
                match fetched_accounts {
                    Ok(response) if response.ok() => {
                        match response.json::<Vec<AccountWithBalance>>().await {
                            Ok(data) => accounts.set(data),
                            Err(e) => error.set(Some(format!("Failed to parse accounts: {}", e))),
                        }
                    }
                    Ok(response) => error.set(Some(format!(
                        "Failed to fetch accounts: {}",
                        response.status()
                    ))),
                    Err(e) => error.set(Some(format!("Network error: {}", e))),
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
        let navigator = navigator.clone();
        Callback::from(move |id: Uuid| {
            let partner_to_edit = partner_to_edit.clone();
            let partner_addresses = partner_addresses.clone();
            let partner_contacts = partner_contacts.clone();
            let error = error.clone();
            let user_ctx = user_ctx.clone();
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
                        Err(e) => error.set(Some(format!("Failed to parse partner: {}", e))),
                    },
                    Ok(r) => error.set(Some(format!("Failed to fetch partner: {}", r.status()))),
                    Err(e) => error.set(Some(format!("Network error: {}", e))),
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
                        Err(e) => error.set(Some(format!("Failed to parse addresses: {}", e))),
                    },
                    Ok(r) => error.set(Some(format!("Failed to fetch addresses: {}", r.status()))),
                    Err(e) => error.set(Some(format!("Network error: {}", e))),
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
                        Err(e) => error.set(Some(format!("Failed to parse contacts: {}", e))),
                    },
                    Ok(r) => error.set(Some(format!("Failed to fetch contacts: {}", r.status()))),
                    Err(e) => error.set(Some(format!("Network error: {}", e))),
                }
            });
        })
    };

    let on_delete_click = {
        let partner_to_delete = partner_to_delete.clone();
        Callback::from(move |partner: PartnerListItem| partner_to_delete.set(Some(partner)))
    };

    let on_add_submit = {
        let on_modal_close = on_modal_close.clone();
        let error = error.clone();
        let fetch_data = fetch_data.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        Callback::from(move |req: CreatePartnerRequest| {
            let on_modal_close = on_modal_close.clone();
            let error = error.clone();
            let fetch_data = fetch_data.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Api::post("/api/partners", &req, user_ctx, navigator).await;
                match resp {
                    Ok(r) if r.ok() => {
                        on_modal_close.emit(());
                        fetch_data.emit(());
                    }
                    Ok(r) => error.set(Some(format!("Failed to add partner: {}", r.status()))),
                    Err(e) => error.set(Some(format!("Network error: {}", e))),
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
        let navigator = navigator.clone();
        Callback::from(move |_: ()| {
            if let Some(id) = partner_id {
                let on_modal_close = on_modal_close.clone();
                let error = error.clone();
                let fetch_data = fetch_data.clone();
                let user_ctx = user_ctx.clone();
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
                            error.set(Some(format!("Failed to delete partner: {}", r.status())))
                        }
                        Err(e) => error.set(Some(format!("Network error: {}", e))),
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
        return html! { <p>{ "Loading..." }</p> };
    }
    if let Some(err) = &*error {
        return html! { <div class="error">{ err }</div> };
    }

    html! {
        <>
            <div class="table-actions">
                <button onclick={on_add_click}>{ "Add Partner" }</button>
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
                        <th class="table__text-col">{ "Legal Name" }</th>
                        <th class="table__text-col">{ "Trade Name" }</th>
                        <th class="table__text-col">{ "Type" }</th>
                        <th class="table__col-actions">{ "Actions" }</th>
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
