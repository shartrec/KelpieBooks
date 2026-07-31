/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 * (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use std::collections::{
    BTreeMap,
    BTreeSet,
};

use shared_core::inventory::models::warehouse::WarehouseLocation;
use yew::prelude::*;

use crate::contexts::locale_context::use_locale;

#[derive(Properties, PartialEq, Clone)]
pub struct TreeProps {
    pub locations: Vec<WarehouseLocation>,
    pub on_select: Callback<(Option<String>, Option<String>)>,
}

#[function_component(LocationTree)]
pub fn location_tree(props: &TreeProps) -> Html {
    let i18n = use_locale();
    let expanded_zones = use_state(BTreeSet::<String>::new);
    let active_node = use_state(|| (None::<String>, None::<String>));

    // Construct hierarchy: Zone -> Unique Aisles
    let mut topography: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for loc in &props.locations {
        topography
            .entry(loc.zone.clone())
            .or_default()
            .insert(loc.aisle.clone());
    }

    let make_select_callback = |zone: Option<String>, aisle: Option<String>| {
        let on_select = props.on_select.clone();
        let active_node = active_node.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            active_node.set((zone.clone(), aisle.clone()));
            on_select.emit((zone.clone(), aisle.clone()));
        })
    };

    let toggle_zone = |zone: String| {
        let expanded = expanded_zones.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            let mut set = (*expanded).clone();
            if set.contains(&zone) {
                set.remove(&zone);
            } else {
                set.insert(zone.clone());
            }
            expanded.set(set);
        })
    };

    let is_node_active = |z: &Option<String>, a: &Option<String>| {
        if *active_node == (z.clone(), a.clone()) {
            "tree-node--active"
        } else {
            ""
        }
    };

    html! {
        <div class="location-tree">
            <h3>{ i18n.t("location-tree-header") }</h3>
            <ul class="tree-root">
                <li class={classes!("tree-node", is_node_active(&None, &None))} onclick={make_select_callback(None, None)}>
                    <span class="tree-label icon-warehouse">{ i18n.t("location-tree-all-view") }</span>
                </li>

                { for topography.into_iter().map(|(zone, aisles)| {
                    let is_expanded = expanded_zones.contains(&zone);
                    let zone_clone = zone.clone();

                    html! {
                        <li class="tree-branch">
                            <div class={classes!("tree-node", is_node_active(&Some(zone.clone()), &None))} onclick={make_select_callback(Some(zone.clone()), None)}>
                                <button class={classes!("tree-toggle", if is_expanded { "expanded" } else { "" })} onclick={toggle_zone(zone_clone)} />
                                <span class="tree-label icon-zone">{ &zone }</span>
                            </div>

                            if is_expanded {
                                <ul class="tree-sub-branch">
                                    { for aisles.into_iter().map(|aisle| {
                                        let z = Some(zone.clone());
                                        let a = Some(aisle.clone());
                                        html! {
                                            <li class={classes!("tree-node", is_node_active(&z, &a))} onclick={make_select_callback(z, a)}>
                                                <span class="tree-label icon-aisle">{ format!("{} {}", i18n.t("location-tree-aisle-prefix"), aisle) }</span>
                                            </li>
                                        }
                                    })}
                                </ul>
                            }
                        </li>
                    }
                })}
            </ul>
        </div>
    }
}
