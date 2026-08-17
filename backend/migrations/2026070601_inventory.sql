/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

ALTER TYPE system_privilege ADD VALUE IF NOT EXISTS 'use_inventory';
ALTER TYPE system_privilege ADD VALUE IF NOT EXISTS 'manage_inventory';

ALTER TYPE system_tag ADD VALUE IF NOT EXISTS 'inventory_asset';
ALTER TYPE system_tag ADD VALUE IF NOT EXISTS 'received_not_invoiced';
ALTER TYPE system_tag ADD VALUE IF NOT EXISTS 'inventory_adjustment';

DROP TYPE sales_order_status;

-- 1. Physical / Dispatch Tracking
CREATE TYPE fulfillment_status AS ENUM (
    'unfulfilled',        -- Nothing shipped yet / Service pending
    'partially_fulfilled', -- Partial shipment
    'fulfilled',          -- Shipped / Delivered / Service Completed
    'not_required'        -- Pure digital/service lines with no delivery step
    );

-- 2. Financial / Billing Tracking
CREATE TYPE payment_status AS ENUM (
    'unpaid',             -- Invoice issued, $0 received
    'partially_paid',      -- Deposit or partial payment received
    'paid',               -- Fully settled
    'refunded'            -- Voided / Returned
    );

-- 3. Document Lifecycle (Overall Document State)
CREATE TYPE sales_document_status AS ENUM (
    'draft',              -- Quote / Unapproved Draft
    'open',               -- Approved & Active
    'completed',          -- Fully Fulfilled AND Fully Paid
    'cancelled'           -- Voided
    );

CREATE TYPE stock_transaction_type AS ENUM (
    'receipt',
    'adjustment',
    'allocation',
    'pick',
    'shipment'
    );

CREATE TYPE reference_type AS ENUM (
    'purchase_order',
    'sales_order',
    'manual_adjustment',
    'cycle_count'
    );

-- 1. Add cost tracking fields to Item master
ALTER TABLE items
    ADD COLUMN purchase_unit_cost DECIMAL(12, 4) NOT NULL DEFAULT 0.0000;

