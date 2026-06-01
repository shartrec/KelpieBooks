/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
#![forbid(unsafe_code)]

use crate::routes::{
    accounts, configurations, dashboard, onboarding, organization, partners, period_end, privileges, reports,
    security, transactions, users, vendor_invoices, vendor_payments,
};
use crate::util::logging::setup_logging;
use rocket::fs::{relative, FileServer, NamedFile};
use rocket::{get, routes};
use rocket_db_pools::Database;

mod db;
mod export;
mod routes;
mod services;
mod util;

#[derive(Database)]
#[database("kelpie_db")]
pub(crate) struct DbKelpie(sqlx::PgPool);

#[get("/<_..>", rank = 20)]
async fn spa_index() -> Option<NamedFile> {
    // This tells Rocket: "If nothing else matched, just send them the index.html"
    NamedFile::open(relative!("./static/index.html")).await.ok()
}

#[rocket::launch]
fn rocket() -> _ {
    setup_logging();
    tracing::info!("Starting server...");

    let rocket = rocket::build()
        .attach(DbKelpie::init())
        .mount("/", security::routes())
        .mount("/", onboarding::routes())
        .mount("/", users::routes())
        .mount("/", accounts::routes())
        .mount("/", partners::routes())
        .mount("/", reports::routes())
        .mount("/", transactions::routes())
        .mount("/", period_end::routes())
        .mount("/", configurations::routes())
        .mount("/", organization::routes())
        .mount("/", vendor_invoices::routes())
        .mount("/", vendor_payments::routes())
        .mount("/", privileges::routes())
        .mount("/api/dashboard", dashboard::routes())
        .mount("/", FileServer::from(relative!("./static")))
        // 3. Mount the fallback route with a lower priority (rank 2)
        .mount("/", routes![spa_index]);

    rocket
}
