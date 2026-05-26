/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
pub mod info;

use icu_calendar::Date;
use icu_datetime::{options::length, DateFormatter};
use icu_decimal::FixedDecimalFormatter;
use icu_locid::{locale, Locale};
use fixed_decimal::FixedDecimal;

/// Formats standard cents integers (i64) into localized decimals.
/// e.g., 123456 -> "1,234.56" (en-AU) or "1 234,56" (fr-FR)
pub fn format_currency_icu(amount_cents: i64, target_locale: Option<&str>) -> String {
    // Parse targeted locale string or default dynamically to en-AU
    let locale_ident: Locale = target_locale
        .and_then(|l| l.parse::<Locale>().ok())
        .unwrap_or_else(|| locale!("en-AU"));

    // Instantiate localized decimal formatter
    let formatter = FixedDecimalFormatter::try_new(&locale_ident.into(), Default::default())
        .expect("Failed to initialize ICU4X Decimal Formatter");

    // Convert i64 cents to an explicit fixed-point scale representation
    // to bypass potential floating-point rounding precision errors in accounting logs
    let mut decimal = FixedDecimal::from(amount_cents);
    decimal.multiply_pow10(-2);

    let formatted_buffer = formatter.format_to_string(&decimal);

    formatted_buffer
}

/// Specialized wrapper for Typst reporting layouts
pub fn format_currency_icu_typ(amount_cents: i64, target_locale: Option<&str>) -> String {
    let formatted = format_currency_icu(amount_cents, target_locale);

    // Safety check: In Typst, a leading standard hyphen can interpret as an unintended
    // structural markdown list element. Map seamlessly to the clean minus sign (U+2212).
    if formatted.starts_with('-') {
        format!("−{}", &formatted[1..])
    } else {
        formatted
    }
}

/// Formats naive calendar variables into human-readable regional text streams
/// e.g., (2026, 5, 25) -> "25 May 2026"
pub fn format_date_icu(year: i32, month: u32, day: u32, target_locale: Option<&str>) -> String {
    let locale_ident: Locale = target_locale
        .and_then(|l| l.parse::<Locale>().ok())
        .unwrap_or_else(|| locale!("en-AU"));

    // Initialize the thread-safe date compiler using native ISO calendar layout maps
    let date_formatter = DateFormatter::try_new_with_length(
        &locale_ident.into(),
        length::Date::Medium,
    )
    .expect("Failed to construct ICU4X DateTime engine configuration context");

    // Build the isolated date object structure
    let date_object = Date::try_new_iso_date(year, month as u8, day as u8)
        .expect("Invalid Date integer bounds provided to ledger component system");

    // Format immediately to a native safe string
    date_formatter.format_to_string(&date_object.to_any()).expect("Failed to format date")
}

#[cfg(test)]
mod icu_integration_tests {
    use crate::util::{format_currency_icu, format_currency_icu_typ};

    #[test]
    fn verify_currency_localization() {
        // Test standard Australian/UK notation structures
        assert_eq!(format_currency_icu(123456, Some("en-AU")), "1,234.56");
        assert_eq!(format_currency_icu(1, Some("en-AU")), "0.01");
        assert_eq!(format_currency_icu(0, Some("en-AU")), "0.00");

        // Test Typst negative rendering protection
        assert_eq!(format_currency_icu_typ(-123456, Some("en-AU")), "−1,234.56");
    }

    // #[test]
    // fn verify_date_localization() {
    //     // Default target confirmation (25 May 2026)
    //     let au_date = format_date(2026, 5, 25, Some("en-AU"));
    //     assert_eq!(au_date, "25 May 2026");
    //
    //     // Verify alternative regional locales map structural layouts automatically
    //     let us_date = format_date(2026, 5, 25, Some("en-US"));
    //     assert_eq!(us_date, "May 25, 2026");
    // }
}