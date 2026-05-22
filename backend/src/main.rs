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
#![forbid(unsafe_code)]

use crate::routes::{
    accounts, configurations, dashboard, onboarding, organization, partners, period_end, reports,
    security, transactions, users, vendor_invoices, vendor_invoice_payments,
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
        .mount("/", vendor_invoice_payments::routes())
        .mount("/api/dashboard", dashboard::routes())
        .mount("/", FileServer::from(relative!("./static")))
        // 3. Mount the fallback route with a lower priority (rank 2)
        .mount("/", routes![spa_index]);

    rocket
}
