use crate::routes::security::AuthenticatedUser;
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
use crate::services::partner_service;

pub(crate) fn routes() -> Vec<Route> {
    routes![
        get_partners,
        get_partner,
        get_partner_addresses,
        get_partner_contacts,
        create_partner,
        update_partner,
        delete_partner,
    ]
}

#[get("/api/partners")]
async fn get_partners(
    mut pool: Connection<DbKelpie>,
    user: AuthenticatedUser,
) -> Result<Json<Vec<PartnerListItem>>, ApiError> {
    let partners =
        partner_service::get_partners(&mut pool, user.organization_id).await?;
    Ok(Json(partners))
}

#[get("/api/partners/<id>")]
async fn get_partner(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
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
) -> Result<Json<Vec<PartnerAddress>>, ApiError> {
    let addresses = partner_service::get_partner_addresses(&mut pool, *id).await?;
    Ok(Json(addresses))
}

#[get("/api/partners/<id>/contacts")]
async fn get_partner_contacts(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
) -> Result<Json<Vec<PartnerContact>>, ApiError> {
    let contacts = partner_service::get_partner_contacts(&mut pool, *id).await?;
    Ok(Json(contacts))
}

#[post("/api/partners", data = "<req>")]
async fn create_partner(
    mut pool: Connection<DbKelpie>,
    user: AuthenticatedUser,
    req: Json<CreatePartnerRequest>,
) -> Result<Json<Partner>, ApiError> {
    let new_partner = partner_service::create_partner(&mut pool, user.organization_id, &req).await?;
    Ok(Json(new_partner))
}

#[put("/api/partners/<id>", data = "<req>")]
async fn update_partner(
    mut pool: Connection<DbKelpie>,
    user: AuthenticatedUser,
    id: PathUuid,
    req: Json<UpdatePartnerRequest>,
) -> Result<Json<Partner>, ApiError> {
    let updated_partner = partner_service::update_partner(&mut pool, user.organization_id, *id, &req).await?;
    Ok(Json(updated_partner))
}

#[delete("/api/partners/<id>")]
async fn delete_partner(
    mut pool: Connection<DbKelpie>,
    id: PathUuid,
) -> Result<&'static str, ApiError> {
    let rows_affected = partner_service::delete_partner(&mut pool, *id).await?;
    if rows_affected == 0 {
        return Err(ApiError::NotFound("Partner not found.".to_string()));
    }

    Ok("Partner deleted successfully.")
}
