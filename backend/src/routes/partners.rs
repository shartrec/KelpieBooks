/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use crate::security::{ManagePartners, RequirePrivilege, UsePartners};
use crate::services::partner_service;
use crate::util::types::PathUuid;
use crate::util::ApiError;
use crate::DbKelpie;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, routes, Route};
use rocket_db_pools::Connection;
use shared_core::dtos::partner_list_item::PartnerListItem;
use shared_core::models::partner::Partner;
use shared_core::models::partner_address::PartnerAddress;
use shared_core::models::partner_contact::PartnerContact;
use shared_core::requests::partner::{CreatePartnerRequest, UpdatePartnerRequest};

pub(crate) fn routes() -> Vec<Route> {
    routes![
        get_partners,
        get_partner,
        get_partner_addresses,
        get_partner_contacts,
        create_partner,
        update_partner,
        delete_partner,
        create_address,
        update_address,
        delete_address,
        create_contact,
        update_contact,
        delete_contact,
    ]
}

#[get("/api/partners")]
async fn get_partners(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<UsePartners>,
) -> Result<Json<Vec<PartnerListItem>>, ApiError> {
    let user = guard.0;
    let partners = partner_service::get_partners(&mut pool, user.organization_id).await?;
    Ok(Json(partners))
}

#[get("/api/partners/<id>")]
async fn get_partner(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
    _guard: RequirePrivilege<UsePartners>,
) -> Result<Json<Partner>, ApiError> {
    let partner = partner_service::get_partner(&mut pool, *id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Partner not found".to_string()))?;
    Ok(Json(partner))
}

#[get("/api/partners/<id>/addresses")]
async fn get_partner_addresses(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
    _guard: RequirePrivilege<UsePartners>,
) -> Result<Json<Vec<PartnerAddress>>, ApiError> {
    let addresses = partner_service::get_partner_addresses(&mut pool, *id).await?;
    Ok(Json(addresses))
}

#[get("/api/partners/<id>/contacts")]
async fn get_partner_contacts(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
    _guard: RequirePrivilege<UsePartners>,
) -> Result<Json<Vec<PartnerContact>>, ApiError> {
    let contacts = partner_service::get_partner_contacts(&mut pool, *id).await?;
    Ok(Json(contacts))
}

#[post("/api/partners", data = "<req>")]
async fn create_partner(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<ManagePartners>,
    req: Json<CreatePartnerRequest>,
) -> Result<Json<Partner>, ApiError> {
    let user = guard.0;
    let new_partner =
        partner_service::create_partner(&mut pool, user.organization_id, &req).await?;
    Ok(Json(new_partner))
}

#[put("/api/partners/<id>", data = "<req>")]
async fn update_partner(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<ManagePartners>,
    id: PathUuid,
    req: Json<UpdatePartnerRequest>,
) -> Result<Json<Partner>, ApiError> {
    let user = guard.0;
    let updated_partner =
        partner_service::update_partner(&mut pool, user.organization_id, *id, &req).await?;
    Ok(Json(updated_partner))
}

#[delete("/api/partners/<id>")]
async fn delete_partner(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
    guard: RequirePrivilege<ManagePartners>,
) -> Result<&'static str, ApiError> {

    let user = guard.0;

    let rows_affected = partner_service::delete_partner(&mut pool, user.organization_id, *id).await?;
    if rows_affected == 0 {
        return Err(ApiError::NotFound("Partner not found.".to_string()));
    }

    Ok("Partner deleted successfully.")
}

#[post("/api/partners/<partner_id>/addresses", data = "<address>")]
async fn create_address(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<ManagePartners>,
    partner_id: PathUuid,
    address: Json<PartnerAddress>,
) -> Result<Json<PartnerAddress>, ApiError> {
    let user = guard.0;
    let new_address =
        partner_service::create_address(&mut pool, user.organization_id, *partner_id, &address)
            .await?;
    Ok(Json(new_address))
}

#[put(
    "/api/partners/<_partner_id>/addresses/<address_id>",
    data = "<address>"
)]
async fn update_address(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<ManagePartners>,
    _partner_id: PathUuid,
    address_id: PathUuid,
    address: Json<PartnerAddress>,
) -> Result<Json<PartnerAddress>, ApiError> {
    let user = guard.0;
    let updated_address =
        partner_service::update_address(&mut pool, user.organization_id, *address_id, &address)
            .await?;
    Ok(Json(updated_address))
}

#[delete("/api/partners/<_partner_id>/addresses/<address_id>")]
async fn delete_address(
    mut pool: Connection<DbKelpie>,
    _partner_id: PathUuid,
    address_id: PathUuid,
    guard: RequirePrivilege<ManagePartners>,
) -> Result<&'static str, ApiError> {

    let user = guard.0;
    let rows_affected = partner_service::delete_address(&mut pool, user.organization_id, *address_id).await?;
    if rows_affected == 0 {
        return Err(ApiError::NotFound("Address not found.".to_string()));
    }
    Ok("Address deleted successfully.")
}

#[post("/api/partners/<partner_id>/contacts", data = "<contact>")]
async fn create_contact(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<ManagePartners>,
    partner_id: PathUuid,
    contact: Json<PartnerContact>,
) -> Result<Json<PartnerContact>, ApiError> {
    let user = guard.0;
    let new_contact =
        partner_service::create_contact(&mut pool, user.organization_id, *partner_id, &contact)
            .await?;
    Ok(Json(new_contact))
}

#[put(
    "/api/partners/<_partner_id>/contacts/<contact_id>",
    data = "<contact>"
)]
async fn update_contact(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<ManagePartners>,
    _partner_id: PathUuid,
    contact_id: PathUuid,
    contact: Json<PartnerContact>,
) -> Result<Json<PartnerContact>, ApiError> {
    let user = guard.0;
    let updated_contact =
        partner_service::update_contact(&mut pool, user.organization_id, *contact_id, &contact)
            .await?;
    Ok(Json(updated_contact))
}

#[delete("/api/partners/<_partner_id>/contacts/<contact_id>")]
async fn delete_contact(
    mut pool: Connection<DbKelpie>,
    _partner_id: PathUuid,
    contact_id: PathUuid,
    guard: RequirePrivilege<ManagePartners>,
) -> Result<&'static str, ApiError> {

    let user = guard.0;

    let rows_affected = partner_service::delete_contact(&mut pool, user.organization_id, *contact_id).await?;
    if rows_affected == 0 {
        return Err(ApiError::NotFound("Contact not found.".to_string()));
    }
    Ok("Contact deleted successfully.")
}
