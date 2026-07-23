/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 * (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use fluent::fluent_args;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use shared_core::inventory::models::warehouse::{Warehouse, WarehouseLocation};
use crate::{
    api::Api,
    contexts::{auth_context::use_user_context, locale_context::use_locale},
    core::components::layout::Layout,
    inventory::components::{
        location_grid::LocationGrid,
        location_tree::LocationTree,
    },
};

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub warehouse_id: uuid::Uuid,
}

#[function_component(WarehouseLocationsPage)]
pub fn warehouse_locations_page(props: &Props) -> Html {
    let i18n = use_locale();
    let user_ctx = use_user_context();
    let navigator = use_navigator().unwrap();

    let warehouse = use_state(|| None::<Warehouse>);
    let locations = use_state(Vec::<WarehouseLocation>::new);
    let selected_zone = use_state(|| None::<String>);
    let selected_aisle = use_state(|| None::<String>);
    let refresh_trigger = use_state(|| 0);

    // Fetch the active master Warehouse details
    {
        let warehouse = warehouse.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        let warehouse_id = props.warehouse_id;
        use_effect_with(warehouse_id, move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(resp) = Api::get(&format!("/api/inventory/warehouses/{}", warehouse_id), user_ctx, navigator).await {
                    if let Ok(data) = resp.json::<Warehouse>().await {
                        warehouse.set(Some(data));
                    }
                }
            });
            || ()
        });
    }

    // Fetch all storage slots under this specific hub
    {
        let locations = locations.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        let warehouse_id = props.warehouse_id;
        let refresh = *refresh_trigger;
        use_effect_with((warehouse_id, refresh), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(resp) = Api::get(&format!("/api/inventory/warehouses/{}/locations", warehouse_id), user_ctx, navigator).await {
                    if let Ok(data) = resp.json::<Vec<WarehouseLocation>>().await {
                        locations.set(data);
                    }
                }
            });
            || ()
        });
    }

    let on_tree_select = {
        let selected_zone = selected_zone.clone();
        let selected_aisle = selected_aisle.clone();
        Callback::from(move |(zone, aisle): (Option<String>, Option<String>)| {
            selected_zone.set(zone);
            selected_aisle.set(aisle);
        })
    };

    let on_action_complete = {
        let refresh_trigger = refresh_trigger.clone();
        Callback::from(move |_: ()| {
            refresh_trigger.set(*refresh_trigger + 1);
        })
    };

    let wh_name = warehouse.as_ref().map(|w| w.name.clone()).unwrap_or_default();

    html! {
    <Layout>
        <div class="locations-page-container">
            <h1>{ i18n.t_args("warehouse-locations-title", &fluent_args!["name" => wh_name]) }</h1>

            <div class="locations-workspace">
                <aside class="locations-sidebar">
                        <LocationTree
                            locations={(*locations).clone()}
                            on_select={on_tree_select}
                        />
                </aside>
                <main class="locations-main-content">
                        <LocationGrid
                            warehouse_id={props.warehouse_id}
                            locations={(*locations).clone()}
                            zone={(*selected_zone).clone()}
                            aisle={(*selected_aisle).clone()}
                            on_refresh={on_action_complete}
                        />
                </main>
            </div>
        </div>
    </Layout>
  }
}

