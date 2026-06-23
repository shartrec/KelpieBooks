/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

//! Internationalisation module

use std::{
    cell::RefCell,
    collections::HashMap,
    sync::LazyLock,
};

use fluent::{
    concurrent::FluentBundle,
    FluentArgs,
    FluentResource,
};
use icu_calendar::Date;
use icu_datetime::{
    fieldsets::YMD,
    DateTimeFormatter,
};
use icu_decimal::DecimalFormatter;
use icu_provider::prelude::icu_locale_core::{
    locale,
    Locale,
};
use include_dir::{
    include_dir,
    Dir,
};
use rust_decimal::Decimal;
use unic_langid::{
    langid,
    LanguageIdentifier,
};

// Define the directory where translation files are.
static TRANSLATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/translations");

thread_local! {
    static DECIMAL_FORMATTER_CACHE: RefCell<HashMap<Locale, DecimalFormatter>> =
        RefCell::new(HashMap::new());
    static PERCENT_FORMATTER_CACHE: RefCell<HashMap<Locale, DecimalFormatter>> =
        RefCell::new(HashMap::new());
    static DATE_FORMATTER_CACHE: RefCell<HashMap<Locale, DateTimeFormatter<YMD>>> =
        RefCell::new(HashMap::new());
}

// Create a lazy static instance of the I18nManager.
pub static I18N: LazyLock<I18nManager> = LazyLock::new(I18nManager::new);

// The main struct to manage all translations.
pub struct I18nManager {
    bundles: HashMap<LanguageIdentifier, FluentBundle<FluentResource>>,
    default_locale: Locale,
    default_lang_id: LanguageIdentifier,
}

// shared_core/src/i18n/mod.rs

