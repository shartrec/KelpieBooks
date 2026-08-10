/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use rocket_db_pools::sqlx::{
    self,
    PgConnection,
    Row,
};
use rust_decimal::dec;
use shared_core::{
    ledger::{
        models::system_tag::SystemTag,
        requests::transaction::{
            CreateTransactionRequest,
            JournalEntryLine,
        },
    },
    sales::{
        dtos::sales_invoice_list_item::SalesInvoiceListItem,
        models::{
            invoice_address::InvoiceAddress,
            invoice_status::InvoiceStatus,
            sales_invoice::SalesInvoice,
            sales_invoice_item::SalesInvoiceItem,
            sales_order::SalesOrder,
        },
        requests::sales_invoice::CreateSalesInvoiceRequest,
    },
};
use sqlx::Acquire;
use uuid::Uuid;

use crate::{
    core::db::sequences::{
        get_next_invoice_number,
        SeqType,
    },
    ledger::{
        db::account as account_db,
        services::account_service,
    },
    sales::db::{
        item as item_db,
        sales_invoice as sales_invoice_db,
    },
    util::ApiError,
};

pub(crate) async fn create_invoice(
    pool: &mut PgConnection,
    org_id: Uuid,
    req: &CreateSalesInvoiceRequest,
) -> Result<SalesInvoice, ApiError> {
    let mut tx = pool.begin().await?;

    // Get next invoice number
    let inv_number = get_next_invoice_number(&mut tx, org_id, &SeqType::SalesInvoice).await?;

    // Resolve address snapshots: prefer provided snapshots; if empty and IDs provided, load from DB
    let resolve_snapshot =
        |snap: &InvoiceAddress, maybe_id: Option<Uuid>| -> (Option<Uuid>, InvoiceAddress) {
            // Determine if snapshot is effectively empty (all fields None or empty strings)
            let is_empty = snap.name.as_deref().map_or(true, |s| s.trim().is_empty())
                && snap
                    .attention
                    .as_deref()
                    .map_or(true, |s| s.trim().is_empty())
                && snap
                    .address_line1
                    .as_deref()
                    .map_or(true, |s| s.trim().is_empty())
                && snap
                    .address_line2
                    .as_deref()
                    .map_or(true, |s| s.trim().is_empty())
                && snap.city.as_deref().map_or(true, |s| s.trim().is_empty())
                && snap
                    .state_province
                    .as_deref()
                    .map_or(true, |s| s.trim().is_empty())
                && snap
                    .postal_code
                    .as_deref()
                    .map_or(true, |s| s.trim().is_empty())
                && snap
                    .country
                    .as_deref()
                    .map_or(true, |s| s.trim().is_empty());
            (
                maybe_id,
                if is_empty {
                    InvoiceAddress::default()
                } else {
                    snap.clone()
                },
            )
        };

    let (billing_id, mut bill_to_snap) = resolve_snapshot(&req.bill_to, req.billing_address_id);
    let (shipping_id, mut ship_to_snap) = resolve_snapshot(&req.ship_to, req.shipping_address_id);

    // Helper to load partner address and copy fields into snapshot if snapshot empty
    async fn load_address_into_snapshot(
        tx: &mut PgConnection,
        org_id: Uuid,
        partner_id: Uuid,
        address_id: Uuid,
        target: &mut InvoiceAddress,
    ) -> Result<(), ApiError> {
        let row = sqlx::query(
            r#"
            SELECT id, organization_id, partner_id,
                   address_line1, address_line2, city, state_province, postal_code, country
            FROM partner_addresses
            WHERE id = $1 AND organization_id = $2 AND partner_id = $3
            "#,
        )
        .bind(address_id)
        .bind(org_id)
        .bind(partner_id)
        .fetch_optional(&mut *tx)
        .await?;

        match row {
            Some(r) => {
                // Only fill missing fields, allow frontend overrides to take precedence
                if target
                    .address_line1
                    .as_deref()
                    .map_or(true, |s| s.is_empty())
                {
                    target.address_line1 = Some(r.get::<String, _>("address_line1"));
                }
                if target
                    .address_line2
                    .as_deref()
                    .map_or(true, |s| s.is_empty())
                {
                    target.address_line2 = r.try_get::<String, _>("address_line2").ok();
                }
                if target.city.as_deref().map_or(true, |s| s.is_empty()) {
                    target.city = r.try_get::<String, _>("city").ok();
                }
                if target
                    .state_province
                    .as_deref()
                    .map_or(true, |s| s.is_empty())
                {
                    target.state_province = r.try_get::<String, _>("state_province").ok();
                }
                if target.postal_code.as_deref().map_or(true, |s| s.is_empty()) {
                    target.postal_code = r.try_get::<String, _>("postal_code").ok();
                }
                if target.country.as_deref().map_or(true, |s| s.is_empty()) {
                    target.country = r.try_get::<String, _>("country").ok();
                }
                Ok(())
            }
            None => Err(ApiError::BadRequest(
                "Invalid address selection for this partner/organization".to_string(),
            )),
        }
    }

    if let Some(id) = billing_id {
        load_address_into_snapshot(&mut tx, org_id, req.partner_id, id, &mut bill_to_snap).await?;
    }
    if let Some(id) = shipping_id {
        load_address_into_snapshot(&mut tx, org_id, req.partner_id, id, &mut ship_to_snap).await?;
    }

    let mut invoice = sales_invoice_db::create_draft_invoice(
        &mut tx,
        org_id,
        req.partner_id,
        &inv_number,
        req.issue_date,
        req.due_date,
        billing_id,
        shipping_id,
        bill_to_snap.name.as_deref(),
        bill_to_snap.attention.as_deref(),
        bill_to_snap.address_line1.as_deref(),
        bill_to_snap.address_line2.as_deref(),
        bill_to_snap.city.as_deref(),
        bill_to_snap.state_province.as_deref(),
        bill_to_snap.postal_code.as_deref(),
        bill_to_snap.country.as_deref(),
        ship_to_snap.name.as_deref(),
        ship_to_snap.attention.as_deref(),
        ship_to_snap.address_line1.as_deref(),
        ship_to_snap.address_line2.as_deref(),
        ship_to_snap.city.as_deref(),
        ship_to_snap.state_province.as_deref(),
        ship_to_snap.postal_code.as_deref(),
        ship_to_snap.country.as_deref(),
    )
    .await?;

    for line in &req.lines {
        if line.item_id == Uuid::nil() {
            continue;
        }
        sales_invoice_db::insert_sales_invoice_line(&mut tx, invoice.id, org_id, line).await?;
    }

    invoice.lines = req.lines.clone();
    invoice.calculate();

    sales_invoice_db::update_sales_invoice_totals(
        &mut tx,
        invoice.id,
        invoice.subtotal,
        invoice.tax_total,
        invoice.total_amount,
        invoice.total_amount,
    )
    .await?;
    // 1. Fetch Accounts Receivable (Asset) and Tax Clearing accounts
    let ar_account = account_db::get_by_system_tag(&mut tx, org_id, &SystemTag::AccountsReceivable)
        .await?
        .ok_or_else(|| ApiError::NotFound("Accounts Receivable account not found.".to_string()))?;
    let tax_account = account_db::get_by_system_tag(&mut tx, org_id, &SystemTag::SalesTaxClearing)
        .await?
        .ok_or_else(|| ApiError::NotFound("Tax account not found.".to_string()))?;

    let mut jels = vec![];

    // 2. Loop through lines to credit the revenue accounts
    for line in &req.lines {
        if line.net_amount > dec!(0.00) {
            let item_master = item_db::get(&mut tx, org_id, line.item_id)
                .await?
                .ok_or_else(|| ApiError::NotFound("Item not found.".to_string()))?;

            // Sales revenue lines are CREDITED
            let jel = JournalEntryLine {
                line_id: Uuid::new_v4(),
                account_id: item_master.income_account_id,
                debit: dec!(0.00),
                credit: line.net_amount,
                description: Some(line.description.clone()),
            };
            jels.push(jel);
        }
    }

    // 3. Add tax liability entry if applicable
    if invoice.tax_total > dec!(0.00) {
        // Tax collected from customers is a liability or clearing credit
        let jel = JournalEntryLine {
            line_id: Uuid::new_v4(),
            account_id: tax_account.id,
            debit: dec!(0.00),
            credit: invoice.tax_total,
            description: Some(format!(
                "Tax collected on invoice {}",
                invoice.invoice_number
            )),
        };
        jels.push(jel);
    }

    // 4. Add the balancing Accounts Receivable debit entry
    let jel = JournalEntryLine {
        line_id: Uuid::new_v4(),
        account_id: ar_account.id,
        debit: invoice.total_amount, // Gross amount (Net + Tax)
        credit: dec!(0.00),
        description: Some(format!("Customer sales invoice summary")),
    };
    jels.push(jel);

    // 5. Fire off transaction registration
    let ct_req = CreateTransactionRequest {
        date: invoice.issue_date, // Matches the invoice document date rather than Local time
        description: Some(format!("Sales Invoice {}", invoice.invoice_number)),
        reference: Some(invoice.invoice_number.clone()),
        entries: jels,
    };

    let _transaction_id = account_service::create_transaction(&mut tx, org_id, &ct_req).await?;

    tx.commit().await?;

    Ok(invoice)
}

