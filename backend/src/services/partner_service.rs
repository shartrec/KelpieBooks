use crate::db::partner as partner_db;
use crate::util::ApiError;
use rocket_db_pools::sqlx::{self, PgConnection};
use sqlx::Acquire;
use shared_core::dtos::partner_list_item::PartnerListItem;
use shared_core::models::partner::Partner;
use shared_core::models::partner_address::PartnerAddress;
use shared_core::models::partner_contact::PartnerContact;
use shared_core::requests::partner::{CreatePartnerRequest, UpdatePartnerRequest};
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
    partner_db::delete_addresses(&mut tx, partner_id).await?;
    for address in &req.addresses {
        partner_db::insert_address(&mut tx, organization_id, partner_id, address).await?;
    }
    partner_db::delete_contacts(&mut tx, partner_id).await?;
    for contact in &req.contacts {
        partner_db::insert_contact(&mut tx, organization_id, partner_id, contact).await?;
    }
    tx.commit().await?;
    Ok(updated_partner)
}

pub async fn delete_partner(
    pool: &mut PgConnection,
    partner_id: Uuid,
) -> Result<u64, ApiError> {
    let rows_affected = partner_db::delete(pool, partner_id).await?;
    Ok(rows_affected)
}
