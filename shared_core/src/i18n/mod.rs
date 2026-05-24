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

//! Internationalisation module
use fluent::{FluentArgs, FluentResource};
use fluent::concurrent::FluentBundle;
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

impl I18nManager {
    pub fn new() -> Self {
        let default_locale = langid!("en-AU");
        let mut bundles = HashMap::new();

        for file in TRANSLATIONS_DIR.files() {
            let path = file.path();
            if let Some(stem) = path.file_stem() {
                if let Some(lang_str) = stem.to_str() {
                    if let Ok(lang_id) = lang_str.parse::<LanguageIdentifier>() {
                        let ftl_string = file.contents_utf8().unwrap();
                        let resource = FluentResource::try_new(ftl_string.to_string())
                            .expect("Failed to parse an FTL file.");

                        let mut bundle = FluentBundle::new_concurrent(vec![lang_id.clone()]);
                        bundle.add_resource(resource).expect("Failed to add FTL resource to bundle.");
                        bundles.insert(lang_id, bundle);
                    }
                }
            }
        }

        I18nManager {
            bundles,
            default_locale,
        }
    }

    pub fn get_bundle(&self, locale: &LanguageIdentifier) -> Option<&FluentBundle<FluentResource>> {
        self.bundles.get(locale)
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

// A simple function to get a message.
pub fn t(key: &str) -> String {
    let bundle = I18N.get_default_bundle();
    let msg = bundle.get_message(key).expect("Message not found");
    let pattern = msg.value().expect("Message has no value");
    let mut errors = vec![];
    let value = bundle.format_pattern(pattern, None, &mut errors);
    if !errors.is_empty() {
        eprintln!("Fluent errors: {:?}", errors);
    }
    value.to_string()
}

// A function to get a message with arguments.
pub fn t_args<'a>(key: &str, args: &'a FluentArgs) -> String {
    let bundle = I18N.get_default_bundle();
    let msg = bundle.get_message(key).expect("Message not found");
    let pattern = msg.value().expect("Message has no value");
    let mut errors = vec![];
    let value = bundle.format_pattern(pattern, Some(args), &mut errors);
    if !errors.is_empty() {
        eprintln!("Fluent errors: {:?}", errors);
    }
    value.to_string()
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
