/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
#![forbid(unsafe_code)]

use std::env;
use std::path::PathBuf;
use log::error;
use rocket::{fairing::AdHoc, fs::{
    relative,
    FileServer,
    NamedFile,
}, get, routes, Build, Rocket};
use rocket_db_pools::Database;
use core::routes::{
    configurations,
    dashboard,
    onboarding,
    organization,
    period_end,
    privileges,
    roles,
    security as security_routes,
    users,
};
use crate::util::logging::setup_logging;

#[cfg(feature = "ledger")]
pub(crate) mod ledger;
#[cfg(feature = "partners")]
pub(crate) mod partners;
#[cfg(feature = "payables")]
pub(crate) mod payables;
#[cfg(feature = "sales")]
pub mod sales;

pub(crate) mod security;
mod util;
pub mod core;
pub mod config;

#[derive(Database)]
#[database("kelpie_db")]
pub(crate) struct DbKelpie(sqlx::PgPool);

#[get("/<_..>", rank = 20)]
async fn spa_index() -> Option<NamedFile> {
    // This tells Rocket: "If nothing else matched, just send them the index.html"
    NamedFile::open(relative!("./static/index.html")).await.ok()
}

fn get_static_assets_dir(rocket: &Rocket<Build>) -> PathBuf {
    let profile = rocket.figment().profile().as_ref();

    match profile {
        "debug" => {
            // Development Mode: Point directly to the source tree folder
            // This allows local asset changes to show up immediately
            println!("🚀 Running in DEVELOPMENT mode. Using source assets tree.");
            rocket::fs::relative!("static").into()
        }
        _ => {
            // Production / Release Mode: Look next to the running executable binary
            println!("⚙️ Running in PRODUCTION mode. Using neighboring standalone assets folder.");
            let current_dir = env::current_dir().expect("Failed to read runtime directory");
            current_dir.join("static")
        }
    }
}

fn run_migrations() -> AdHoc {
    AdHoc::try_on_ignite("SQLx Migrations", |rocket| async {
        let db = DbKelpie::fetch(&rocket).expect("Database pool not initialized");

        sqlx::migrate!("./migrations")
            .run(&**db)
            .await
            .unwrap_or_else(|e| {
                error!("Failed to run migrations. err: {}", e);
            });

        Ok(rocket)
    })
}

#[rocket::launch]
fn rocket() -> _ {
    setup_logging();
    config::load_config();
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
    let rocket = rocket.mount("/", partners::routes::partners::routes());
    #[cfg(feature = "payables")]
    let rocket = rocket
        .mount("/", payables::routes::reports::routes())
        .mount("/", payables::routes::vendor_invoices::routes())
        .mount("/", payables::routes::vendor_payments::routes());
    #[cfg(feature = "sales")]
    let rocket = rocket
        .mount("/", sales::routes::items::routes())
        .mount("/", sales::routes::uoms::routes())
        .mount("/", sales::routes::tax_categories::routes());

    // Determine the environment directory pathway
    let assets_dir = get_static_assets_dir(&rocket);
    let rocket = rocket
        .mount("/", FileServer::from(assets_dir))
        // 3. Mount the fallback route with a lower priority (rank 2)
        .mount("/", routes![spa_index]);

    rocket
}