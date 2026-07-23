/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
use std::fs;

use rocket::State;
use rocket_db_pools::Connection;
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
    sales::db::sales_invoice::get_sales_invoice_with_lines,
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
    invoice_id: Uuid,
) -> Result<Vec<u8>, ApiError> {
    let i18n = LocaleContext::new(&user.locale);
    let template_dir = config.root_directory.to_string_lossy();

    // Gather the audit details and generate a structure to pass to the Typst template.
    let mut dict = Dict::new();

    if let Some(org) = db_org::get(conn, user.organization_id).await? {
        dict.insert("company-name".into(), Value::Str(org.name.into()));
    } else {
        return Err(ApiError::Forbidden("Organization ID does not exist".into()));
    }

    let invoice = get_sales_invoice_with_lines(conn, invoice_id, user.organization_id).await?;

    if let Some(invoice) = invoice {
        dict.insert(
            "invoice-number".into(),
            Value::Str(invoice.invoice_number.into()),
        );
        let inv_due = i18n.format_date(invoice.due_date);
        dict.insert("due-date".into(), Value::Str(inv_due.into()));
        let invoice_date = i18n.format_date(invoice.issue_date);
        dict.insert("invoice-date".into(), Value::Str(invoice_date.into()));

        let inv_net = i18n.format_money_typ(invoice.subtotal.round_dp(2));
        dict.insert("invoice-net".into(), Value::Str(inv_net.into()));
        let inv_tax = i18n.format_money_typ(invoice.tax_total.round_dp(2));
        dict.insert("invoice-tax".into(), Value::Str(inv_tax.into()));
        let inv_gross = i18n.format_money_typ(invoice.total_amount.round_dp(2));
        dict.insert("invoice-gross".into(), Value::Str(inv_gross.into()));

        let mut bill_to = Dict::new();
        bill_to.insert(
            "name".into(),
            Value::Str(invoice.bill_to.name.unwrap_or_default().into()),
        );
        bill_to.insert(
            "attn".into(),
            Value::Str(invoice.bill_to.attention.unwrap_or_default().into()),
        );
        bill_to.insert(
            "addr_line1".into(),
            Value::Str(invoice.bill_to.address_line1.unwrap_or_default().into()),
        );
        bill_to.insert(
            "addr_line2".into(),
            Value::Str(invoice.bill_to.address_line2.unwrap_or_default().into()),
        );
        bill_to.insert(
            "city".into(),
            Value::Str(invoice.bill_to.city.unwrap_or_default().into()),
        );
        bill_to.insert(
            "state".into(),
            Value::Str(invoice.bill_to.state_province.unwrap_or_default().into()),
        );
        bill_to.insert(
            "post_code".into(),
            Value::Str(invoice.bill_to.postal_code.unwrap_or_default().into()),
        );
        dict.insert("bill_to".into(), Value::Dict(bill_to));

        let mut ship_to = Dict::new();
        ship_to.insert(
            "name".into(),
            Value::Str(invoice.ship_to.name.unwrap_or_default().into()),
        );
        ship_to.insert(
            "attn".into(),
            Value::Str(invoice.ship_to.attention.unwrap_or_default().into()),
        );
        ship_to.insert(
            "addr_line1".into(),
            Value::Str(invoice.ship_to.address_line1.unwrap_or_default().into()),
        );
        ship_to.insert(
            "addr_line2".into(),
            Value::Str(invoice.ship_to.address_line2.unwrap_or_default().into()),
        );
        ship_to.insert(
            "city".into(),
            Value::Str(invoice.ship_to.city.unwrap_or_default().into()),
        );
        ship_to.insert(
            "state".into(),
            Value::Str(invoice.ship_to.state_province.unwrap_or_default().into()),
        );
        ship_to.insert(
            "post_code".into(),
            Value::Str(invoice.ship_to.postal_code.unwrap_or_default().into()),
        );
        dict.insert("ship_to".into(), Value::Dict(ship_to));

        // Now we add the lines to an array each as a Dict
        let mut lines = Array::new();
        for line in invoice.lines {
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

            lines.push(Value::Dict(item));
        }
        dict.insert("lines".into(), Value::Array(lines));
    }
    build_invoice_pdf(dict, &*template_dir)
}

fn build_invoice_pdf(invoice: Dict, template_path: &str) -> Result<Vec<u8>, ApiError> {
    let template_source = fs::read_to_string(template_path)
        .map_err(|e| ApiError::Internal(format!("Failed to read template file: {}", e)))?;

    let template = TypstEngine::builder()
        .main_file(template_source)
        .fonts(fonts())
        .search_fonts_with(TypstKitFontOptions::default())
        .build();

    // Run it
    let doc = template.compile_with_input(invoice).output;
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