pub(crate) async fn update_sales_invoice(
    pool: &mut PgConnection,
    org_id: Uuid,
    req: &shared_core::sales::requests::sales_invoice::UpdateSalesInvoiceRequest,
) -> Result<SalesInvoice, ApiError> {
    // 1. Fetch current record first to verify ownership tenancy & structural lock eligibility
    let current_invoice =
        sales_invoice_db::get_sales_invoice_with_lines(&mut *pool, req.id, org_id)
            .await?
            .ok_or_else(|| ApiError::NotFound("Sales invoice not found.".to_string()))?;

    if current_invoice.org_id != org_id {
        return Err(ApiError::Forbidden(
            "You do not have permission to modify this invoice.".to_string(),
        ));
    }

    // 💡 Accounting Guard: Prevent modifying historical addresses/dates if already finalized
    if current_invoice.status == InvoiceStatus::Paid {
        return Err(ApiError::BadRequest(
            "Cannot update details on an invoice that has already been marked as Paid.".to_string(),
        ));
    }

    // 2. Perform the database write update operation
    sales_invoice_db::update_sales_invoice(&mut *pool, org_id, req).await?;

    // 3. Fetch and return the freshly updated invoice record layout matching our web UI view loop
    let updated_invoice =
        sales_invoice_db::get_sales_invoice_with_lines(&mut *pool, req.id, org_id)
            .await?
            .ok_or_else(|| {
                ApiError::NotFound("Failed to retrieve updated invoice details.".to_string())
            })?;

    Ok(updated_invoice)
}

