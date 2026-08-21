/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 * (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use shared_core::{
    core::models::auth::SystemPrivilege,
    inventory::models::warehouse::WarehouseLocation,
};
use yew::prelude::*;

use crate::{
    contexts::{
        auth_context::use_user_context,
        locale_context::use_locale,
    },
    inventory::components::bulk_generator_modal::BulkGeneratorModal,
};

#[derive(Properties, PartialEq, Clone)]
pub struct GridProps {
    pub warehouse_id: uuid::Uuid,
    pub locations: Vec<WarehouseLocation>,
    pub zone: Option<String>,
    pub aisle: Option<String>,
    pub on_refresh: Callback<()>,
}

#[function_component(LocationGrid)]
pub fn location_grid(props: &GridProps) -> Html {
    let i18n = use_locale();
    let user_ctx = use_user_context();
    let show_bulk_modal = use_state(|| false);

    // Apply tree drilldown state boundaries
    let filtered_locations: Vec<&WarehouseLocation> = props
        .locations
        .iter()
        .filter(|loc| {
            props
                .zone
                .as_ref()
                .map_or(true, |z| loc.zone.as_ref() == Some(z))
        })
        .filter(|loc| {
            props
                .aisle
                .as_ref()
                .map_or(true, |a| loc.aisle.as_ref() == Some(a))
        })
        .collect();

    let open_bulk = {
        let show_bulk_modal = show_bulk_modal.clone();
        Callback::from(move |_| show_bulk_modal.set(true))
    };

    let close_bulk = {
        let show_bulk_modal = show_bulk_modal.clone();
        Callback::from(move |_: ()| show_bulk_modal.set(false))
    };

    let on_bulk_submit = {
        let show_bulk_modal = show_bulk_modal.clone();
        let on_refresh = props.on_refresh.clone();
        Callback::from(move |_: ()| {
            show_bulk_modal.set(false);
            on_refresh.emit(());
        })
    };

    html! {
        <div class="location-grid-pane">
            <div class="pane-action-bar">
                <div class="pane-breadcrumbs">
                    <span>{ props.zone.as_deref().unwrap_or("All Zones") }</span>
                    if let Some(a) = &props.aisle {
                        <span class="divider">{ " > " }</span>
                        <span>{ format!("Aisle {}", a) }</span>
                    }
                </div>

                if user_ctx.has_privilege(&SystemPrivilege::ManageInventory) {
                    <div class="action-buttons">
                        <button onclick={open_bulk} class="button-primary-outline">{ i18n.t("location-btn-bulk-generate") }</button>
                    </div>
                }
            </div>

            <table class="table dense-grid">
                <thead>
                    <tr>
                        <th class="table__text-col">{ i18n.t("location-grid-label") }</th>
                        <th class="table__text-col">{ i18n.t("location-grid-zone") }</th>
                        <th class="table__text-col">{ i18n.t("location-grid-aisle") }</th>
                        <th class="table__text-col">{ i18n.t("location-grid-shelf") }</th>
                        <th class="table__text-col">{ i18n.t("location-grid-bin") }</th>
                        <th class="table__text-col">{ i18n.t("location-grid-picking") }</th>
                    </tr>
                </thead>
                <tbody>
                    { for filtered_locations.into_iter().map(|loc| html! {
                        <tr>
                            <td class="table__text-col">{ &loc.display_label }</td>
                            <td class="table__text-col">{ &loc.zone.clone().unwrap_or("".to_string()) }</td>
                            <td class="table__text-col">{ &loc.aisle.clone().unwrap_or("".to_string()) }</td>
                            <td class="table__text-col">{ &loc.shelf.clone().unwrap_or("".to_string()) }</td>
                            <td class="table__text-col">{ &loc.bin.clone().unwrap_or("".to_string()) }</td>
                            <td class="table__text-col">
                                <input type="checkbox" checked={loc.is_picking_location} disabled=true />
                            </td>
                        </tr>
                    })}
                </tbody>
            </table>

            if *show_bulk_modal {
                <BulkGeneratorModal
                    warehouse_id={props.warehouse_id}
                    on_close={close_bulk}
                    on_submit={on_bulk_submit}
                />
            }
        </div>
    }
}
