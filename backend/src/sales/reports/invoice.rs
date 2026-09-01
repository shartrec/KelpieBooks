/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use std::{
    fs,
    path::{
        Path,
    },
};
use rocket::State;
use rocket_db_pools::Connection;
use shared_core::OrderId;
use typst_as_lib::{
    typst_kit_options::TypstKitFontOptions,
    TypstEngine,
};
use typst_assets::fonts;
use typst_library::foundations::{
    Array,
    Dict,
    Value,
};
use uuid::Uuid;
use crate::{
    core::{
        db::organization as db_org,
        routes::security::AuthenticatedUser,
    },
    inventory::db::{
        inventory::get_first_balance_for_item_warehouse,
        location::get_location,
    },
    sales::{
        db::sales_order::get_sales_order,
        services::{
            item_service::get_item,
            uom_service::get_uom,
        },
    },
    util::{
        locale_context::LocaleContext,
        ApiError,
    },
    DbKelpie,
    TemplateConfig,
};

pub(crate) async fn generate_invoice(
    conn: &mut Connection<DbKelpie>,
    user: AuthenticatedUser,
    config: &State<TemplateConfig>,
    order_id: OrderId,
) -> Result<Vec<u8>, ApiError> {
    let dict = gather_order_dictionary(conn, &user, order_id).await?;

    let template_dir = config.root_directory.to_string_lossy();

    let path = Path::new(&*template_dir);

    build_order_pdf(dict, path, "invoice_template.typ")
}
pub(crate) async fn generate_picklist(
    conn: &mut Connection<DbKelpie>,
    user: AuthenticatedUser,
    config: &State<TemplateConfig>,
    order_id: OrderId,
) -> Result<Vec<u8>, ApiError> {
    let dict = gather_order_dictionary(conn, &user, order_id).await?;

    let template_dir = config.root_directory.to_string_lossy();

    let path = Path::new(&*template_dir);

    build_order_pdf(dict, path, "picklist_template.typ")
}

