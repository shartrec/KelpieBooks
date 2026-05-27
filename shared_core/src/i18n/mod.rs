/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

//! Internationalisation module
use fixed_decimal::FixedDecimal;
use fluent::concurrent::FluentBundle;
use fluent::{FluentArgs, FluentError, FluentResource};
use icu_calendar::Date;
use icu_datetime::options::length;
use icu_datetime::DateFormatter;
use icu_decimal::FixedDecimalFormatter;
use icu_locid::{locale, Locale};
use include_dir::{include_dir, Dir};
use std::collections::HashMap;
use std::sync::LazyLock;
use unic_langid::langid;
use unic_langid::LanguageIdentifier;

// Define the directory where translation files are.
static TRANSLATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/translations");

// Create a lazy static instance of the I18nManager.
pub static I18N: LazyLock<I18nManager> = LazyLock::new(I18nManager::new);

// The main struct to manage all translations.
pub struct I18nManager {
    bundles: HashMap<LanguageIdentifier, FluentBundle<FluentResource>>,
    default_locale: LanguageIdentifier,
}

// shared_core/src/i18n/mod.rs

impl I18nManager {
    pub fn new() -> Self {
        let default_locale = langid!("en-AU");
        let mut bundles = HashMap::new();

        // 1. First pass: Load all base language files (e.g., "en", "fr", "es")
        // and initialize their concurrent bundles
        for file in TRANSLATIONS_DIR.files() {
            let path = file.path();
            if let Some(lang_str) = path.file_stem().and_then(|s| s.to_str()) {
                if let Ok(lang_id) = lang_str.parse::<LanguageIdentifier>() {
                    // Only process base languages without regions in the first pass (e.g., length == 1 or region is None)
                    if lang_id.region.is_none() {
                        let ftl_string = file.contents_utf8().unwrap();
                        let resource = FluentResource::try_new(ftl_string.to_string())
                            .expect("Failed to parse base FTL file.");

                        let mut bundle = FluentBundle::new_concurrent(vec![lang_id.clone()]);
                        bundle.add_resource(resource).expect("Failed to add base resource.");

                        // Disable isolation markers if desired, or keep default
                        bundles.insert(lang_id, bundle);
                    }
                }
            }
        }

        // 2. Second pass: Load regional overrides (e.g., "en-AU", "fr-CA")
        // and layer them into a dedicated regional bundle that inherits the base file!
        for file in TRANSLATIONS_DIR.files() {
            let path = file.path();
            if let Some(lang_str) = path.file_stem().and_then(|s| s.to_str()) {
                if let Ok(lang_id) = lang_str.parse::<LanguageIdentifier>() {
                    if lang_id.region.is_some() {
                        let ftl_string = file.contents_utf8().unwrap();
                        let regional_resource = FluentResource::try_new(ftl_string.to_string())
                            .expect("Failed to parse regional FTL file.");

                        // Create a specific bundle for the regional variant (e.g., en-AU)
                        let mut regional_bundle = FluentBundle::new_concurrent(vec![lang_id.clone()]);

                        // 🔥 THE MAGIC LAYER: Add the regional override resource FIRST so it takes priority
                        regional_bundle.add_resource(regional_resource).expect("Failed to add regional resource.");

                        // Find its matching base file content (e.g., "en") and append it as the fallback layer!
                        let mut base_id = lang_id.clone();
                        base_id.region = None;

                        if let Some(base_file) = TRANSLATIONS_DIR.get_file(format!("{}.ftl", base_id)) {
                            let base_ftl_string = base_file.contents_utf8().unwrap();
                            let base_resource = FluentResource::try_new(base_ftl_string.to_string())
                                .expect("Failed to parse shared base resource.");

                            // Append the base layer second. Fluent will only use this if the regional resource lacks the key!
                            match regional_bundle.add_resource(base_resource) {
                                Ok(_) => {}
                                Err(_) => {} //Ignore errors
                            }
                        }

                        bundles.insert(lang_id, regional_bundle);
                    }
                }
            }
        }

        I18nManager {
            bundles,
            default_locale,
        }
    }

    /// Fetches a translation bundle with smart language sub-tag fallback tracking
    pub fn get_bundle(&self, locale: &LanguageIdentifier) -> Option<&FluentBundle<FluentResource>> {
        // 1. Try to find an exact match first (e.g., "fr-FR" -> matches "fr-FR.ftl")
        if let Some(bundle) = self.bundles.get(locale) {
            return Some(bundle);
        }

        // 2. Fallback: If the locale has a region tag (like "FR" in "fr-FR"),
        // strip it down to the base language and look for a base dictionary (e.g., "fr.ftl")
        if locale.region.is_some() {
            let mut base_lang = locale.clone();
            base_lang.region = None; // Strips the region portion completely

            if let Some(bundle) = self.bundles.get(&base_lang) {
                return Some(bundle);
            }
        }

        // 3. No match found at all
        None
    }

    pub fn get_default_bundle(&self) -> &FluentBundle<FluentResource> {
        self.bundles.get(&self.default_locale).expect("Default bundle not found!")
    }
}

impl Default for I18nManager {
    fn default() -> Self {
        Self::new()
    }
}

