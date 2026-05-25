/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use std::sync::OnceLock;
use tracing::level_filters::LevelFilter;
use tracing_appender::rolling;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, Registry};

// Use a global variable to store the guard
static LOGGING_GUARD: OnceLock<tracing_appender::non_blocking::WorkerGuard> = OnceLock::new();

pub(crate) fn setup_logging() {
    let file_appender = rolling::daily("logs", "kelpie_books.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // Store the guard globally to ensure flushing on shutdown
    LOGGING_GUARD
        .set(guard)
        .expect("Failed to set logging guard");

    // Console logging
    let console_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_target(false) // Optional: hide target info
        .with_level(true); // Show log levels

    // File logging
    let file_layer = fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false) // Disable ANSI escape codes for file
        .with_target(true) // Include target info
        .with_level(true);

    // Combine both layers
    let subscriber = Registry::default()
        .with(console_layer)
        .with(file_layer)
        .with(LevelFilter::INFO);

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global subscriber");
}