async fn gather_order_dictionary(
    conn: &mut Connection<DbKelpie>,
    user: &AuthenticatedUser,
    order_id: OrderId,
) -> Result<Dict, ApiError> {
    let i18n = LocaleContext::new(&user.locale);

    // Gather the audit details and generate a structure to pass to the Typst template.
    let mut dict = Dict::new();

    if let Some(org) = db_org::get(conn, user.organization_id).await? {
        dict.insert("company-name".into(), Value::Str(org.name.into()));
    } else {
        return Err(ApiError::Forbidden("Organization ID does not exist".into()));
    }

    let order = get_sales_order(conn, user.organization_id, order_id).await?;

    if let Some(order_dto) = order {
        let order = order_dto.order;

        let uuid = order.id.hyphenated().encode_lower(&mut Uuid::encode_buffer()).to_string();
        dict.insert("order-id".into(), Value::Str(uuid.into()));
        dict.insert("order-number".into(), Value::Str(order.order_number.into()));
        let inv_due = i18n.format_date(order.due_date);
        dict.insert("due-date".into(), Value::Str(inv_due.into()));
        let order_date = i18n.format_date(order.order_date);
        dict.insert("order-date".into(), Value::Str(order_date.into()));

        let inv_net = i18n.format_money_typ(order.subtotal.round_dp(2));
        dict.insert("order-net".into(), Value::Str(inv_net.into()));
        let inv_tax = i18n.format_money_typ(order.tax_total.round_dp(2));
        dict.insert("order-tax".into(), Value::Str(inv_tax.into()));
        let inv_gross = i18n.format_money_typ(order.total_amount.round_dp(2));
        dict.insert("order-gross".into(), Value::Str(inv_gross.into()));

        let mut bill_to = Dict::new();
        bill_to.insert(
            "name".into(),
            Value::Str(order_dto.bill_to.name.unwrap_or_default().into()),
        );
        bill_to.insert(
            "attn".into(),
            Value::Str(order_dto.bill_to.attention.unwrap_or_default().into()),
        );
        bill_to.insert(
            "addr_line1".into(),
            Value::Str(order_dto.bill_to.line1.unwrap_or_default().into()),
        );
        bill_to.insert(
            "addr_line2".into(),
            Value::Str(order_dto.bill_to.line2.unwrap_or_default().into()),
        );
        bill_to.insert(
            "city".into(),
            Value::Str(order_dto.bill_to.city.unwrap_or_default().into()),
        );
        bill_to.insert(
            "state".into(),
            Value::Str(order_dto.bill_to.region.unwrap_or_default().into()),
        );
        bill_to.insert(
            "post_code".into(),
            Value::Str(order_dto.bill_to.postal_code.unwrap_or_default().into()),
        );
        dict.insert("bill_to".into(), Value::Dict(bill_to));

        let mut ship_to = Dict::new();
        ship_to.insert(
            "name".into(),
            Value::Str(order_dto.ship_to.name.unwrap_or_default().into()),
        );
        ship_to.insert(
            "attn".into(),
            Value::Str(order_dto.ship_to.attention.unwrap_or_default().into()),
        );
        ship_to.insert(
            "addr_line1".into(),
            Value::Str(order_dto.ship_to.line1.unwrap_or_default().into()),
        );
        ship_to.insert(
            "addr_line2".into(),
            Value::Str(order_dto.ship_to.line2.unwrap_or_default().into()),
        );
        ship_to.insert(
            "city".into(),
            Value::Str(order_dto.ship_to.city.unwrap_or_default().into()),
        );
        ship_to.insert(
            "state".into(),
            Value::Str(order_dto.ship_to.region.unwrap_or_default().into()),
        );
        ship_to.insert(
            "post_code".into(),
            Value::Str(order_dto.ship_to.postal_code.unwrap_or_default().into()),
        );
        dict.insert("ship_to".into(), Value::Dict(ship_to));

        // Now we add the lines to an array each as a Dict
        let mut lines = Array::new();
        for line in order_dto.items {
            let mut item = Dict::new();
            item.insert("name".into(), Value::Str(line.name.into()));
            item.insert("code".into(), Value::Str(line.code.into()));
            let qty = i18n.format_decimal_typ(line.quantity.normalize());
            item.insert("qty".into(), Value::Str(qty.into()));
            let up = i18n.format_money_typ(line.unit_price);
            item.insert("unit_price".into(), Value::Str(up.into()));
            let net = i18n.format_money_typ(line.net_amount.round_dp(2));
            item.insert("net".into(), Value::Str(net.into()));
            let tax = i18n.format_money_typ(line.tax_amount.round_dp(2));
            item.insert("tax".into(), Value::Str(tax.into()));
            let gross = i18n.format_money_typ(line.net_amount.round_dp(2));
            item.insert("gross".into(), Value::Str(gross.into()));

            #[cfg(feature = "inventory")]
            {
                let it = get_item(conn, order.org_id, line.item_id).await?;
                if let Some(it) = it {
                    let uom = get_uom(conn, order.org_id, it.uom_id).await?;
                    if let Some(uom) = uom {
                        item.insert("uom".into(), Value::Str(uom.name.into()));
                    }
                }
                let wib = get_first_balance_for_item_warehouse(
                    conn,
                    order.org_id,
                    line.item_id,
                    order.warehouse_id,
                )
                .await?;
                if let Some(wib) = wib {
                    let wl = get_location(conn, order.org_id, wib.location_id).await?;
                    if let Some(wl) = wl {
                        item.insert("location".into(), Value::Str(wl.display_label.into()));
                    }
                }
            }

            lines.push(Value::Dict(item));
        }
        dict.insert("lines".into(), Value::Array(lines));
    }
    Ok(dict)
}

fn build_order_pdf(order: Dict, template_dir: &Path, template: &str ) -> Result<Vec<u8>, ApiError> {

    let path = Path::new(&*template_dir).join(template);

    let template_source = fs::read_to_string(path)
        .map_err(|e| ApiError::Internal(format!("Failed to read template file: {}", e)))?;

    let template = TypstEngine::builder()
        .with_file_system_resolver(template_dir)
        .main_file(template_source)
        .fonts(fonts())
        .search_fonts_with(TypstKitFontOptions::default())
        .build();

    // Run it
    let doc = template.compile_with_input(order).output;
    match doc {
        Ok(doc) => {
            let options = Default::default();
            let pdf = typst_pdf::pdf(&doc, &options).expect("Could not generate pdf.");
            Ok(pdf)
        }
        Err(e) => Err(ApiError::Internal(format!(
            "typst::compile() returned an error!: {}",
            e
        ))),
    }
}
