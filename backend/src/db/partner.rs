/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use rocket_db_pools::sqlx::{self, PgConnection, Row};
use shared_core::dtos::partner_list_item::PartnerListItem;
use shared_core::models::address_type::AddressType;
use shared_core::models::partner::Partner;
use shared_core::models::partner_address::PartnerAddress;
use shared_core::models::partner_contact::PartnerContact;
use shared_core::requests::partner::{CreatePartnerRequest, UpdatePartnerRequest};
use std::str::FromStr;
use uuid::Uuid;

fn from_row_to_partner(row: &sqlx::postgres::PgRow) -> Partner {
    Partner {
        id: row.get("id"),
        organization_id: row.get("organization_id"),
        legal_name: row.get("legal_name"),
        trade_name: row.get("trade_name"),
        tax_identifier: row.get("tax_identifier"),
        is_vendor: row.get("is_vendor"),
        is_customer: row.get("is_customer"),
        default_ap_account_id: row.get("default_ap_account_id"),
        default_ar_account_id: row.get("default_ar_account_id"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

fn from_row_to_partner_address(row: &sqlx::postgres::PgRow) -> PartnerAddress {
    let address_type_str: String = row.get("address_type");
    let address_type = AddressType::from_str(&address_type_str).unwrap();
    PartnerAddress {
        id: row.get("id"),
        organization_id: row.get("organization_id"),
        partner_id: row.get("partner_id"),
        address_type,
        is_primary: row.get("is_primary"),
        address_line1: row.get("address_line1"),
        address_line2: row.get("address_line2"),
        city: row.get("city"),
        state_province: row.get("state_province"),
        postal_code: row.get("postal_code"),
        country: row.get("country"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

fn from_row_to_partner_contact(row: &sqlx::postgres::PgRow) -> PartnerContact {
    PartnerContact {
        id: row.get("id"),
        organization_id: row.get("organization_id"),
        partner_id: row.get("partner_id"),
        is_primary: row.get("is_primary"),
        full_name: row.get("full_name"),
        preferred_name: row.get("preferred_name"),
        email: row.get("email"),
        phone: row.get("phone"),
        role_title: row.get("role_title"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

fn from_row_to_partner_list_item(row: &sqlx::postgres::PgRow) -> PartnerListItem {
    PartnerListItem {
        id: row.get("id"),
        legal_name: row.get("legal_name"),
        trade_name: row.get("trade_name"),
        is_vendor: row.get("is_vendor"),
        is_customer: row.get("is_customer"),
        can_delete: row.get("can_delete"),
    }
}

pub(crate) async fn get(pool: &mut PgConnection, id: Uuid) -> Result<Option<Partner>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT *
        FROM partners
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map(|row| row.map(|r| from_row_to_partner(&r)))
}

pub(crate) async fn get_addresses(
    pool: &mut PgConnection,
    partner_id: Uuid,
) -> Result<Vec<PartnerAddress>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT id, organization_id, partner_id, address_type::TEXT, is_primary, address_line1, address_line2, city, state_province, postal_code, country, created_at, updated_at
        FROM partner_addresses
        WHERE partner_id = $1
        "#,
    )
    .bind(partner_id)
    .fetch_all(pool)
    .await
    .map(|rows| rows.iter().map(from_row_to_partner_address).collect())
}

pub(crate) async fn get_contacts(
    pool: &mut PgConnection,
    partner_id: Uuid,
) -> Result<Vec<PartnerContact>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT *
        FROM partner_contacts
        WHERE partner_id = $1
        "#,
    )
    .bind(partner_id)
    .fetch_all(pool)
    .await
    .map(|rows| rows.iter().map(from_row_to_partner_contact).collect())
}

pub(crate) async fn get_all_by_org(
    pool: &mut PgConnection,
    organization_id: Uuid,
) -> Result<Vec<PartnerListItem>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT
            p.id,
            p.legal_name,
            p.trade_name,
            p.is_vendor,
            p.is_customer,
            CASE
                WHEN COUNT(vi.id) > 0 THEN FALSE
                ELSE TRUE
            END AS can_delete
        FROM
            partners p
        LEFT JOIN
            vendor_invoices vi ON p.id = vi.partner_id
        WHERE
            p.organization_id = $1
        GROUP BY
            p.id, p.legal_name
        ORDER BY
            p.legal_name
        "#,
    )
    .bind(organization_id)
    .fetch_all(pool)
    .await
    .map(|rows| rows.iter().map(from_row_to_partner_list_item).collect())
}

pub(crate) async fn insert(
    pool: &mut PgConnection,
    org_id: Uuid,
    req: &CreatePartnerRequest,
) -> Result<Partner, sqlx::Error> {
    let row = sqlx::query(
        r#"
        INSERT INTO partners (organization_id, legal_name, trade_name, tax_identifier, is_vendor, is_customer, default_ap_account_id, default_ar_account_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
        "#,
    )
    .bind(org_id)
    .bind(&req.legal_name)
    .bind(&req.trade_name)
    .bind(&req.tax_identifier)
    .bind(req.is_vendor)
    .bind(req.is_customer)
    .bind(req.default_ap_account_id)
    .bind(req.default_ar_account_id)
    .fetch_one(pool)
    .await?;
    Ok(from_row_to_partner(&row))
}

pub(crate) async fn update(
    pool: &mut PgConnection,
    org_id: Uuid,
    id: Uuid,
    req: &UpdatePartnerRequest,
) -> Result<Partner, sqlx::Error> {
    let row = sqlx::query(
        r#"
        UPDATE partners
        SET legal_name = $1, trade_name = $2, tax_identifier = $3, is_vendor = $4, is_customer = $5, default_ap_account_id = $6, default_ar_account_id = $7
        WHERE id = $8 AND organization_id = $9
        RETURNING *
        "#,
    )
    .bind(&req.legal_name)
    .bind(&req.trade_name)
    .bind(&req.tax_identifier)
    .bind(req.is_vendor)
    .bind(req.is_customer)
    .bind(req.default_ap_account_id)
    .bind(req.default_ar_account_id)
    .bind(id)
    .bind(org_id)
    .fetch_one(pool)
    .await?;
    Ok(from_row_to_partner(&row))
}

pub(crate) async fn delete(pool: &mut PgConnection, id: Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM partners WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}

pub(crate) async fn delete_addresses(
    pool: &mut PgConnection,
    partner_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM partner_addresses WHERE partner_id = $1")
        .bind(partner_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub(crate) async fn insert_address(
    pool: &mut PgConnection,
    organization_id: Uuid,
    partner_id: Uuid,
    address: &PartnerAddress,
) -> Result<PartnerAddress, sqlx::Error> {
    let row = sqlx::query(
        r#"
        INSERT INTO partner_addresses (organization_id, partner_id, address_type, is_primary, address_line1, address_line2, city, state_province, postal_code, country)
        VALUES ($1, $2, $3::address_type, $4, $5, $6, $7, $8, $9, $10)
        RETURNING id, organization_id, partner_id, address_type::TEXT, is_primary, address_line1, address_line2, city, state_province, postal_code, country, created_at, updated_at
        "#,
    )
    .bind(organization_id)
    .bind(partner_id)
    .bind(address.address_type.to_string())
    .bind(address.is_primary)
    .bind(&address.address_line1)
    .bind(&address.address_line2)
    .bind(&address.city)
    .bind(&address.state_province)
    .bind(&address.postal_code)
    .bind(&address.country)
    .fetch_one(pool)
    .await?;
    Ok(from_row_to_partner_address(&row))
}

pub(crate) async fn update_address(
    pool: &mut PgConnection,
    organization_id: Uuid,
    address_id: Uuid,
    address: &PartnerAddress,
) -> Result<PartnerAddress, sqlx::Error> {
    let row = sqlx::query(
        r#"
        UPDATE partner_addresses
        SET address_type = $1::address_type, is_primary = $2, address_line1 = $3, address_line2 = $4, city = $5, state_province = $6, postal_code = $7, country = $8
        WHERE id = $9 AND organization_id = $10
        RETURNING id, organization_id, partner_id, address_type::TEXT, is_primary, address_line1, address_line2, city, state_province, postal_code, country, created_at, updated_at
        "#,
    )
    .bind(address.address_type.to_string())
    .bind(address.is_primary)
    .bind(&address.address_line1)
    .bind(&address.address_line2)
    .bind(&address.city)
    .bind(&address.state_province)
    .bind(&address.postal_code)
    .bind(&address.country)
    .bind(address_id)
    .bind(organization_id)
    .fetch_one(pool)
    .await?;
    Ok(from_row_to_partner_address(&row))
}

pub(crate) async fn delete_address(pool: &mut PgConnection, id: Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM partner_addresses WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}

pub(crate) async fn delete_contacts(
    pool: &mut PgConnection,
    partner_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM partner_contacts WHERE partner_id = $1")
        .bind(partner_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub(crate) async fn insert_contact(
    pool: &mut PgConnection,
    organization_id: Uuid,
    partner_id: Uuid,
    contact: &PartnerContact,
) -> Result<PartnerContact, sqlx::Error> {
    let row = sqlx::query(
        r#"
        INSERT INTO partner_contacts (organization_id, partner_id, is_primary, full_name, preferred_name, email, phone, role_title)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
        "#,
    )
    .bind(organization_id)
    .bind(partner_id)
    .bind(contact.is_primary)
    .bind(&contact.full_name)
    .bind(&contact.preferred_name)
    .bind(&contact.email)
    .bind(&contact.phone)
    .bind(&contact.role_title)
    .fetch_one(pool)
    .await?;
    Ok(from_row_to_partner_contact(&row))
}

pub(crate) async fn update_contact(
    pool: &mut PgConnection,
    organization_id: Uuid,
    contact_id: Uuid,
    contact: &PartnerContact,
) -> Result<PartnerContact, sqlx::Error> {
    let row = sqlx::query(
        r#"
        UPDATE partner_contacts
        SET is_primary = $1, full_name = $2, preferred_name = $3, email = $4, phone = $5, role_title = $6
        WHERE id = $7 AND organization_id = $8
        RETURNING *
        "#,
    )
    .bind(contact.is_primary)
    .bind(&contact.full_name)
    .bind(&contact.preferred_name)
    .bind(&contact.email)
    .bind(&contact.phone)
    .bind(&contact.role_title)
    .bind(contact_id)
    .bind(organization_id)
    .fetch_one(pool)
    .await?;
    Ok(from_row_to_partner_contact(&row))
}

pub(crate) async fn delete_contact(pool: &mut PgConnection, id: Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM partner_contacts WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}
