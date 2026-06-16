/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use std::rc::Rc;
use yew::prelude::*;
use shared_core::sales::models::item::ItemType;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ItemFilterState {
    pub search_term: String,
    pub item_type: Option<ItemType>,
    pub include_inactive: bool,
    pub limit: u32,
}

pub enum ItemFilterAction {
    SetSearchTerm(String),
    SetItemType(Option<ItemType>),
    SetIncludeInactive(bool),
    SetLimit(u32),
    IncrementLimit,
}

impl Reducible for ItemFilterState {
    type Action = ItemFilterAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut next_state = (*self).clone();
        match action {
            ItemFilterAction::SetSearchTerm(term) => next_state.search_term = term,
            ItemFilterAction::SetItemType(item_type) => next_state.item_type = item_type,
            ItemFilterAction::SetIncludeInactive(include) => next_state.include_inactive = include,
            ItemFilterAction::SetLimit(limit) => next_state.limit = limit,
            ItemFilterAction::IncrementLimit => next_state.limit += 20,
        }
        next_state.into()
    }
}

pub type ItemFilterContext = UseReducerHandle<ItemFilterState>;

#[derive(Properties, PartialEq)]
pub struct ItemFilterProviderProps {
    pub children: Children,
}

#[function_component(ItemFilterProvider)]
pub fn item_filter_provider(props: &ItemFilterProviderProps) -> Html {
    let filter_state = use_reducer(|| ItemFilterState {
        limit: 20,
        ..Default::default()
    });

    html! {
        <ContextProvider<ItemFilterContext> context={filter_state}>
            {props.children.clone()}
        </ContextProvider<ItemFilterContext>>
    }
}

#[hook]
pub fn use_item_filter() -> ItemFilterContext {
    use_context::<ItemFilterContext>().expect("No ItemFilterContext found")
}