// A smart function to get a message matching a dynamic targeted locale context
pub fn t(key: &str, target_locale: Option<&str>) -> String {
    // 1. Try to parse the target string into a valid LanguageIdentifier
    let lang_id: Option<LanguageIdentifier> = target_locale
        .and_then(|l| l.parse::<LanguageIdentifier>().ok());

    // 2. Fetch the target bundle if it exists; otherwise fall back to default
    let bundle = lang_id
        .and_then(|id| I18N.get_bundle(&id))
        .unwrap_or_else(|| I18N.get_default_bundle());

    let msg = bundle.get_message(key).or_else(|| {
        eprintln!("Message for key \"{}\" not found", key);
        None
    });
    match msg {
        Some(msg) => {
            let pattern = msg.value().expect("Message has no value");
            let mut errors = vec![];
            let value = bundle.format_pattern(pattern, None, &mut errors);
            if !errors.is_empty() {
                eprintln!("Fluent errors: {:?}", errors);
            }
            value.to_string()
        }
        None => {
            format!("<{}>", key)
        }
    }
}

    // A smart function to get a message with arguments matching a dynamic targeted locale context
pub fn t_args<'a>(key: &str, args: &'a FluentArgs, target_locale: Option<&str>) -> String {
    // 1. Try to parse the target string into a valid LanguageIdentifier
    let lang_id: Option<LanguageIdentifier> = target_locale
        .and_then(|l| l.parse::<LanguageIdentifier>().ok());

    // 2. Fetch the target bundle if it exists; otherwise fall back to default
    let bundle = lang_id
        .and_then(|id| I18N.get_bundle(&id))
        .unwrap_or_else(|| I18N.get_default_bundle());

    let msg = bundle.get_message(key).or_else(|| {
        eprintln!("Message for key \"{}\" not found", key);
        None
    });
    match msg {
        Some(msg) => {
            let pattern = msg.value().expect("Message has no value");
            let mut errors = vec![];
            let value = bundle.format_pattern(pattern, Some(args), &mut errors);
            if !errors.is_empty() {
                eprintln!("Fluent errors: {:?}", errors);
            }
            value.to_string()
        }
        None => {
            format!("<{}>", key)
        }
    }
}
        // The trait might be useful later for dependency injection.
pub trait I18n {
    fn text(&self, lang: &LanguageIdentifier, key: &str) -> String;
    fn text_args<'a>(&self, lang: &LanguageIdentifier, key: &str, args: &'a FluentArgs) -> String;
}

impl I18n for I18nManager {
    fn text(&self, lang: &LanguageIdentifier, key: &str) -> String {
        let bundle = self.get_bundle(lang).unwrap_or_else(|| self.get_default_bundle());
        let msg = bundle.get_message(key).expect("Message not found");
        let pattern = msg.value().expect("Message has no value");
        let mut errors = vec![];
        let value = bundle.format_pattern(pattern, None, &mut errors);
        value.to_string()
    }

    fn text_args<'a>(&self, lang: &LanguageIdentifier, key: &str, args: &'a FluentArgs) -> String {
        let bundle = self.get_bundle(lang).unwrap_or_else(|| self.get_default_bundle());
        let msg = bundle.get_message(key).expect("Message not found");
        let pattern = msg.value().expect("Message has no value");
        let mut errors = vec![];
        let value = bundle.format_pattern(pattern, Some(args), &mut errors);
        value.to_string()
    }
}

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
mod tests {
    use super::*;
    use fluent::fluent_args;
    // Define a clear, readable alias for the Narrow No-Break Space character
    const NNBSP: &str = "\u{202f}";

    #[test]
    fn test_t() {
        assert_eq!(t("test-key", Some("en-AU")), "Test Value");
        assert_eq!(t("test-key-override", Some("en")), "Test Value 2");
        assert_eq!(t("test-key-override", Some("en-AU")), "Test Value 2 AU");
        assert_eq!(t("test-key", Some("fr-FR")), "Valeur de test");
        assert_eq!(t("test-key", None), "Test Value");
        assert_eq!(t("non-existent-key", Some("en-AU")), "<non-existent-key>");
    }

    #[test]
    fn test_t_args() {
        let args = fluent_args!["name" => "world"];
        // Explicitly add the isolation tokens to match Fluent's formatting byte-for-byte!
        assert_eq!(
            t_args("test-key-args", &args, Some("en-AU")),
            "Hello, \u{2068}world\u{2069}!"
        );
        assert_eq!(
            t_args("test-key-args", &args, Some("fr-FR")),
            "Bonjour, \u{2068}world\u{2069}!"
        );
        assert_eq!(
            t_args("test-key-args", &args, None),
            "Hello, \u{2068}world\u{2069}!"
        );
    }

    #[test]
    fn test_format_currency_icu() {
        assert_eq!(format_currency_icu(123456, Some("en-AU")), "1,234.56");
        // Define a clear, readable alias for the Narrow No-Break Space character
        let expected = format!("1{}234,56", NNBSP);
        assert_eq!(format_currency_icu(123456, Some("fr-FR")), expected);
        assert_eq!(format_currency_icu(123456, None), "1,234.56");
        assert_eq!(format_currency_icu(-123456, Some("en-AU")), "-1,234.56");
    }

    #[test]
    fn test_format_currency_icu_typ() {
        assert_eq!(format_currency_icu_typ(-123456, Some("en-AU")), "−1,234.56");
    }

    #[test]
    fn test_format_date_icu() {
        assert_eq!(format_date_icu(2026, 5, 25, Some("en-AU")), "25 May 2026");
        assert_eq!(format_date_icu(2026, 5, 25, Some("fr-FR")), "25 mai 2026");
        assert_eq!(format_date_icu(2026, 5, 25, None), "25 May 2026");
    }
}
