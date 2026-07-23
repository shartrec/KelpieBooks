/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

ALTER TYPE system_privilege ADD VALUE 'use_inventory';
ALTER TYPE system_privilege ADD VALUE 'manage_inventory';

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

    -- Structured coordinate paths for path finding/picking logic
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
    -- Tracks items that are physically in the building but allocated to sales orders
                                              quantity_allocated NUMERIC(15,4) NOT NULL DEFAULT 0.0000,

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