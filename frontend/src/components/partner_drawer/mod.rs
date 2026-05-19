pub mod addresses_view;
pub mod general_view;
pub mod address_edit_card;
pub mod contact_edit_card;
pub mod contacts_view;

use yew::prelude::*;
use shared_core::models::partner::Partner;
use shared_core::models::partner_address::PartnerAddress;
use shared_core::models::partner_contact::PartnerContact;
use crate::components::partner_drawer::addresses_view::AddressesView;
use crate::components::partner_drawer::contacts_view::ContactsView;
use crate::components::partner_drawer::general_view::GeneralView;
use shared_core::requests::partner::UpdatePartnerRequest;
use uuid::Uuid;

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
    pub on_submit: Callback<UpdatePartnerRequest>,
    pub ap_accounts: Vec<(Uuid, String)>,
    pub ar_accounts: Vec<(Uuid, String)>,
}

#[function_component(PartnerDrawer)]
pub fn partner_drawer(props: &PartnerDrawerProps) -> Html {
    let active_tab = use_state(|| DrawerTab::General);
    let request = use_state(|| UpdatePartnerRequest {
        legal_name: props.partner.legal_name.clone(),
        trade_name: props.partner.trade_name.clone(),
        tax_identifier: props.partner.tax_identifier.clone(),
        is_vendor: props.partner.is_vendor,
        is_customer: props.partner.is_customer,
        default_ap_account_id: props.partner.default_ap_account_id,
        default_ar_account_id: props.partner.default_ar_account_id,
        addresses: props.partner_addresses.clone(),
        contacts: props.partner_contacts.clone(),
    });

    {
        let request = request.clone();
        let props_partner = props.partner.clone();
        let props_addresses = props.partner_addresses.clone();
        let props_contacts = props.partner_contacts.clone();

        use_effect_with((props_partner, props_addresses, props_contacts), move |(partner, addresses, contacts)| {
            request.set(UpdatePartnerRequest {
                legal_name: partner.legal_name.clone(),
                trade_name: partner.trade_name.clone(),
                tax_identifier: partner.tax_identifier.clone(),
                is_vendor: partner.is_vendor,
                is_customer: partner.is_customer,
                default_ap_account_id: partner.default_ap_account_id,
                default_ar_account_id: partner.default_ar_account_id,
                addresses: addresses.clone(),
                contacts: contacts.clone(),
            });
            || ()
        });
    }

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

    let on_change = {
        let request = request.clone();
        Callback::from(move |req: UpdatePartnerRequest| {
            request.set(req);
        })
    };

    let on_addresses_change = {
        let request = request.clone();
        Callback::from(move |addresses: Vec<PartnerAddress>| {
            let mut req = (*request).clone();
            req.addresses = addresses;
            request.set(req);
        })
    };

    let on_contacts_change = {
        let request = request.clone();
        Callback::from(move |contacts: Vec<PartnerContact>| {
            let mut req = (*request).clone();
            req.contacts = contacts;
            request.set(req);
        })
    };

    let on_submit = {
        let on_submit = props.on_submit.clone();
        let request = request.clone();
        Callback::from(move |_| {
            on_submit.emit((*request).clone());
        })
    };

    html! {
        <div class="drawer-overlay" onclick={on_close.clone()}>
            <div class="partner-drawer" onclick={|e: MouseEvent| e.stop_propagation()}>
                <header class="partner-drawer__header">
                    <h3>{ &props.partner.legal_name }</h3>
                    <button class="btn-close" onclick={on_close.clone()}>{ "✖" }</button>
                </header>
                <div class="partner-drawer__tabs">
                    <button
                        class={classes!("tab-trigger", (*active_tab == DrawerTab::General).then_some("tab-trigger--active"))}
                        onclick={set_tab.reform(|_| DrawerTab::General)}
                    >
                        { "General" }
                    </button>
                    <button
                        class={classes!("tab-trigger", (*active_tab == DrawerTab::Addresses).then_some("tab-trigger--active"))}
                        onclick={set_tab.reform(|_| DrawerTab::Addresses)}
                    >
                        { "Addresses" }
                    </button>
                    <button
                        class={classes!("tab-trigger", (*active_tab == DrawerTab::Contacts).then_some("tab-trigger--active"))}
                        onclick={set_tab.reform(|_| DrawerTab::Contacts)}
                    >
                        { "Contacts" }
                    </button>
                </div>
                <div class="partner-drawer__content">
                    {
                        match *active_tab {
                            DrawerTab::General => html! {
                                <GeneralView
                                    request={(*request).clone()}
                                    on_change={on_change}
                                    ap_accounts={props.ap_accounts.clone()}
                                    ar_accounts={props.ar_accounts.clone()}
                                />
                            },
                            DrawerTab::Addresses => html! { <AddressesView addresses={request.addresses.clone()} on_addresses_change={on_addresses_change} /> },
                            DrawerTab::Contacts => html! { <ContactsView contacts={request.contacts.clone()} on_contacts_change={on_contacts_change} /> },
                        }
                    }
                </div>
                <footer class="partner-drawer__footer">
                    <button class="button-secondary" onclick={on_close.clone()}>{ "Cancel" }</button>
                    <button class="button-primary" onclick={on_submit}>{ "Save Partner" }</button>
                </footer>
            </div>
        </div>
    }
}
