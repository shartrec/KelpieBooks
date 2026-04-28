/*
 * Copyright (c) 2026-2026. Trevor Campbell and others.
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

use shared_core::models::AccountCategory;
use shared_core::requests::account::CreateAccountRequest;
use std::str::FromStr;
use uuid::Uuid;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AddAccountModalProps {
    pub on_close: Callback<()>,
    pub on_submit: Callback<CreateAccountRequest>,
    pub parent_accounts: Vec<(Uuid, String)>, // Vec of (id, name) for the dropdown
}

#[function_component(AddAccountModal)]
pub fn add_account_modal(props: &AddAccountModalProps) -> Html {
    let request = use_state(CreateAccountRequest::default);
    let error = use_state(|| None::<String>);

    let on_code_input = {
        let state = request.clone();
        Callback::from(move |e: InputEvent| {
            let mut info = (*state).clone();
            info.code = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            state.set(info);
        })
    };
    let on_name_input = {
        let state = request.clone();
        Callback::from(move |e: InputEvent| {
            let mut info = (*state).clone();
            info.name = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            state.set(info);
        })
    };
    let on_category_change = {
        let state = request.clone();
        Callback::from(move |e: Event| {
            let mut info = (*state).clone();
            info.category = AccountCategory::from_str(
                &e.target_unchecked_into::<web_sys::HtmlSelectElement>()
                    .value(),
            )
            .unwrap();
            state.set(info);
        })
    };
    let on_parent_change = {
        let state = request.clone();
        Callback::from(move |e: Event| {
            let mut info = (*state).clone();
            let val = e
                .target_unchecked_into::<web_sys::HtmlSelectElement>()
                .value();
            info.parent_id = if val.is_empty() {
                None
            } else {
                Uuid::from_str(&val).ok()
            };
            state.set(info);
        })
    };
    let on_is_group_change = {
        let state = request.clone();
        Callback::from(move |e: Event| {
            let mut info = (*state).clone();
            info.is_group = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .checked();
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
                <h2>{ "Add New Account" }</h2>
                <form onsubmit={on_form_submit} class="modal-form">
                    <label>{"Code:"}</label>
                    <input type="text" oninput={on_code_input} required=true />

                    <label>{"Name:"}</label>
                    <input type="text" oninput={on_name_input} required=true />

                    <label>{"Category:"}</label>
                    <select onchange={on_category_change}>
                        <option value="Asset" selected=true>{ "Asset" }</option>
                        <option value="Liability">{ "Liability" }</option>
                        <option value="Equity">{ "Equity" }</option>
                        <option value="Revenue">{ "Revenue" }</option>
                        <option value="Expense">{ "Expense" }</option>
                    </select>

                    <label>{"Parent Account:"}</label>
                    <select onchange={on_parent_change}>
                        <option value="">{ "None (Root Account)" }</option>
                        { for props.parent_accounts.iter().map(|(id, name)| html! {
                            <option value={id.to_string()}>{name}</option>
                        })}
                    </select>

                    <label>{"Is Group:"}</label>
                    <input type="checkbox" onchange={on_is_group_change} />

                    <div class="form-actions">
                        <button type="button" onclick={on_cancel_click} class="button-secondary">{ "Cancel" }</button>
                        <button type="submit">{ "Add Account" }</button>
                    </div>
                    if let Some(err) = &*error {
                        <div class="error">{ err }</div>
                    }
                </form>
            </div>
        </div>
    }
}
