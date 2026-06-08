/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

pub fn detect_browser_locale() -> String {
    web_sys::window()
        .and_then(|win| win.navigator().language())
        .unwrap_or_else(|| "en-GB".to_string()) // Safe fallback
}
