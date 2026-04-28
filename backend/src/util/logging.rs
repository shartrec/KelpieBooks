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
