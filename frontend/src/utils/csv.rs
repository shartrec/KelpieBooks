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

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::js_sys;
use web_sys::{Blob, BlobPropertyBag, Url};

pub fn download_csv(filename: &str, content: &str) -> Result<(), JsValue> {
    let blob = Blob::new_with_str_sequence_and_options(
        &js_sys::Array::of1(&content.into()),
        BlobPropertyBag::new().type_("text/csv;charset=utf-8;"),
    )?;

    let link = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("a")?
        .dyn_into::<web_sys::HtmlAnchorElement>()?;

    let url = Url::create_object_url_with_blob(&blob)?;
    link.set_href(&url);
    link.set_download(filename);
    link.style().set_property("display", "none")?;

    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .append_child(&link)?;

    link.click();

    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .remove_child(&link)?;

    Url::revoke_object_url(&url)?;

    Ok(())
}
