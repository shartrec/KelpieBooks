/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
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
    full_name      TEXT            NOT NULL,
    preferred_name       TEXT            NOT NULL,
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