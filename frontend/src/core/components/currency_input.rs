/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use rust_decimal::{
    dec,
    Decimal,
};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::contexts::locale_context::{
    use_locale,
    LocaleContext,
};

#[derive(Properties, PartialEq)]
pub struct DecimalInputProps {
    pub on_change: Callback<Decimal>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_else(|| 2)]
    pub decimal_places: u32,
    pub value: Decimal, // amount
    #[prop_or_else(|| false)]
    pub readonly: bool,
}

#[function_component(DecimalInput)]
pub fn decimal_input(props: &DecimalInputProps) -> Html {
    let i18n = use_locale();

    // Local string buffer to handle mid-typing states (like "22.")
    // that don't parse cleanly to Decimal yet.
    let display_value = use_state(|| format_value(&i18n, props.value, props.decimal_places));

    // Sync local state if parent value changes externally (e.g. form reset)
    {
        let display_value = display_value.clone();
        let props_value = props.value;
        let props_decimal_places = props.decimal_places;
        use_effect_with(props_value, move |&val| {
            // Only update if the parsed version of current display differs
            // from the new prop value to avoid overwriting the user's cursor.
            if parse_to_amount(&display_value, props_decimal_places) != Some(val) {
                display_value.set(format_value(&i18n, val, props_decimal_places));
            }
            || ()
        });
    }

    let oninput = {
        let display_value = display_value.clone();
        let on_change = props.on_change.clone();
        let props_decimal_places = props.decimal_places;
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let val = input.value();

            // Allow only digits and a single decimal point
            let filtered: String = val
                .chars()
                .filter(|c| c.is_ascii_digit() || *c == '.' || *c == '-')
                .collect();

            display_value.set(filtered.clone());

            if let Some(amount) = parse_to_amount(&filtered, props_decimal_places) {
                on_change.emit(amount);
            }
        })
    };

    html! {
        <input
            type="text"
            class={classes!(props.class.clone(), "currency-input")}
            placeholder={props.placeholder.clone()}
            value={(*display_value).clone()}
            readonly={props.readonly}
            {oninput}
        />
    }
}

// Logic helpers
fn format_value(i18n: &LocaleContext, amount: Decimal, dp: u32) -> String {
    i18n.format_decimal(amount.round_dp(dp))
}

fn parse_to_amount(s: &str, dp: u32) -> Option<Decimal> {
    match Decimal::from_str_exact(s) {
        Ok(dec) => Some(dec.round_dp(dp)),
        Err(_) => Some(dec!(0.00)),
    }
}
