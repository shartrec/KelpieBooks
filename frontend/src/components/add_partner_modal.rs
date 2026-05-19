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

use shared_core::requests::partner::CreatePartnerRequest;
use uuid::Uuid;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AddPartnerModalProps {
    pub on_close: Callback<()>,
    pub on_submit: Callback<CreatePartnerRequest>,
    pub ap_accounts: Vec<(Uuid, String)>,
    pub ar_accounts: Vec<(Uuid, String)>,
}

#[function_component(AddPartnerModal)]
pub fn add_partner_modal(props: &AddPartnerModalProps) -> Html {
    let request = use_state(CreatePartnerRequest::default);
    let error = use_state(|| None::<String>);

    let on_legal_name_input = {
        let state = request.clone();
        Callback::from(move |e: InputEvent| {
            let mut info = (*state).clone();
            info.legal_name = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            state.set(info);
        })
    };

    let on_trade_name_input = {
        let state = request.clone();
        Callback::from(move |e: InputEvent| {
            let mut info = (*state).clone();
            info.trade_name = Some(
                e.target_unchecked_into::<web_sys::HtmlInputElement>()
                    .value(),
            );
            state.set(info);
        })
    };

    let on_tax_identifier_input = {
        let state = request.clone();
        Callback::from(move |e: InputEvent| {
            let mut info = (*state).clone();
            info.tax_identifier = Some(
                e.target_unchecked_into::<web_sys::HtmlInputElement>()
                    .value(),
            );
            state.set(info);
        })
    };

    let on_is_vendor_change = {
        let state = request.clone();
        Callback::from(move |e: Event| {
            let mut info = (*state).clone();
            info.is_vendor = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .checked();
            state.set(info);
        })
    };

    let on_is_customer_change = {
        let state = request.clone();
        Callback::from(move |e: Event| {
            let mut info = (*state).clone();
            info.is_customer = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .checked();
            state.set(info);
        })
    };

    let on_ap_account_change = {
        let state = request.clone();
        Callback::from(move |e: Event| {
            let mut info = (*state).clone();
            let value = e
                .target_unchecked_into::<web_sys::HtmlSelectElement>()
                .value();
            info.default_ap_account_id = Uuid::parse_str(&value).ok();
            state.set(info);
        })
    };

    let on_ar_account_change = {
        let state = request.clone();
        Callback::from(move |e: Event| {
            let mut info = (*state).clone();
            let value = e
                .target_unchecked_into::<web_sys::HtmlSelectElement>()
                .value();
            info.default_ar_account_id = Uuid::parse_str(&value).ok();
            state.set(info);
        })
    };

    let on_form_submit = {
        let on_submit = props.on_submit.clone();
        let request = request.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            on_submit.emit((*request).clone());
        })
    };

    let on_overlay_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |_e: MouseEvent| {
            on_close.emit(());
        })
    };

    let on_cancel_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |_e: MouseEvent| {
            on_close.emit(());
        })
    };

    html! {
        <div class="modal-overlay" onclick={on_overlay_click}>
            <div class="modal-content" onclick={|e: MouseEvent| e.stop_propagation()}>
                <h2>{ "Add New Partner" }</h2>
                <form onsubmit={on_form_submit} class="modal__form">
                    <label>{"Legal Name:"}</label>
                    <input type="text" oninput={on_legal_name_input} required=true />

                    <label>{"Trade Name:"}</label>
                    <input type="text" oninput={on_trade_name_input} />

                    <label>{"Tax Identifier:"}</label>
                    <input type="text" oninput={on_tax_identifier_input} />

                    <label>{"Is Vendor:"}</label>
                    <input type="checkbox" onchange={on_is_vendor_change} />

                    <label>{"Is Customer:"}</label>
                    <input type="checkbox" onchange={on_is_customer_change} />

                    <label>{"Default AP Account:"}</label>
                    <select onchange={on_ap_account_change}>
                        <option value="" selected=true>{ "None" }</option>
                        { for props.ap_accounts.iter().map(|(id, name)| html! {
                            <option value={id.to_string()}>{name}</option>
                        })}
                    </select>

                    <label>{"Default AR Account:"}</label>
                    <select onchange={on_ar_account_change}>
                        <option value="" selected=true>{ "None" }</option>
                        { for props.ar_accounts.iter().map(|(id, name)| html! {
                            <option value={id.to_string()}>{name}</option>
                        })}
                    </select>

                    <div class="modal__form__actions">
                        <button type="button" onclick={on_cancel_click} class="button-secondary">{ "Cancel" }</button>
                        <button type="submit">{ "Add Partner" }</button>
                    </div>
                    if let Some(err) = &*error {
                        <div class="error">{ err }</div>
                    }
                </form>
            </div>
        </div>
    }
}
