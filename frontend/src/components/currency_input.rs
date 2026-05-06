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

use yew::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct CurrencyProps {
    pub value: i64, // Cents
    pub on_change: Callback<i64>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub placeholder: String,
}

#[function_component(CurrencyInput)]
pub fn currency_input(props: &CurrencyProps) -> Html {
    // Local string buffer to handle mid-typing states (like "22.")
    // that don't parse cleanly to i64 yet.
    let display_value = use_state(|| format_cents(props.value));

    // Sync local state if parent value changes externally (e.g. form reset)
    {
        let display_value = display_value.clone();
        let props_value = props.value;
        use_effect_with(props_value, move |&val| {
            // Only update if the parsed version of current display differs
            // from the new prop value to avoid overwriting the user's cursor.
            if parse_to_cents(&display_value) != Some(val) {
                display_value.set(format_cents(val));
            }
            || ()
        });
    }

    let oninput = {
        let display_value = display_value.clone();
        let on_change = props.on_change.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let val = input.value();

            // Allow only digits and a single decimal point
            let filtered: String = val.chars()
                .filter(|c| c.is_ascii_digit() || *c == '.')
                .collect();

            display_value.set(filtered.clone());

            if let Some(cents) = parse_to_cents(&filtered) {
                on_change.emit(cents);
            }
        })
    };

    html! {
        <input
            type="text"
            class={classes!(props.class.clone(), "currency-input")}
            placeholder={props.placeholder.clone()}
            value={(*display_value).clone()}
            {oninput}
        />
    }
}

// Logic helpers (keep in the same file or a separate logic util)
fn format_cents(cents: i64) -> String {
    let dollars = cents / 100;
    let fractional = (cents % 100).abs();
    format!("{}.{:02}", dollars, fractional)
}

fn parse_to_cents(s: &str) -> Option<i64> {
    if s.is_empty() { return Some(0); }
    let parts: Vec<&str> = s.split('.').collect();
    match parts.as_slice() {
        [d] => d.parse::<i64>().ok().map(|v| v * 100),
        [d, c] => {
            let d_val = d.parse::<i64>().unwrap_or(0);
            let mut c_str = c.to_string();
            c_str.push_str("00");
            let c_val = c_str[..2].parse::<i64>().unwrap_or(0);
            Some(d_val * 100 + c_val)
        }
        _ => None,
    }
}