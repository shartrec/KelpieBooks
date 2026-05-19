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

use crate::contexts::auth_context::UserContextHandle;
use crate::router::Route;
use gloo_net::http::{Request, Response};
use gloo_net::Error;
use log::info;
use serde::Serialize;
use yew_router::prelude::Navigator;

pub struct Api;

impl Api {
    fn check_auth(
        response: &Result<Response, Error>,
        user_ctx: &UserContextHandle,
        navigator: &Navigator,
    ) {
        info!("Response: {:?}", response);
        if let Ok(resp) = response {
            if resp.status() == 401 {
                user_ctx.dispatch(None);
                navigator.push(&Route::Login);
            }
        }
    }

    pub async fn get(
        url: &str,
        user_ctx: UserContextHandle,
        navigator: Navigator,
    ) -> Result<Response, Error> {
        let response = Request::get(url).send().await;
        Self::check_auth(&response, &user_ctx, &navigator);
        response
    }

    pub async fn post<T: Serialize>(
        url: &str,
        body: &T,
        user_ctx: UserContextHandle,
        navigator: Navigator,
    ) -> Result<Response, Error> {
        let response = Request::post(url).json(body).unwrap().send().await;
        Self::check_auth(&response, &user_ctx, &navigator);
        response
    }

    pub async fn put<T: Serialize>(
        url: &str,
        body: &T,
        user_ctx: UserContextHandle,
        navigator: Navigator,
    ) -> Result<Response, Error> {
        let response = Request::put(url).json(body).unwrap().send().await;
        Self::check_auth(&response, &user_ctx, &navigator);
        response
    }

    pub async fn delete(
        url: &str,
        user_ctx: UserContextHandle,
        navigator: Navigator,
    ) -> Result<Response, Error> {
        let response = Request::delete(url).send().await;
        Self::check_auth(&response, &user_ctx, &navigator);
        response
    }
}