CREATE TABLE item_warehouse_profiles (
                                         item_id UUID PRIMARY KEY REFERENCES items(id) ON DELETE CASCADE,
                                         organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Physical Dimensions (Using standard numeric precision for physical units)
                                         weight_kg NUMERIC(10,3) DEFAULT 0.000,
                                         length_cm NUMERIC(10,2) DEFAULT 0.00,
                                         width_cm  NUMERIC(10,2) DEFAULT 0.00,
                                         height_cm NUMERIC(10,2) DEFAULT 0.00,

    -- Warehouse Management Controls
                                         reorder_point NUMERIC(15,4) DEFAULT 0.0000,
                                         safety_stock  NUMERIC(15,4) DEFAULT 0.0000,

                                         updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_item_wh_profiles_org ON item_warehouse_profiles(organization_id);

CREATE TABLE warehouses (
                            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                            organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
                            code VARCHAR(20) NOT NULL, -- e.g., 'WH-SYD', 'WH-MELB'
                            name VARCHAR(100) NOT NULL,
                            is_active BOOLEAN NOT NULL DEFAULT true,
                            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

                            CONSTRAINT uq_warehouse_code_per_org UNIQUE (organization_id, code)
);

CREATE TABLE warehouse_locations (
                                     id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                                     organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
                                     warehouse_id UUID NOT NULL REFERENCES warehouses(id) ON DELETE CASCADE,

                                     zone VARCHAR(20) DEFAULT '',         -- e.g., 'Bulk', 'Cold Storage'
                                     aisle VARCHAR(10) DEFAULT '',        -- e.g., 'A1', 'B5'
                                     shelf VARCHAR(10) DEFAULT '',        -- e.g., 'S3'
                                     bin VARCHAR(10) DEFAULT '',          -- e.g., 'B02'

                                     display_label VARCHAR(50) NOT NULL, -- e.g., 'A1-S3-B02'
                                     is_picking_location BOOLEAN NOT NULL DEFAULT true,

                                     created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_wh_locations_lookup ON warehouse_locations(warehouse_id, display_label);

CREATE TABLE warehouse_inventory_balances (
                                              id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                                              organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
                                              item_id UUID NOT NULL REFERENCES items(id) ON DELETE RESTRICT,
                                              warehouse_id UUID NOT NULL REFERENCES warehouses(id) ON DELETE CASCADE,
                                              location_id UUID NOT NULL REFERENCES warehouse_locations(id) ON DELETE RESTRICT,

                                              quantity_on_hand NUMERIC(15,4) NOT NULL DEFAULT 0.0000,
                                              quantity_allocated NUMERIC(15,4) NOT NULL DEFAULT 0.0000,

                                              unit_cost NUMERIC(12, 4) NOT NULL DEFAULT 0.0000,

                                              updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

                                              CONSTRAINT uq_item_per_location UNIQUE (location_id, item_id)
);

CREATE INDEX idx_wh_inv_item_search ON warehouse_inventory_balances(organization_id, item_id);

CREATE TYPE purchase_order_status AS ENUM ('draft', 'approved', 'sent', 'partially_received', 'received', 'cancelled');

CREATE TABLE purchase_orders (
                                 id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                                 organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
                                 vendor_id UUID NOT NULL REFERENCES partners(id) ON DELETE RESTRICT,
                                 destination_warehouse_id UUID NOT NULL REFERENCES warehouses(id) ON DELETE RESTRICT,

                                 po_number VARCHAR(50) NOT NULL,
                                 status purchase_order_status NOT NULL DEFAULT 'draft',
                                 order_date DATE NOT NULL,
                                 expected_delivery_date DATE,

                                 notes TEXT,
                                 created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

                                 CONSTRAINT uq_po_number_per_org UNIQUE (organization_id, po_number)
);

CREATE TABLE purchase_order_lines (
                                      id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                                      organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
                                      purchase_order_id UUID NOT NULL REFERENCES purchase_orders(id) ON DELETE CASCADE,
                                      item_id UUID NOT NULL REFERENCES items(id) ON DELETE RESTRICT,

                                      description TEXT,
                                      quantity_ordered NUMERIC(15,4) NOT NULL,
                                      quantity_received NUMERIC(15,4) NOT NULL DEFAULT 0.0000,
                                      unit_cost NUMERIC(15,4) NOT NULL,

                                      CONSTRAINT check_po_qty_positive CHECK (quantity_ordered > 0)
);

CREATE INDEX idx_po_lines_parent ON purchase_order_lines(purchase_order_id);

CREATE TABLE inventory_receipt_logs (
                                        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                                        organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
                                        purchase_order_line_id UUID NOT NULL REFERENCES purchase_order_lines(id) ON DELETE RESTRICT,
                                        received_at_location_id UUID NOT NULL REFERENCES warehouse_locations(id) ON DELETE RESTRICT,

                                        quantity_received NUMERIC(15,4) NOT NULL,
                                        received_date DATE NOT NULL DEFAULT CURRENT_DATE,
                                        received_by_user_id UUID,

                                        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_receipt_logs_po_line ON inventory_receipt_logs(purchase_order_line_id);


CREATE TABLE stock_transactions (
                                    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                                    organization_id UUID NOT NULL,
                                    warehouse_id UUID NOT NULL,
                                    location_id UUID NOT NULL REFERENCES warehouse_locations(id),
                                    item_id UUID NOT NULL REFERENCES items(id),
                                    transaction_type stock_transaction_type NOT NULL,
                                    quantity_change NUMERIC(12, 4) NOT NULL,
                                    reference_type reference_type,
                                    reference_id UUID,          -- Links to po_id or sales_order_id

                                    unit_cost NUMERIC(12, 4) NOT NULL DEFAULT 0.0000,
                                    journal_entry_id UUID REFERENCES journal_entries(id),

                                    notes TEXT,
                                    created_by UUID NOT NULL,
                                    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE sales_orders
(
    id              UUID PRIMARY KEY         DEFAULT gen_random_uuid(),
    organization_id UUID            NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    partner_id      UUID            NOT NULL REFERENCES partners(id) ON DELETE RESTRICT,
    warehouse_id    UUID            NOT NULL REFERENCES warehouses(id) ON DELETE RESTRICT,

    order_number    TEXT            NOT NULL,
    order_date      DATE            NOT NULL DEFAULT CURRENT_DATE,
    due_date        DATE            NOT NULL DEFAULT CURRENT_DATE,

    -- Statuses
    fulfillment_status fulfillment_status NOT NULL DEFAULT 'unfulfilled',
    payment_status payment_status NOT NULL DEFAULT 'unpaid',
    document_status sales_document_status NOT NULL DEFAULT 'draft',

    -- Addresses: selected saved IDs (optional) + immutable snapshots stored on the order
    billing_address_id  UUID REFERENCES partner_addresses(id) ON DELETE SET NULL,
    shipping_address_id UUID REFERENCES partner_addresses(id) ON DELETE SET NULL,

    -- Bill To snapshot
    bill_to_name        TEXT,
    bill_to_attention   TEXT,
    bill_to_line1       TEXT,
    bill_to_line2       TEXT,
    bill_to_city        TEXT,
    bill_to_region      TEXT,
    bill_to_postal_code TEXT,
    bill_to_country     TEXT,

    -- Ship To snapshot
    ship_to_name        TEXT,
    ship_to_attention   TEXT,
    ship_to_line1       TEXT,
    ship_to_line2       TEXT,
    ship_to_city        TEXT,
    ship_to_region      TEXT,
    ship_to_postal_code TEXT,
    ship_to_country     TEXT,

    -- Financial summary (denormalized for fast reading)
    subtotal        NUMERIC(15,4)   NOT NULL DEFAULT 0,
    tax_total       NUMERIC(15,4)   NOT NULL DEFAULT 0,
    total_amount    NUMERIC(15,4)   NOT NULL DEFAULT 0,
    amount_remaining    NUMERIC(15,4)   NOT NULL DEFAULT 0,

    created_at      TIMESTAMPTZ     NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_sales_orders_org ON sales_orders(organization_id);
CREATE UNIQUE INDEX idx_sales_orders_org_number ON sales_orders(organization_id, order_number);

CREATE TABLE sales_order_items
(
    id              UUID PRIMARY KEY         DEFAULT gen_random_uuid(),
    order_id        UUID            NOT NULL REFERENCES sales_orders(id) ON DELETE CASCADE,
    item_id         UUID            NOT NULL REFERENCES items(id) ON DELETE RESTRICT,

    code            TEXT            NOT NULL,
    name            TEXT            NOT NULL,
    description     TEXT,
    quantity        NUMERIC(15,4)   NOT NULL DEFAULT 0,
    unit_price      NUMERIC(15,4)   NOT NULL DEFAULT 0,
    tax_category_id UUID            REFERENCES tax_categories(id) ON DELETE SET NULL,
    tax_rate        NUMERIC(15,4)   NOT NULL DEFAULT 0,
    tax_amount      NUMERIC(15,4)   NOT NULL DEFAULT 0,
    net_amount      NUMERIC(15,4)   NOT NULL DEFAULT 0,
    sort_order      INT             NOT NULL DEFAULT 0
);

CREATE INDEX idx_sales_order_items_org ON sales_order_items(order_id);

-- =============================================================================
-- 1. Customer Payments
-- =============================================================================
CREATE TABLE customer_payments
(
    id                 UUID PRIMARY KEY         DEFAULT gen_random_uuid(),
    organization_id    UUID            NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    partner_id        UUID            NOT NULL REFERENCES partners (id) ON DELETE RESTRICT,

    -- The transaction clearing the asset (Debit Bank, Credit AR)
    transaction_id     UUID                     REFERENCES transactions (id) ON DELETE SET NULL,

    payment_date       DATE            NOT NULL,
    deposited_to_account UUID          NOT NULL, -- Ledger Account (e.g., Operating Bank Account)
    amount             NUMERIC(15,4)   NOT NULL, -- Total payment amount received
    reference          TEXT,                     -- Check number, Stripe transfer ID, or EFT reference

    created_at         TIMESTAMPTZ     NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_customer_payments_org ON customer_payments (organization_id);
CREATE INDEX idx_customer_payments_customer ON customer_payments (partner_id);

-- =============================================================================
-- 2. Customer Payment Allocations
-- =============================================================================
CREATE TABLE customer_payment_allocations
(
    id                 UUID PRIMARY KEY         DEFAULT gen_random_uuid(),
    organization_id    UUID            NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    sales_order_id   UUID            NOT NULL REFERENCES sales_orders (id) ON DELETE CASCADE,
    customer_payment_id UUID           NOT NULL REFERENCES customer_payments (id) ON DELETE CASCADE,

    allocated_amount   NUMERIC(15,4)   NOT NULL, -- Amount applied to this specific invoice
    created_at         TIMESTAMPTZ     NOT NULL DEFAULT NOW(),

    CONSTRAINT check_customer_alloc_positive CHECK (allocated_amount > 0)
);

CREATE INDEX idx_cust_allocations_invoice ON customer_payment_allocations (sales_order_id);
CREATE INDEX idx_cust_allocations_payment ON customer_payment_allocations (customer_payment_id);