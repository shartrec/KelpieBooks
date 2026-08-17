/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

-- Seed Initial Static Privileges Array
CREATE TYPE system_privilege AS ENUM (
    'security_admin',
    'use_accounts',
    'manage_accounts',
    'use_partners',
    'manage_partners',
    'use_vendor_invoices',
    'manage_vendor_invoices',
    'use_transactions',
    'manage_transactions',
    'use_sales',
    'manage_sales',
    'manage_users',
    'manage_organization'
    );

-- 1. Organizations (Multi-tenancy)
CREATE TABLE organizations
(
    id                UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    name              TEXT        NOT NULL,
    strict_audit_mode BOOLEAN     NOT NULL DEFAULT TRUE,
    locked_until      DATE,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE organization_contacts (
                                       id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
                                       organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Descriptive label (e.g., 'Headquarters', 'Billing Dept', 'Sydney Warehouse')
                                       label VARCHAR(100) NOT NULL,
                                       is_primary BOOLEAN NOT NULL DEFAULT false,

    -- Unstructured Address Stack
                                       address_line1 TEXT NOT NULL,
                                       address_line2 TEXT DEFAULT '',
                                       address_line3 TEXT DEFAULT '',
                                       address_line4 TEXT DEFAULT '',

    -- Core Communication Channels
                                       phone VARCHAR(50) DEFAULT '',
                                       email VARCHAR(255) DEFAULT '',

                                       created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                                       updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Indexing for fast multi-tenant lookups on both listing and invoice tasks
CREATE INDEX idx_org_contacts_org ON organization_contacts(organization_id);

-- Enforce that only one contact block per organization can be marked 'is_primary' at a time
CREATE UNIQUE INDEX idx_org_contacts_primary_one
    ON organization_contacts(organization_id)
    WHERE (is_primary = true);

-- 2. Create Dynamic Tenant-Specific Roles Table
CREATE TABLE roles (
                       id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                       organization_id UUID NOT NULL, -- Isolated multi-tenant target
                       name VARCHAR(100) NOT NULL,    -- e.g., 'Senior Accountant'
                       created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- Prevent an organization from making duplicate role names
                       CONSTRAINT uk_org_role_name UNIQUE (organization_id, name)
);

-- 3. Create the Many-to-Many Privilege-to-Role Matrix Join Table
CREATE TABLE role_privileges
(
    role_id      UUID             NOT NULL REFERENCES roles (id) ON DELETE CASCADE,
    privilege_id system_privilege NOT NULL,

    PRIMARY KEY (role_id, privilege_id)
);

-- 4. Users
CREATE TABLE users
(
    id              UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    organization_id UUID        NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    email           TEXT        NOT NULL UNIQUE,
    password_hash   TEXT        NOT NULL,
    full_name       TEXT        NOT NULL,
    display_name    TEXT,
    role_id         UUID        REFERENCES roles(id) ON DELETE SET NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);


CREATE TABLE password_reset_tokens (
                                       id SERIAL PRIMARY KEY,
                                       user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                                       token_hash VARCHAR(64) NOT NULL UNIQUE, -- SHA-256 hash of the token text
                                       expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
                                       created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                                       used BOOLEAN NOT NULL DEFAULT FALSE
);

-- =============================================================================
-- Chart of Accounts
-- =============================================================================

CREATE TYPE account_category AS ENUM ('asset', 'liability', 'equity', 'revenue', 'expense');

CREATE TYPE system_tag AS ENUM (
    'cash_at_bank',
    'accounts_receivable',
    'accounts_payable',
    'retained_earnings',
    'sales_tax_payable',
    'sales_tax_clearing',
    'revenue',
    'expense',
    'cost_of_goods_sold'
    );

CREATE TABLE accounts
(
    id              UUID PRIMARY KEY          DEFAULT gen_random_uuid(),
    organization_id UUID             NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    parent_id       UUID             REFERENCES accounts (id) ON DELETE SET NULL,

    -- User-defined fields
    code            TEXT             NOT NULL,
    name            TEXT             NOT NULL,
    category        account_category NOT NULL,

    -- Logic flags
    is_group        BOOLEAN          NOT NULL DEFAULT FALSE, -- If true, no postings allowed
    is_bank_account BOOLEAN          NOT NULL DEFAULT FALSE,
    system_tag      system_tag,                             -- If not null, this is a system account

    created_at      TIMESTAMPTZ      NOT NULL DEFAULT NOW(),

    -- Ensure codes are unique within one organization
    UNIQUE (organization_id, code),

    -- Prevent an account from being its own parent
    CONSTRAINT check_not_self_parent CHECK (id <> parent_id)
);

-- Indexing for performance on tree roll-ups and organization filtering
CREATE INDEX idx_accounts_org ON accounts (organization_id);
CREATE INDEX idx_accounts_parent ON accounts (parent_id);

-- =============================================================================
-- Transactions and Journal Entries
-- =============================================================================

-- The Header: Represents the "event"
CREATE TABLE transactions
(
    id              UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    organization_id UUID        NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    date            DATE        NOT NULL,
    description     TEXT,
    reference       TEXT, -- e.g., Check # or Invoice #
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- The Lines: The actual Debits and Credits
CREATE TABLE journal_entries
(
    id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    transaction_id UUID   NOT NULL REFERENCES transactions (id) ON DELETE CASCADE,
    account_id     UUID   NOT NULL REFERENCES accounts (id),

    debit          NUMERIC(15,4) NOT NULL  DEFAULT 0,
    credit         NUMERIC(15,4) NOT NULL  DEFAULT 0,

    description    TEXT, -- Line-specific memo
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT check_amount_positive CHECK (debit >= 0 AND credit >= 0),
    CONSTRAINT check_not_both_zero CHECK (debit > 0 OR credit > 0)
);

CREATE INDEX idx_je_transaction ON journal_entries (transaction_id);
CREATE INDEX idx_je_account ON journal_entries (account_id);

-- =============================================================================
-- Unified Contacts System (Partners)
-- =============================================================================
-- =============================================================================
-- Partner Addresses Table (Supporting Multiple Locations)
-- =============================================================================
CREATE TYPE address_type AS ENUM ('billing', 'shipping', 'general');

CREATE TABLE partners
(
    id                    UUID PRIMARY KEY         DEFAULT gen_random_uuid(),
    organization_id       UUID            NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    legal_name            TEXT            NOT NULL,
    trade_name            TEXT,
    tax_identifier        TEXT, -- e.g., EIN, ABN, SSN, VAT number

    -- Role Triggers
    is_vendor             BOOLEAN         NOT NULL DEFAULT FALSE,
    is_customer           BOOLEAN         NOT NULL DEFAULT FALSE,

    -- Default Ledger Routing Accounts (Automation defaults)
    default_ap_account_id UUID                     REFERENCES accounts (id) ON DELETE SET NULL,
    default_ar_account_id UUID                     REFERENCES accounts (id) ON DELETE SET NULL,

    created_at            TIMESTAMPTZ     NOT NULL DEFAULT NOW(),
    updated_at            TIMESTAMPTZ     NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_partners_org ON partners (organization_id);
-- Ensure tax identifiers are unique per organization, ignoring null entries
CREATE UNIQUE INDEX idx_partners_org_tax_id ON partners (organization_id, tax_identifier) WHERE tax_identifier IS NOT NULL;

CREATE TABLE partner_addresses
(
    id              UUID PRIMARY KEY         DEFAULT gen_random_uuid(),
    organization_id UUID            NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    partner_id      UUID            NOT NULL REFERENCES partners (id) ON DELETE CASCADE,

    address_type    address_type    NOT NULL DEFAULT 'general',
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
-- Partner Contacts Table (Individuals within the Organization)
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


-- =============================================================================
-- Accounts Payable: Vendor Invoices (Bills)
-- =============================================================================
CREATE TYPE invoice_status AS ENUM ('draft', 'open', 'paid', 'partially_paid', 'void');

CREATE TABLE vendor_invoices
(
    id              UUID PRIMARY KEY         DEFAULT gen_random_uuid(),
    organization_id UUID            NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    partner_id      UUID            NOT NULL REFERENCES partners (id) ON DELETE RESTRICT,

    -- The financial event tie-in: points to the header transaction created by this bill
    transaction_id  UUID                     REFERENCES transactions (id) ON DELETE SET NULL,

    invoice_number  TEXT            NOT NULL, -- The vendor's invoice number
    status          invoice_status  NOT NULL DEFAULT 'open',

    issue_date      DATE            NOT NULL,
    due_date        DATE            NOT NULL,

    net_amount       NUMERIC(15,4)         NOT NULL, -- Net amount of the invoice
    tax_amount       NUMERIC(15,4)         NOT NULL, -- Tax amount of the invoice
    gross_amount     NUMERIC(15,4)         NOT NULL, -- Gross amount of the invoice
    amount_remaining NUMERIC(15,4)         NOT NULL, -- Amount left to pay (for partial tracking)

    notes           TEXT,
    created_at      TIMESTAMPTZ     NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ     NOT NULL DEFAULT NOW(),

    CONSTRAINT check_positive_amounts CHECK (gross_amount >= 0 AND amount_remaining >= 0),
    CONSTRAINT check_remaining_le_due CHECK (amount_remaining <= gross_amount),
    CONSTRAINT check_due_date_valid CHECK (due_date >= issue_date)
);

CREATE INDEX idx_vendor_invoices_org ON vendor_invoices (organization_id);
CREATE INDEX idx_vendor_invoices_partner ON vendor_invoices (partner_id);
-- Prevent duplicate processing of the same invoice number from a single vendor
CREATE UNIQUE INDEX idx_vendor_invoice_uniq_per_vendor ON vendor_invoices (organization_id, partner_id, invoice_number);

CREATE TABLE vendor_invoice_items
(
    id                UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    vendor_invoice_id UUID        NOT NULL REFERENCES vendor_invoices (id) ON DELETE CASCADE,

    -- The critical GL link: reference to your Chart of Accounts
    account_id        UUID        NOT NULL REFERENCES accounts (id) ON DELETE RESTRICT,

    description       TEXT                 DEFAULT '',
    net_amount        NUMERIC(15,4)      NOT NULL DEFAULT 0, -- Line amount before tax
    tax_amount        NUMERIC(15,4)      NOT NULL DEFAULT 0, -- Tax applied to this specific line
    total_amount      NUMERIC(15,4)      NOT NULL DEFAULT 0, -- Net + Tax

    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_invoice_items_invoice ON vendor_invoice_items (vendor_invoice_id);

-- =============================================================================
-- 4. Accounts Payable: Vendor Payments
-- =============================================================================
CREATE TABLE vendor_payments
(
    id              UUID PRIMARY KEY         DEFAULT gen_random_uuid(),
    organization_id UUID            NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    partner_id      UUID            NOT NULL REFERENCES partners (id) ON DELETE RESTRICT,

    -- The transaction clearing the liability (Credit Bank, Debit AP)
    transaction_id  UUID                     REFERENCES transactions (id) ON DELETE SET NULL,

    payment_date       DATE             NOT NULL,
    paid_from_account  UUID             NOT NULL, -- Ledger Account paid from
    amount             NUMERIC(15,4)    NOT NULL, -- Total payment
    reference          TEXT,                     -- Check number or bank trace number

    created_at      TIMESTAMPTZ     NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_vendor_payments_org ON vendor_payments (organization_id);
CREATE INDEX idx_vendor_payments_partner ON vendor_payments (partner_id);


-- =============================================================================
-- Invoice/Payment Allocation Join Table
-- =============================================================================
-- Needed because a single payment check can cover multiple invoices (or a single invoice can have partial payments)
CREATE TABLE vendor_payment_allocations
(
    id                UUID PRIMARY KEY         DEFAULT gen_random_uuid(),
    organization_id   UUID            NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    vendor_invoice_id UUID            NOT NULL REFERENCES vendor_invoices (id) ON DELETE CASCADE,
    vendor_payment_id UUID            NOT NULL REFERENCES vendor_payments (id) ON DELETE CASCADE,

    allocated_amount  NUMERIC(15,4)          NOT NULL, -- How much of this payment went to this invoice
    created_at        TIMESTAMPTZ     NOT NULL DEFAULT NOW(),

    CONSTRAINT check_positive_allocation CHECK (allocated_amount > 0)
);

CREATE INDEX idx_allocations_invoice ON vendor_payment_allocations (vendor_invoice_id);
CREATE INDEX idx_allocations_payment ON vendor_payment_allocations (vendor_payment_id);



-- =============================================================================
-- Sales and invoicing
-- =============================================================================

CREATE TYPE item_type AS ENUM ('stocked', 'non_stocked', 'service');

CREATE TABLE tax_categories (
                                id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
                                organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
                                name VARCHAR(50) NOT NULL UNIQUE,       -- e.g., "Standard Rate", "Zero Rated", "Exempt"
                                description VARCHAR(255),
                                is_active BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE tax_rates (
                           id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
                           organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
                           tax_category_id UUID NOT NULL REFERENCES tax_categories(id) ON DELETE CASCADE,
                           name VARCHAR(50) NOT NULL,              -- e.g., "VAT 20%", "State Tax 6%"
                           rate NUMERIC(6, 4) NOT NULL,            -- e.g., 0.2000 for 20%, 0.0625 for 6.25%

    -- Ledger Mapping
                           liability_account_id UUID NOT NULL,      -- Chart of Accounts link (e.g., "Sales Tax Payable")

    -- Date Range Validations
                           valid_from DATE NOT NULL DEFAULT CURRENT_DATE,
                           valid_to DATE,                          -- Nullable means it's currently active indefinitely

                           created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE units_of_measure (
                                  id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
                                  organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
                                  code VARCHAR(10) NOT NULL UNIQUE,       -- e.g., "EA", "HR", "M", "KG"
                                  name VARCHAR(50) NOT NULL,              -- e.g., "Each", "Hour", "Meter", "Kilogram"
                                  is_active BOOLEAN NOT NULL DEFAULT TRUE
);

-- Indexing for fast historical date lookups
CREATE INDEX idx_tax_rates_lookup ON tax_rates(tax_category_id, valid_from, valid_to);

CREATE TABLE items (
                       id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
                       organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
                       code VARCHAR(50) NOT NULL UNIQUE,       -- SKU or Item ID (e.g., "CONS-01", "WIDGET-X")
                       name VARCHAR(150) NOT NULL,
                       description TEXT,
                       item_type item_type NOT NULL DEFAULT 'non_stocked',
                       uom_id UUID NOT NULL REFERENCES units_of_measure(id),
                       tax_category_id UUID REFERENCES tax_categories(id) ON DELETE SET NULL,
    -- Financial Mapping
                       unit_price NUMERIC(15,4) NOT NULL DEFAULT 0, -- Base sales price
                       income_account_id UUID NOT NULL,         -- Links to Chart of Accounts (e.g., "Revenue")

                       is_active BOOLEAN NOT NULL DEFAULT TRUE,
                       created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE organization_sequences (
                                        org_id UUID NOT NULL,
                                        document_type VARCHAR(50) NOT NULL, -- e.g., 'sales_invoice'
                                        prefix VARCHAR(20) DEFAULT '',      -- e.g., 'INV-'
                                        next_value INT NOT NULL DEFAULT 1000,
                                        PRIMARY KEY (org_id, document_type)
);