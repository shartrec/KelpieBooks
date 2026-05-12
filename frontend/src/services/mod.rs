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

pub mod auth;
pub mod accounts;
pub mod transactions;
pub mod reports;
pub mod organization;
pub mod dashboard;

use gloo_net::Error;
use gloo_net::http::Response;
use serde::de::DeserializeOwned;

pub async fn handle_response<T: DeserializeOwned>(response: Result<Response, Error>) -> Result<T, String> {
    match response {
        Ok(response) if response.ok() => {
            response.json::<T>().await.map_err(|e| e.to_string())
        }
        Ok(response) => {
            let status = response.status();
            let err_body = response.text().await.unwrap_or_default();
            Err(format!("HTTP error {}: {}", status, err_body))
        }
        Err(e) => Err(e.to_string()),
    }
}
