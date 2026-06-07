/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
#![forbid(unsafe_code)]

use crate::routes::{
    configurations, dashboard, onboarding, organization, period_end, privileges, roles, security as security_routes, users,
};
use crate::util::logging::setup_logging;
use rocket::fairing::AdHoc;
use rocket::fs::{relative, FileServer, NamedFile};
use rocket::{get, routes};
use rocket_db_pools::Database;

#[cfg(feature = "ledger")]
pub(crate) mod ledger;
#[cfg(feature = "partners")]
pub(crate) mod partners;
#[cfg(feature = "payables")]
pub(crate) mod payables;

mod db;
mod routes;
mod services;
mod util;
pub(crate) mod security;

#[derive(Database)]
#[database("kelpie_db")]
pub(crate) struct DbKelpie(sqlx::PgPool);

#[get("/<_..>", rank = 20)]
async fn spa_index() -> Option<NamedFile> {
    // This tells Rocket: "If nothing else matched, just send them the index.html"
    NamedFile::open(relative!("./static/index.html")).await.ok()
}

fn run_migrations() -> AdHoc {
    AdHoc::try_on_ignite("SQLx Migrations", |rocket| async {
        let db = DbKelpie::fetch(&rocket)
            .expect("Database pool not initialized");

        sqlx::migrate!("./migrations")
            .run(&**db)
            .await
            .expect("Failed to run migrations");

        Ok(rocket)
    })
}

#[rocket::launch]
fn rocket() -> _ {
    setup_logging();
    tracing::info!("Starting server...");

    let rocket = rocket::build()
        .attach(DbKelpie::init())
        .attach(run_migrations())
        .mount("/", security_routes::routes())
        .mount("/", onboarding::routes())
        .mount("/", users::routes())
        .mount("/", configurations::routes())
        .mount("/", organization::routes())
        .mount("/", privileges::routes())
        .mount("/", roles::routes())
        .mount("/api/dashboard", dashboard::routes());
    #[cfg(feature = "ledger")]
    let rocket = rocket
        .mount("/", ledger::routes::accounts::routes())
        .mount("/", ledger::routes::reports::routes())
        .mount("/", ledger::routes::transactions::routes())
        .mount("/", period_end::routes());
    #[cfg(feature = "partners")]
    let rocket = rocket
        .mount("/", partners::routes::partners::routes());
    #[cfg(feature = "payables")]
    let rocket = rocket
        .mount("/", payables::routes::reports::routes())
        .mount("/", payables::routes::vendor_invoices::routes())
        .mount("/", payables::routes::vendor_payments::routes());
    let rocket = rocket
        .mount("/", FileServer::from(relative!("./static")))
        // 3. Mount the fallback route with a lower priority (rank 2)
        .mount("/", routes![spa_index]);

    rocket
}
