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

use shared_core::requests::partner::UpdatePartnerRequest;
use yew::prelude::*;
use uuid::Uuid;

#[derive(Properties, PartialEq)]
pub struct GeneralViewProps {
    pub request: UpdatePartnerRequest,
    pub on_change: Callback<UpdatePartnerRequest>,
    pub ap_accounts: Vec<(Uuid, String)>,
    pub ar_accounts: Vec<(Uuid, String)>,
}

#[function_component(GeneralView)]
pub fn general_view(props: &GeneralViewProps) -> Html {
    let on_input = |field_updater: fn(&mut UpdatePartnerRequest, String)| {
        let on_change = props.on_change.clone();
        let request = props.request.clone();
        Callback::from(move |e: InputEvent| {
            let mut req = request.clone();
            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
            field_updater(&mut req, value);
            on_change.emit(req);
        })
    };

    let on_checkbox_change = |field_updater: fn(&mut UpdatePartnerRequest, bool)| {
        let on_change = props.on_change.clone();
        let request = props.request.clone();
        Callback::from(move |e: Event| {
            let mut req = request.clone();
            let checked = e.target_unchecked_into::<web_sys::HtmlInputElement>().checked();
            field_updater(&mut req, checked);
            on_change.emit(req);
        })
    };

    let on_select_change = |field_updater: fn(&mut UpdatePartnerRequest, Option<Uuid>)| {
        let on_change = props.on_change.clone();
        let request = props.request.clone();
        Callback::from(move |e: Event| {
            let mut req = request.clone();
            let value = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
            let uuid = if value.is_empty() { None } else { Uuid::parse_str(&value).ok() };
            field_updater(&mut req, uuid);
            on_change.emit(req);
        })
    };

    html! {
        <form class="modal__form">
            <label>{"Legal Name:"}</label>
            <input type="text" value={props.request.legal_name.clone()} oninput={on_input(|r, v| r.legal_name = v)} required=true />

            <label>{"Trade Name:"}</label>
            <input type="text" value={props.request.trade_name.clone().unwrap_or_default()} oninput={on_input(|r, v| r.trade_name = Some(v))} />

            <label>{"Tax Identifier:"}</label>
            <input type="text" value={props.request.tax_identifier.clone().unwrap_or_default()} oninput={on_input(|r, v| r.tax_identifier = Some(v))} />

            <label>{"Is Vendor:"}</label>
            <input type="checkbox" checked={props.request.is_vendor} onchange={on_checkbox_change(|r, v| r.is_vendor = v)} />

            <label>{"Is Customer:"}</label>
            <input type="checkbox" checked={props.request.is_customer} onchange={on_checkbox_change(|r, v| r.is_customer = v)} />

            <label>{"Default AP Account:"}</label>
            <select onchange={on_select_change(|r, v| r.default_ap_account_id = v)}>
                <option value="" selected={props.request.default_ap_account_id.is_none()}>{ "None" }</option>
                { for props.ap_accounts.iter().map(|(id, name)| html! {
                    <option value={id.to_string()} selected={props.request.default_ap_account_id == Some(*id)}>{name}</option>
                })}
            </select>

            <label>{"Default AR Account:"}</label>
            <select onchange={on_select_change(|r, v| r.default_ar_account_id = v)}>
                <option value="" selected={props.request.default_ar_account_id.is_none()}>{ "None" }</option>
                { for props.ar_accounts.iter().map(|(id, name)| html! {
                    <option value={id.to_string()} selected={props.request.default_ar_account_id == Some(*id)}>{name}</option>
                })}
            </select>
        </form>
    }
}
