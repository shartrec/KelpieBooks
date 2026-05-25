/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use shared_core::dtos::partner_list_item::PartnerListItem;
use shared_core::i18n::t;
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
        t("partner-row-vendor-customer")
    } else if props.partner.is_vendor {
        t("common-vendor")
    } else if props.partner.is_customer {
        t("common-customer")
    } else {
        t("common-none")
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
                        <img src="/images/view.svg" alt={t("common-view")} />
                    </button>
                    <button class="icon-button btn-action" onclick={on_delete} disabled={!props.partner.can_delete}>
                        <img src="/images/delete.svg" alt={t("common-delete")} />
                    </button>
                </div>
            </td>
        </tr>
    }
}
