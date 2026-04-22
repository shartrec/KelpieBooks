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

use crate::db::security::create_initial_admin;
use crate::routes::{onboarding, security, users};
use crate::util::logging::setup_logging;
use rocket_db_pools::Database;

mod db;
mod util;
mod routes;

#[derive(Database)]
#[database("kelpie_db")]
pub(crate) struct DbKelpie(sqlx::PgPool);

#[rocket::launch]
fn rocket() -> _ {
    setup_logging();
    tracing::info!("Starting server...");

    let rocket = rocket::build()
        .attach(DbKelpie::init())
        .attach(rocket::fairing::AdHoc::on_ignite("Create Initial Admin", |rocket| async {
            let db = DbKelpie::fetch(&rocket).unwrap();
            let mut con = db.acquire().await.unwrap();
            if let Err(e) = create_initial_admin(&mut con).await {
                tracing::error!("Failed to create initial admin user: {:?}", e);
            }
            rocket
        }))
        .mount("/", security::routes())
        .mount("/", onboarding::routes())
        .mount("/", users::routes());

    rocket
}