impl I18nManager {
    pub fn new() -> Self {
        let default_langid = langid!("en-AU");
        let mut bundles = HashMap::new();

        // 1. First pass: Load all base language files (e.g., "en", "fr", "es")
        // and initialize their concurrent bundles
        for file in TRANSLATIONS_DIR.files() {
            let path = file.path();
            let err_msg = format!("Translation dir {:?}",path);
            if let Some(lang_str) = path.file_stem().and_then(|s| s.to_str()) {
                if let Ok(lang_id) = lang_str.parse::<LanguageIdentifier>() {
                    // Only process base languages without regions in the first pass (e.g., length == 1 or region is None)
                    if lang_id.region.is_none() {
                        let ftl_string = file.contents_utf8().unwrap();
                        let resource = FluentResource::try_new(ftl_string.to_string())
                            .expect("Failed to parse base FTL file.");

                        let mut bundle = FluentBundle::new_concurrent(vec![lang_id.clone()]);
                        bundle
                            .add_resource(resource)
                            .expect(&err_msg);

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
                        let mut regional_bundle =
                            FluentBundle::new_concurrent(vec![lang_id.clone()]);

                        // 🔥 THE MAGIC LAYER: Add the regional override resource FIRST so it takes priority
                        regional_bundle
                            .add_resource(regional_resource)
                            .expect("Failed to add regional resource.");

                        // Find its matching base file content (e.g., "en") and append it as the fallback layer!
                        let mut base_id = lang_id.clone();
                        base_id.region = None;

                        if let Some(base_file) =
                            TRANSLATIONS_DIR.get_file(format!("{}.ftl", base_id))
                        {
                            let base_ftl_string = base_file.contents_utf8().unwrap();
                            let base_resource =
                                FluentResource::try_new(base_ftl_string.to_string())
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
            default_locale: locale!("en-AU"),
            default_lang_id: default_langid,
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
        self.bundles
            .get(&self.default_lang_id)
            .expect("Default bundle not found!")
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
    let lang_id: Option<LanguageIdentifier> =
        target_locale.and_then(|l| l.parse::<LanguageIdentifier>().ok());

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
    let lang_id: Option<LanguageIdentifier> =
        target_locale.and_then(|l| l.parse::<LanguageIdentifier>().ok());

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
        let bundle = self
            .get_bundle(lang)
            .unwrap_or_else(|| self.get_default_bundle());
        let msg = bundle.get_message(key).expect("Message not found");
        let pattern = msg.value().expect("Message has no value");
        let mut errors = vec![];
        let value = bundle.format_pattern(pattern, None, &mut errors);
        value.to_string()
    }

    fn text_args<'a>(&self, lang: &LanguageIdentifier, key: &str, args: &'a FluentArgs) -> String {
        let bundle = self
            .get_bundle(lang)
            .unwrap_or_else(|| self.get_default_bundle());
        let msg = bundle.get_message(key).expect("Message not found");
        let pattern = msg.value().expect("Message has no value");
        let mut errors = vec![];
        let value = bundle.format_pattern(pattern, Some(args), &mut errors);
        value.to_string()
    }
}

/// Formats standard decimal into localized decimal strings.
/// e.g., 123456 -> "1,234.56" (en-AU) or "1 234,56" (fr-FR)
pub fn format_currency_icu(amount: Decimal, target_locale: Option<&str>) -> String {
    format_decimal_icu(amount, target_locale)
}

/// Formats standard decimal into localized decimal strings.
/// e.g., 123456 -> "1,234.56" (en-AU) or "1 234,56" (fr-FR)
pub fn format_decimal_icu(amount: Decimal, target_locale: Option<&str>) -> String {
    let locale: Locale = target_locale
        .and_then(|l| l.parse().ok())
        .unwrap_or_else(|| I18N.default_locale.clone());

    DECIMAL_FORMATTER_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();

        // Evict if an attacker tries to flood the map with infinite dynamic variations
        if cache.len() > 50 {
            cache.clear();
        }

        let formatter = cache.entry(locale.clone()).or_insert_with(|| {
            DecimalFormatter::try_new(locale.into(), Default::default())
                .expect("Failed to initialize ICU4X Decimal Formatter")
        });

        let mut icu_decimal = icu_decimal::input::Decimal::from(amount.mantissa() as i64);
        icu_decimal.multiply_pow10(-(amount.scale() as i16));
        icu_decimal.pad_end(-2);

        formatter.format_to_string(&icu_decimal)
    })
}

pub fn format_percentage_icu(amount: Decimal, target_locale: Option<&str>) -> String {

    // Todo Use ICU formatting when Notation formatting is no longer experimental
    let s = format_decimal_icu(amount, target_locale);
    format!("{}%", s)

}

/// Specialized wrapper for Typst reporting layouts
pub fn format_currency_icu_typ(amount: Decimal, target_locale: Option<&str>) -> String {
    let formatted = format_currency_icu(amount, target_locale);

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
    let locale: Locale = target_locale
        .and_then(|l| l.parse().ok())
        .unwrap_or_else(|| I18N.default_locale.clone());

    DATE_FORMATTER_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();

        // Evict if an attacker tries to flood the map with infinite dynamic variations
        if cache.len() > 50 {
            cache.clear();
        }

        let formatter = cache.entry(locale.clone()).or_insert_with(|| {
            DateTimeFormatter::try_new(locale.into(), YMD::medium())
                .expect("Failed to construct ICU4X DateTime engine configuration context")
        });

        let date_object = Date::try_new_iso(year, month as u8, day as u8)
            .expect("Invalid Date integer bounds provided to ledger component system");

        formatter.format(&date_object).to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use fluent::fluent_args;
    use rust_decimal::dec;
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

        assert_eq!(format_currency_icu(dec!(-1234), Some("en-AU")), "-1,234.00");
        assert_eq!(format_currency_icu(dec!(1234), Some("en-AU")), "1,234.00");
        assert_eq!(format_currency_icu(dec!(1234.56), Some("en-AU")), "1,234.56");
        // Define a clear, readable alias for the Narrow No-Break Space character
        let expected = format!("1{}234,56", NNBSP);
        assert_eq!(format_currency_icu(dec!(1234.56), Some("fr-FR")), expected);
        assert_eq!(format_currency_icu(dec!(1234.56), None), "1,234.56");
        assert_eq!(format_currency_icu(dec!(-1234.56), Some("en-AU")), "-1,234.56");
    }

    #[test]
    fn test_format_currency_icu_typ() {
        assert_eq!(format_currency_icu_typ(dec!(-1234.56), Some("en-AU")), "−1,234.56");
    }

    #[test]
    fn test_format_date_icu() {
        assert_eq!(format_date_icu(2026, 5, 25, Some("en-AU")), "25 May 2026");
        assert_eq!(format_date_icu(2026, 5, 25, Some("fr-FR")), "25 mai 2026");
        assert_eq!(format_date_icu(2026, 5, 25, None), "25 May 2026");
    }

    #[test]
    fn audit_missing_translations() {
        use std::{
            collections::HashSet,
            fs,
        };

        let base_content =
            fs::read_to_string("translations/en.ftl").expect("Failed to read base en.ftl");

        // Extract keys from base file
        let base_keys: HashSet<&str> = base_content
            .lines()
            .filter(|line| line.contains('=') && !line.starts_with('#'))
            .map(|line| line.split('=').next().unwrap().trim())
            .collect();

        // Iterate through other target files
        let target_locales = vec!["translations/fr.ftl"];
        for locale_path in target_locales {
            let content = fs::read_to_string(locale_path).unwrap();
            let current_keys: HashSet<&str> = content
                .lines()
                .filter(|line| line.contains('=') && !line.starts_with('#'))
                .map(|line| line.split('=').next().unwrap().trim())
                .collect();

            let missing_keys: Vec<&&str> = base_keys.difference(&current_keys).collect();

            if !missing_keys.is_empty() {
                let mut error_message = format!(
                    "🚨 Localization Leak: The following keys are missing from target file '{}':\n",
                    locale_path
                );
                for key in missing_keys {
                    error_message.push_str(&format!("- {}\n", key));
                }
                panic!("{}", error_message);
            }
        }
    }
}
