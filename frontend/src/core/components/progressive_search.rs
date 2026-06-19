/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use web_sys::HtmlInputElement;
use yew::prelude::*;
use super::SearchableItem;

#[derive(Properties, PartialEq)]
pub struct Props<T: SearchableItem> {
    /// The current text value typed into the input field
    pub query: String,
    /// The filtered matching results passed down by the parent container
    pub suggestions: Vec<T>,
    /// Placeholder text for the input box
    pub placeholder: String,
    /// Callback triggered whenever the text input changes
    pub on_input: Callback<String>,
    /// Callback triggered when a selection is finalized (via Click or Enter key)
    pub on_select: Callback<T>,
}

#[function_component(ProgressiveSearch)]
pub fn progressive_search<T: SearchableItem>(props: &Props<T>) -> Html {
    // Tracks which suggestion index is currently highlighted by the keyboard arrow keys
    let active_index = use_state(|| -1_i32);
    let dropdown_open = use_state(|| false);

    // Reset keyboard highlight whenever the suggestions list changes
    {
        let active_index = active_index.clone();
        use_effect_with(props.suggestions.clone(), move |_| {
            active_index.set(-1);
            || ()
        });
    }

    // Handle standard text input changes
    let oninput = {
        let on_input = props.on_input.clone();
        let dropdown_open = dropdown_open.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            dropdown_open.set(!value.is_empty());
            on_input.emit(value);
        })
    };

    // Intercept Keyboard Navigation keys
    let onkeydown = {
        let suggestions = props.suggestions.clone();
        let active_index = active_index.clone();
        let on_select = props.on_select.clone();
        let dropdown_open = dropdown_open.clone();

        Callback::from(move |e: KeyboardEvent| {
            if suggestions.is_empty() || !*dropdown_open {
                return;
            }

            let max_idx = (suggestions.len() as i32) - 1;
            let current_idx = *active_index;

            match e.key().as_str() {
                "ArrowDown" => {
                    e.prevent_default(); // Prevent the text cursor from jumping around
                    let next_idx = if current_idx >= max_idx { 0 } else { current_idx + 1 };
                    active_index.set(next_idx);
                }
                "ArrowUp" => {
                    e.prevent_default();
                    let next_idx = if current_idx <= 0 { max_idx } else { current_idx - 1 };
                    active_index.set(next_idx);
                }
                "Enter" => {
                    if current_idx >= 0 && current_idx <= max_idx {
                        e.prevent_default();
                        let selected_item = &suggestions[current_idx as usize];
                        on_select.emit(selected_item.clone());
                        dropdown_open.set(false);
                    }
                }
                "Escape" => {
                    e.prevent_default();
                    dropdown_open.set(false);
                }
                _ => {}
            }
        })
    };

    // Handle blurring out the component safely
    let onblur = {
        let dropdown_open = dropdown_open.clone();
        Callback::from(move |_| {
            // 💡 Timeout prevents the dropdown from instantly vanishing
            // before a mouse click registers on a list element.
            let dropdown_open = dropdown_open.clone();
            gloo_timers::callback::Timeout::new(150, move || {
                dropdown_open.set(false);
            }).forget();
        })
    };

    let onfocus = {
        let dropdown_open = dropdown_open.clone();
        let query = props.query.clone();
        Callback::from(move |_| {
            if !query.is_empty() {
                dropdown_open.set(true);
            }
        })
    };

    html! {
        <div class="progressive-search" onblur={onblur} onfocus={onfocus}>
            <input
                type="text"
                class="form-input"
                placeholder={props.placeholder.clone()}
                value={props.query.clone()}
                {oninput}
                {onkeydown}
                autocomplete="off"
            />

            if *dropdown_open && !props.suggestions.is_empty() {
                <ul class="suggestions">
                    {for props.suggestions.iter().enumerate().map(|(idx, item)| {
                        let item_clone = item.clone();
                        let on_click_select = props.on_select.clone();
                        let dropdown_close = dropdown_open.clone();

                        // Apply the matching active class from your SCSS file
                        let is_active = (idx as i32) == *active_index;
                        let class = if is_active { "active" } else { "" };

                        html! {
                            <li
                                {class}
                                onmousedown={Callback::from(move |_| {
                                    on_click_select.emit(item_clone.clone());
                                    dropdown_close.set(false);
                                })}
                            >
                                { item.display_label() }
                                if let Some(subtitle) = item.subtitle() {
                                    <span class="search-subtitle">{ subtitle }</span>
                                }
                            </li>
                        }
                    })}
                </ul>
            }
        </div>
    }
}