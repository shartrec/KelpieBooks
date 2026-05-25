/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

pub mod address_edit_card;
pub mod addresses_view;
pub mod contact_edit_card;
pub mod contacts_view;
pub mod delete_address_confirmation_modal;
pub mod delete_contact_confirmation_modal;
pub mod general_view;

use crate::api::Api;
use crate::components::partner_drawer::addresses_view::AddressesView;
use crate::components::partner_drawer::contacts_view::ContactsView;
use crate::components::partner_drawer::general_view::GeneralView;
use crate::contexts::auth_context::use_user_context;
use fluent::fluent_args;
use shared_core::i18n::{t, t_args};
use shared_core::models::partner::Partner;
use shared_core::models::partner_address::PartnerAddress;
use shared_core::models::partner_contact::PartnerContact;
use shared_core::requests::partner::UpdatePartnerRequest;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawerTab {
    General,
    Addresses,
    Contacts,
}

#[derive(Properties, PartialEq)]
pub struct PartnerDrawerProps {
    pub partner: Partner,
    pub partner_addresses: Vec<PartnerAddress>,
    pub partner_contacts: Vec<PartnerContact>,
    pub on_close: Callback<()>,
    pub on_change: Callback<()>,
    pub ap_accounts: Vec<(Uuid, String)>,
    pub ar_accounts: Vec<(Uuid, String)>,
}

#[function_component(PartnerDrawer)]
pub fn partner_drawer(props: &PartnerDrawerProps) -> Html {
    let user_ctx = use_user_context();
    let navigator = use_navigator().unwrap();
    let active_tab = use_state(|| DrawerTab::General);
    let error = use_state(|| None::<String>);

    let set_tab = {
        let active_tab = active_tab.clone();
        Callback::from(move |tab: DrawerTab| {
            active_tab.set(tab);
        })
    };

    let on_close = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| {
            on_close.emit(());
        })
    };

    let on_general_submit = {
        let on_change = props.on_change.clone();
        let partner_id = props.partner.id;
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        let error = error.clone();
        Callback::from(move |req: UpdatePartnerRequest| {
            let on_change = on_change.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Api::put(
                    &format!("/api/partners/{}", partner_id),
                    &req,
                    user_ctx,
                    navigator,
                )
                .await;
                match resp {
                    Ok(r) if r.ok() => {
                        on_change.emit(());
                    }
                    Ok(r) => error.set(Some(t_args(
                        "partner-drawer-error-save",
                        &fluent_args!["status" => r.status()],
                    ))),
                    Err(e) => error.set(Some(t_args(
                        "common-network-error",
                        &fluent_args!["error" => e.to_string()],
                    ))),
                }
            });
        })
    };

    html! {
        <div class="drawer-overlay" onclick={on_close.clone()}>
            <div class="drawer" onclick={|e: MouseEvent| e.stop_propagation()}>
                <header class="drawer__header">
                    <h3>{ &props.partner.legal_name } </h3>
                        <button class="btn-close" type="button" onclick={on_close.clone()}>
                            <img src="/images/x.svg" alt={t("common-close")} />
                        </button>
                </header>
                <div class="drawer__tabs">
                    <button
                        class={classes!("tab-trigger", (*active_tab == DrawerTab::General).then_some("tab-trigger--active"))}
                        onclick={set_tab.reform(|_| DrawerTab::General)}
                    >
                        { t("common-general") }
                    </button>
                    <button
                        class={classes!("tab-trigger", (*active_tab == DrawerTab::Addresses).then_some("tab-trigger--active"))}
                        onclick={set_tab.reform(|_| DrawerTab::Addresses)}
                    >
                        { t("common-addresses") }
                    </button>
                    <button
                        class={classes!("tab-trigger", (*active_tab == DrawerTab::Contacts).then_some("tab-trigger--active"))}
                        onclick={set_tab.reform(|_| DrawerTab::Contacts)}
                    >
                        { t("common-contacts") }
                    </button>
                </div>
                <div class="drawer__content">
                    if let Some(e) = &*error {
                        <div class="error">{e}</div>
                    }
                    {
                        match *active_tab {
                            DrawerTab::General => html! {
                                <GeneralView
                                    partner={props.partner.clone()}
                                    on_submit={on_general_submit}
                                    ap_accounts={props.ap_accounts.clone()}
                                    ar_accounts={props.ar_accounts.clone()}
                                />
                            },
                            DrawerTab::Addresses => html! { <AddressesView
                                addresses={props.partner_addresses.clone()}
                                on_change={props.on_change.clone()}
                                partner_id={props.partner.id}
                                />
                            },
                            DrawerTab::Contacts => html! { <ContactsView
                                contacts={props.partner_contacts.clone()}
                                on_change={props.on_change.clone()}
                                partner_id={props.partner.id}
                                />
                            },
                        }
                    }
                </div>
                <footer class="drawer__footer">
                    <button class="button-secondary" onclick={on_close.clone()}>{ t("common-close") }</button>
                </footer>
            </div>
        </div>
    }
}
