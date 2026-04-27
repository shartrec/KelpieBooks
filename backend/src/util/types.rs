/*
 * Copyright (c) 2026-2026. Trevor Campbell and others.
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

use rocket::request::FromParam;
use std::ops::Deref;
use uuid::Uuid;

/// A newtype wrapper for `Uuid` to implement `FromParam` and satisfy the orphan rule.
/// This allows Rocket to parse `Uuid` values from URL path segments.
#[derive(Clone, Copy)]
pub struct PathUuid(pub Uuid);

/// Allows `PathUuid` to be used as a `Uuid` via dereferencing (e.g., `*id`).
impl Deref for PathUuid {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Synchronous implementation of `FromParam` for our `PathUuid` newtype.
impl<'r> FromParam<'r> for PathUuid {
    type Error = uuid::Error;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        Uuid::parse_str(param).map(PathUuid)
    }
}
