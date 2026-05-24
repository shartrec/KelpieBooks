-- =============================================================================
-- 1. Unified Contacts System (Partners)
-- =============================================================================
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


-- =============================================================================
-- 2. Link Existing General Ledger Lines to Partners
-- =============================================================================
-- This lets you run a General Ledger detail report filterable by Vendor/Customer
ALTER TABLE journal_entries ADD COLUMN partner_id UUID REFERENCES partners (id) ON DELETE SET NULL;
CREATE INDEX idx_je_partner ON journal_entries (partner_id);


-- =============================================================================
-- 3. Accounts Payable: Vendor Invoices (Bills)
-- =============================================================================
CREATE TYPE invoice_status AS ENUM ('Open', 'Paid', 'PartiallyPaid', 'Void');

CREATE TABLE vendor_invoices
(
    id              UUID PRIMARY KEY         DEFAULT gen_random_uuid(),
    organization_id UUID            NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    partner_id      UUID            NOT NULL REFERENCES partners (id) ON DELETE RESTRICT,

    -- The financial event tie-in: points to the header transaction created by this bill
    transaction_id  UUID                     REFERENCES transactions (id) ON DELETE SET NULL,

    invoice_number  TEXT            NOT NULL, -- The vendor's invoice number
    status          invoice_status  NOT NULL DEFAULT 'Open',

    issue_date      DATE            NOT NULL,
    due_date        DATE            NOT NULL,

    -- Tracking monetary sums in cents (BIGINT to avoid float truncation bugs)
    net_amount       BIGINT         NOT NULL, -- Net amount of the invoice
    tax_amount       BIGINT         NOT NULL, -- Tax amount of the invoice
    gross_amount     BIGINT         NOT NULL, -- Gross amount of the invoice
    amount_remaining BIGINT         NOT NULL, -- Amount left to pay (for partial tracking)

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
    net_amount        BIGINT      NOT NULL DEFAULT 0, -- Line amount before tax
    tax_amount        BIGINT      NOT NULL DEFAULT 0, -- Tax applied to this specific line
    total_amount      BIGINT      NOT NULL DEFAULT 0, -- Net + Tax

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

    payment_date       DATE         NOT NULL,
    paid_from_account  UUID         NOT NULL, -- Ledger Account paid from
    amount             BIGINT       NOT NULL, -- Total payment size in cents
    reference          TEXT,                     -- Check number or bank trace number

    created_at      TIMESTAMPTZ     NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_vendor_payments_org ON vendor_payments (organization_id);
CREATE INDEX idx_vendor_payments_partner ON vendor_payments (partner_id);


-- =============================================================================
-- 5. Invoice/Payment Allocation Join Table
-- =============================================================================
-- Needed because a single payment check can cover multiple invoices (or a single invoice can have partial payments)
CREATE TABLE vendor_payment_allocations
(
    id                UUID PRIMARY KEY         DEFAULT gen_random_uuid(),
    organization_id   UUID            NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    vendor_invoice_id UUID            NOT NULL REFERENCES vendor_invoices (id) ON DELETE CASCADE,
    vendor_payment_id UUID            NOT NULL REFERENCES vendor_payments (id) ON DELETE CASCADE,

    allocated_amount  BIGINT          NOT NULL, -- How much of this payment went to this invoice
    created_at        TIMESTAMPTZ     NOT NULL DEFAULT NOW(),

    CONSTRAINT check_positive_allocation CHECK (allocated_amount > 0)
);

CREATE INDEX idx_allocations_invoice ON vendor_payment_allocations (vendor_invoice_id);
CREATE INDEX idx_allocations_payment ON vendor_payment_allocations (vendor_payment_id);