/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
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