pub(crate) async fn update_invoice_items(
    pool: &mut PgConnection,
    org_id: Uuid,
    invoice_id: Uuid,
    lines: &[SalesInvoiceItem],
) -> Result<SalesInvoice, ApiError> {
    let mut invoice = sales_invoice_db::get_sales_invoice_with_lines(pool, invoice_id, org_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Invoice not found.".to_string()))?;

    if invoice.org_id != org_id {
        return Err(ApiError::Forbidden(
            "You do not have permission to update this invoice.".to_string(),
        ));
    }

    let mut tx = pool.begin().await?;
    sales_invoice_db::delete_sales_invoice_lines(&mut tx, invoice_id).await?;
    for line in lines {
        sales_invoice_db::insert_sales_invoice_line(&mut tx, invoice_id, org_id, line).await?;
    }

    invoice.lines = lines.to_vec();
    invoice.calculate();

    sales_invoice_db::update_sales_invoice_totals(
        &mut tx,
        invoice.id,
        invoice.subtotal,
        invoice.tax_total,
        invoice.total_amount,
        invoice.amount_due,
    )
    .await?;

    tx.commit().await?;

    Ok(invoice)
}

pub(crate) async fn get_sales_invoice(
    pool: &mut PgConnection,
    org_id: Uuid,
    invoice_id: Uuid,
) -> Result<SalesInvoice, ApiError> {
    let invoice = sales_invoice_db::get_sales_invoice_with_lines(pool, invoice_id, org_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Invoice not found.".to_string()))?;

    if invoice.org_id != org_id {
        return Err(ApiError::Forbidden(
            "You do not have permission to view this invoice.".to_string(),
        ));
    }

    Ok(invoice)
}

pub(crate) async fn get_sales_invoices(
    pool: &mut PgConnection,
    org_id: Uuid,
    start_date: Option<chrono::NaiveDate>,
    end_date: Option<chrono::NaiveDate>,
    partner_id: Option<Uuid>,
    min_amount: Option<rust_decimal::Decimal>,
    statuses: Option<Vec<InvoiceStatus>>,
) -> Result<Vec<SalesInvoiceListItem>, ApiError> {
    let items = sales_invoice_db::list_sales_invoices(
        pool, org_id, start_date, end_date, partner_id, min_amount, statuses,
    )
    .await?;
    Ok(items)
}

/// Creates a `SalesInvoice` (status `Open`) from a confirmed `SalesOrder`.
///
/// This function must be called within an already-open database transaction.  It does NOT
/// begin or commit its own transaction — the caller is responsible for the outer transaction
/// boundary.  This allows `confirm_order` to keep the stock allocation and invoice creation
/// within a single ACID transaction.
pub(crate) async fn create_invoice_from_order(
    tx: &mut PgConnection,
    org_id: Uuid,
    order: &SalesOrder,
) -> Result<SalesInvoice, ApiError> {
    // Generate invoice number from the SalesInvoice sequence
    let inv_number = get_next_invoice_number(tx, org_id, &SeqType::SalesInvoice).await?;

    // Use today as issue/due date (no due date on order; can be adjusted later)
    let today = chrono::Local::now().date_naive();

    let mut invoice = sales_invoice_db::create_draft_invoice(
        tx,
        org_id,
        order.partner_id,
        &inv_number,
        today,
        today,
        order.billing_address_id,
        order.shipping_address_id,
        order.bill_to.name.as_deref(),
        order.bill_to.attention.as_deref(),
        order.bill_to.address_line1.as_deref(),
        order.bill_to.address_line2.as_deref(),
        order.bill_to.city.as_deref(),
        order.bill_to.state_province.as_deref(),
        order.bill_to.postal_code.as_deref(),
        order.bill_to.country.as_deref(),
        order.ship_to.name.as_deref(),
        order.ship_to.attention.as_deref(),
        order.ship_to.address_line1.as_deref(),
        order.ship_to.address_line2.as_deref(),
        order.ship_to.city.as_deref(),
        order.ship_to.state_province.as_deref(),
        order.ship_to.postal_code.as_deref(),
        order.ship_to.country.as_deref(),
    )
    .await?;

    // Map order lines to invoice lines
    for line in &order.lines {
        if line.item_id == Uuid::nil() {
            continue;
        }
        let invoice_line = SalesInvoiceItem {
            id: Uuid::nil(),      // DB will assign
            invoice_id: invoice.id,
            item_id: line.item_id,
            code: line.code.clone(),
            name: line.name.clone(),
            description: line.description.clone(),
            quantity: line.quantity,
            unit_price: line.unit_price,
            tax_category_id: line.tax_category_id,
            tax_rate: line.tax_rate,
            tax_amount: line.tax_amount,
            net_amount: line.net_amount,
            sort_order: line.sort_order,
        };
        sales_invoice_db::insert_sales_invoice_line(tx, invoice.id, org_id, &invoice_line).await?;
    }

    invoice.lines = order
        .lines
        .iter()
        .map(|l| SalesInvoiceItem {
            id: Uuid::nil(),
            invoice_id: invoice.id,
            item_id: l.item_id,
            code: l.code.clone(),
            name: l.name.clone(),
            description: l.description.clone(),
            quantity: l.quantity,
            unit_price: l.unit_price,
            tax_category_id: l.tax_category_id,
            tax_rate: l.tax_rate,
            tax_amount: l.tax_amount,
            net_amount: l.net_amount,
            sort_order: l.sort_order,
        })
        .collect();
    invoice.calculate();

    sales_invoice_db::update_sales_invoice_totals(
        tx,
        invoice.id,
        invoice.subtotal,
        invoice.tax_total,
        invoice.total_amount,
        invoice.total_amount,
    )
    .await?;

    // Post GL entries (AR debit, revenue credits, tax credit)
    let ar_account = account_db::get_by_system_tag(tx, org_id, &SystemTag::AccountsReceivable)
        .await?
        .ok_or_else(|| ApiError::NotFound("Accounts Receivable account not found.".to_string()))?;
    let tax_account = account_db::get_by_system_tag(tx, org_id, &SystemTag::SalesTaxClearing)
        .await?
        .ok_or_else(|| ApiError::NotFound("Tax account not found.".to_string()))?;

    let mut jels = vec![];

    for line in &order.lines {
        if line.net_amount > dec!(0.00) {
            let item_master = item_db::get(tx, org_id, line.item_id)
                .await?
                .ok_or_else(|| ApiError::NotFound("Item not found.".to_string()))?;

            jels.push(JournalEntryLine {
                line_id: Uuid::new_v4(),
                account_id: item_master.income_account_id,
                debit: dec!(0.00),
                credit: line.net_amount,
                description: Some(line.description.clone()),
            });
        }
    }

    if invoice.tax_total > dec!(0.00) {
        jels.push(JournalEntryLine {
            line_id: Uuid::new_v4(),
            account_id: tax_account.id,
            debit: dec!(0.00),
            credit: invoice.tax_total,
            description: Some(format!("Tax collected on invoice {}", invoice.invoice_number)),
        });
    }

    jels.push(JournalEntryLine {
        line_id: Uuid::new_v4(),
        account_id: ar_account.id,
        debit: invoice.total_amount,
        credit: dec!(0.00),
        description: Some("Customer sales invoice summary".to_string()),
    });

    let ct_req = CreateTransactionRequest {
        date: invoice.issue_date,
        description: Some(format!("Sales Invoice {}", invoice.invoice_number)),
        reference: Some(invoice.invoice_number.clone()),
        entries: jels,
    };

    account_service::create_transaction(tx, org_id, &ct_req).await?;

    Ok(invoice)
}
