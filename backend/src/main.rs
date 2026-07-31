/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
#![forbid(unsafe_code)]

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
use std::path::PathBuf;

use log::error;
use rocket::{
    fairing::AdHoc,
    fs::{
        FileServer,
        NamedFile,
    },
    get,
    http::ContentType,
    routes,
    State,
};
use rocket_db_pools::Database;

use crate::util::logging::setup_logging;

#[cfg(feature = "inventory")]
pub mod inventory;
#[cfg(feature = "ledger")]
pub(crate) mod ledger;
#[cfg(feature = "partners")]
pub(crate) mod partners;
#[cfg(feature = "payables")]
pub(crate) mod payables;
#[cfg(feature = "sales")]
pub mod sales;

pub mod config;
pub mod core;
pub(crate) mod security;
mod util;

#[derive(Database)]
#[database("kelpie_db")]
pub(crate) struct DbKelpie(sqlx::PgPool);

// In your main setup or utility module
pub struct AssetsDir {
    pub path: std::path::PathBuf,
}

#[get("/<_..>", rank = 20)]
async fn spa_index(assets_state: &State<AssetsDir>) -> Option<NamedFile> {
    // This tells Rocket: "If nothing else matched, just send them the index.html"
    NamedFile::open(assets_state.path.join("index.html"))
        .await
        .ok()
}

#[get("/fonts/<file..>", rank = 10)]
pub async fn serve_fonts(
    file: PathBuf,
    assets_state: &State<AssetsDir>,
) -> Option<(ContentType, NamedFile)> {
    let path = assets_state.path.join("fonts").join(file);
    let ext = path.extension()?.to_str()?;

    let content_type = match ext {
        "woff2" => ContentType::WOFF2,
        "woff" => ContentType::WOFF,
        _ => ContentType::Binary,
    };

    let named_file = NamedFile::open(path).await.ok()?;
    Some((content_type, named_file))
}

pub struct TemplateConfig {
    pub root_directory: PathBuf,
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
        .mount("/", sales::routes::tax_categories::routes())
        .mount("/", sales::routes::sales_invoices::routes())
        .mount("/", sales::routes::customer_payments::routes())
        .mount("/", sales::routes::reports::routes());

    #[cfg(any(feature = "sales", feature = "inventory"))]
    let rocket = rocket
        .mount("/", sales::routes::items::routes())
        .mount("/", sales::routes::uoms::routes());

    #[cfg(feature = "inventory")]
    let rocket = rocket
        .mount("/", inventory::routes::locations::routes())
        .mount("/", inventory::routes::balances::routes())
        .mount("/", inventory::routes::warehouse::routes());

    // Determine the environment directory pathway
    let assets_dir = util::get_static_dir(&rocket);
    let rocket = rocket
        // 1. Store the asset directory pathway in managed state
        .manage(AssetsDir {
            path: assets_dir.clone(),
        })
        .mount("/", routes![serve_fonts])
        .mount("/", FileServer::from(assets_dir).rank(15))
        .mount("/", routes![spa_index]);

    let template_dir = util::get_template_dir(&rocket);
    let rocket = rocket.manage(TemplateConfig {
        root_directory: template_dir,
    });

    rocket
}
