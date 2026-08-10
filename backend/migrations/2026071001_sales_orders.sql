/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

-- =============================================================================
-- Sales Orders
-- =============================================================================

-- Sales order status enum (snake_case to match sqlx rename_all = "snake_case")
CREATE TYPE sales_order_status AS ENUM ('open', 'confirmed', 'cancelled');

-- The 'sales_order' document type is a plain VARCHAR key in organization_sequences;
-- no seq_type enum exists. The sequence row is auto-created on first use by
-- get_next_invoice_number. No ALTER TYPE is required.

CREATE TABLE sales_orders
(
    id              UUID PRIMARY KEY         DEFAULT gen_random_uuid(),
    organization_id UUID            NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    partner_id      UUID            NOT NULL REFERENCES partners(id) ON DELETE RESTRICT,
    warehouse_id    UUID            NOT NULL REFERENCES warehouses(id) ON DELETE RESTRICT,

    order_number    TEXT            NOT NULL,
    order_date      DATE            NOT NULL DEFAULT CURRENT_DATE,
    status          sales_order_status NOT NULL DEFAULT 'open',

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
