use crate::db::partner as partner_db;
use crate::util::ApiError;
use rocket_db_pools::sqlx::{self, PgConnection};
use shared_core::dtos::partner_list_item::PartnerListItem;
use shared_core::models::partner::Partner;
use shared_core::models::partner_address::PartnerAddress;
use shared_core::models::partner_contact::PartnerContact;
use shared_core::requests::partner::{CreatePartnerRequest, UpdatePartnerRequest};
use sqlx::Acquire;
use uuid::Uuid;

pub async fn get_partners(
    pool: &mut PgConnection,
    organization_id: Uuid,
) -> Result<Vec<PartnerListItem>, ApiError> {
    let partners = partner_db::get_all_by_org(pool, organization_id).await?;
    Ok(partners)
}

pub async fn get_partner(
    pool: &mut PgConnection,
    partner_id: Uuid,
) -> Result<Option<Partner>, ApiError> {
    let partner = partner_db::get(pool, partner_id).await?;
    Ok(partner)
}

pub async fn get_partner_addresses(
    pool: &mut PgConnection,
    partner_id: Uuid,
) -> Result<Vec<PartnerAddress>, ApiError> {
    let addresses = partner_db::get_addresses(pool, partner_id).await?;
    Ok(addresses)
}

pub async fn get_partner_contacts(
    pool: &mut PgConnection,
    partner_id: Uuid,
) -> Result<Vec<PartnerContact>, ApiError> {
    let contacts = partner_db::get_contacts(pool, partner_id).await?;
    Ok(contacts)
}

pub async fn create_partner(
    pool: &mut PgConnection,
    organization_id: Uuid,
    req: &CreatePartnerRequest,
) -> Result<Partner, ApiError> {
    let new_partner = partner_db::insert(pool, organization_id, req).await?;
    Ok(new_partner)
}

pub async fn update_partner(
    pool: &mut PgConnection,
    organization_id: Uuid,
    partner_id: Uuid,
    req: &UpdatePartnerRequest,
) -> Result<Partner, ApiError> {
    let mut tx = pool.begin().await?;
    let updated_partner = partner_db::update(&mut tx, partner_id, req).await?;
    tx.commit().await?;
    Ok(updated_partner)
}

pub async fn delete_partner(pool: &mut PgConnection, partner_id: Uuid) -> Result<u64, ApiError> {
    let rows_affected = partner_db::delete(pool, partner_id).await?;
    Ok(rows_affected)
}

pub async fn create_address(
    pool: &mut PgConnection,
    organization_id: Uuid,
    partner_id: Uuid,
    address: &PartnerAddress,
) -> Result<PartnerAddress, ApiError> {
    let new_address =
        partner_db::insert_address(pool, organization_id, partner_id, address).await?;
    Ok(new_address)
}

pub async fn update_address(
    pool: &mut PgConnection,
    organization_id: Uuid,
    address_id: Uuid,
    address: &PartnerAddress,
) -> Result<PartnerAddress, ApiError> {
    let updated_address =
        partner_db::update_address(pool, organization_id, address_id, address).await?;
    Ok(updated_address)
}

pub async fn delete_address(pool: &mut PgConnection, address_id: Uuid) -> Result<u64, ApiError> {
    let rows_affected = partner_db::delete_address(pool, address_id).await?;
    Ok(rows_affected)
}

pub async fn create_contact(
    pool: &mut PgConnection,
    organization_id: Uuid,
    partner_id: Uuid,
    contact: &PartnerContact,
) -> Result<PartnerContact, ApiError> {
    let new_contact =
        partner_db::insert_contact(pool, organization_id, partner_id, contact).await?;
    Ok(new_contact)
}

pub async fn update_contact(
    pool: &mut PgConnection,
    organization_id: Uuid,
    contact_id: Uuid,
    contact: &PartnerContact,
) -> Result<PartnerContact, ApiError> {
    let updated_contact =
        partner_db::update_contact(pool, organization_id, contact_id, contact).await?;
    Ok(updated_contact)
}

pub async fn delete_contact(pool: &mut PgConnection, contact_id: Uuid) -> Result<u64, ApiError> {
    let rows_affected = partner_db::delete_contact(pool, contact_id).await?;
    Ok(rows_affected)
}
