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

use shared_core::dtos::account_with_balance::AccountWithBalance;
use shared_core::models::account_category::AccountCategory;
use shared_core::requests::account::UpdateAccountRequest;
use std::str::FromStr;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EditAccountModalProps {
    pub account: AccountWithBalance,
    pub on_close: Callback<()>,
    pub on_submit: Callback<UpdateAccountRequest>,
}

#[function_component(EditAccountModal)]
pub fn edit_account_modal(props: &EditAccountModalProps) -> Html {
    let request = use_state(|| UpdateAccountRequest {
        name: props.account.name.clone(),
        code: props.account.code.clone(),
        category: props.account.category,
        is_group: props.account.is_group,
        is_bank_account: props.account.is_bank_account,
        system_tag: props.account.system_tag,
    });

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

    let on_is_bank_account_change = {
        let state = request.clone();
        Callback::from(move |e: Event| {
            let mut info = (*state).clone();
            info.is_bank_account = e
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

    let on_cancel = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| {
            on_close.emit(());
        })
    };

    html! {
        <div class="modal-overlay" onclick={on_cancel.clone()}>
            <div class="modal-content" onclick={|e: MouseEvent| e.stop_propagation()}>
                <h2>{ "Edit Account" }</h2>
                <form onsubmit={on_form_submit} class="modal__form">
                    <label>{"Code:"}</label>
                    <input type="text" value={request.code.clone()} oninput={on_code_input} required=true />

                    <label>{"Name:"}</label>
                    <input type="text" value={request.name.clone()} oninput={on_name_input} required=true />

                    <label>{"Category:"}</label>
                    <select onchange={on_category_change}>
                        <option value="Asset" selected={request.category == AccountCategory::Asset}>{ "Asset" }</option>
                        <option value="Liability" selected={request.category == AccountCategory::Liability}>{ "Liability" }</option>
                        <option value="Equity" selected={request.category == AccountCategory::Equity}>{ "Equity" }</option>
                        <option value="Revenue" selected={request.category == AccountCategory::Revenue}>{ "Revenue" }</option>
                        <option value="Expense" selected={request.category == AccountCategory::Expense}>{ "Expense" }</option>
                    </select>

                    <label>{"Is Group:"}</label>
                    <input type="checkbox" checked={request.is_group} onchange={on_is_group_change} />

                    <label>{"Is Bank Account:"}</label>
                    <input type="checkbox" checked={request.is_bank_account} onchange={on_is_bank_account_change} />

                    <div class="modal__form__actions">
                        <button type="button" onclick={on_cancel} class="button-secondary">{ "Cancel" }</button>
                        <button type="submit">{ "Save Changes" }</button>
                    </div>
                </form>
            </div>
        </div>
    }
}
