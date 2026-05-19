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

use shared_core::dtos::partner_list_item::PartnerListItem;
use uuid::Uuid;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PartnerRowProps {
    pub partner: PartnerListItem,
    pub on_edit: Callback<Uuid>,
    pub on_delete: Callback<PartnerListItem>,
}

#[function_component(PartnerRow)]
pub fn partner_row(props: &PartnerRowProps) -> Html {
    let partner_type = if props.partner.is_vendor && props.partner.is_customer {
        "Vendor & Customer".to_string()
    } else if props.partner.is_vendor {
        "Vendor".to_string()
    } else if props.partner.is_customer {
        "Customer".to_string()
    } else {
        "None".to_string()
    };

    let on_edit = {
        let on_edit = props.on_edit.clone();
        let partner_id = props.partner.id;
        Callback::from(move |_| {
            on_edit.emit(partner_id);
        })
    };

    let on_delete = {
        let on_delete = props.on_delete.clone();
        let partner = props.partner.clone();
        Callback::from(move |_| {
            on_delete.emit(partner.clone());
        })
    };

    html! {
        <tr>
            <td>{ &props.partner.legal_name }</td>
            <td>{ props.partner.trade_name.as_deref().unwrap_or("") }</td>
            <td>{ partner_type }</td>
            <td class="table__col-actions">
                <div class="actions-wrapper">
                    <button class="icon-button btn-action" onclick={on_edit}>
                        <img src="/images/edit.svg" alt="Edit" />
                    </button>
                    <button class="icon-button btn-action" onclick={on_delete}>
                        <img src="/images/delete.svg" alt="Delete" />
                    </button>
                </div>
            </td>
        </tr>
    }
}
