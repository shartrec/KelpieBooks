/*
 * Copyright (c) 2026. Trevor Campbell and others.
 *
 * This file is part of KelpieBooks.
 *
 * KelpieBooks is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License,or
 * (at your option) any later version.
 *
 * KelpieBooks is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with KelpieBooks; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 *
 * Contributors:
 *      Trevor Campbell
 *
 */

-- =============================================================================
-- 1. Partner Addresses Table (Supporting Multiple Locations)
-- =============================================================================
CREATE TYPE address_type AS ENUM ('Billing', 'Shipping', 'General');

CREATE TABLE partner_addresses
(
    id              UUID PRIMARY KEY         DEFAULT gen_random_uuid(),
    organization_id UUID            NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    partner_id      UUID            NOT NULL REFERENCES partners (id) ON DELETE CASCADE,

    address_type    address_type    NOT NULL DEFAULT 'General',
    is_primary      BOOLEAN         NOT NULL DEFAULT FALSE,

    address_line1   TEXT            NOT NULL,
    address_line2   TEXT,
    city            TEXT            NOT NULL,
    state_province  TEXT,
    postal_code     TEXT,
    country         TEXT            NOT NULL,

    created_at      TIMESTAMPTZ     NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ     NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_partner_addresses_partner ON partner_addresses (partner_id);

-- Partial unique index: Ensures only ONE primary address exists per partner per type
CREATE UNIQUE INDEX idx_partner_single_primary_address
    ON partner_addresses (partner_id, address_type)
    WHERE is_primary = TRUE;


-- =============================================================================
-- 2. Partner Contacts Table (Individuals within the Organization)
-- =============================================================================
CREATE TABLE partner_contacts
(
    id              UUID PRIMARY KEY         DEFAULT gen_random_uuid(),
    organization_id UUID            NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    partner_id      UUID            NOT NULL REFERENCES partners (id) ON DELETE CASCADE,

    is_primary      BOOLEAN         NOT NULL DEFAULT FALSE,
    first_name      TEXT            NOT NULL,
    last_name       TEXT            NOT NULL,
    email           TEXT,
    phone           TEXT,
    role_title      TEXT, -- e.g., "Accounts Payable Clerk", "Sales Director"

    created_at      TIMESTAMPTZ     NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ     NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_partner_contacts_partner ON partner_contacts (partner_id);

-- Partial unique index: Ensures only ONE primary contact exists per partner
CREATE UNIQUE INDEX idx_partner_single_primary_contact
    ON partner_contacts (partner_id)
    WHERE is_primary = TRUE;