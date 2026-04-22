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

-- KelpieBooks Initial Database Schema
-- This file is a consolidation of all incremental migration scripts
-- for easier setup and development.
--
-- Version: 2026042201
--

-- =============================================================================
-- From: 2026042201_initial_setup.sql
-- =============================================================================

-- 1. Organizations (Multi-tenancy)
CREATE TABLE organizations
(
    id         UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    name       TEXT        NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 2. Users
CREATE TABLE users
(
    id              UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    organization_id UUID        NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    email           TEXT        NOT NULL UNIQUE,
    password_hash   TEXT        NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);


-- =============================================================================
-- From: 2026042202_chart_of_accounts.sql
-- and  2026042204_add_system_tags.sql
-- =============================================================================

CREATE TYPE account_category AS ENUM ('Asset', 'Liability', 'Equity', 'Revenue', 'Expense');

CREATE TYPE system_tag AS ENUM (
    'CASH_AT_BANK',
    'ACCOUNTS_RECEIVABLE',
    'ACCOUNTS_PAYABLE',
    'RETAINED_EARNINGS',
    'SALES_TAX_PAYABLE',
    'REVENUE',
    'EXPENSE',
    'COST_OF_GOODS_SOLD'
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
-- From: 2026042203_journal_entries.sql
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

    -- Amount in cents.
    debit          BIGINT NOT NULL  DEFAULT 0,
    credit         BIGINT NOT NULL  DEFAULT 0,

    description    TEXT, -- Line-specific memo

    CONSTRAINT check_amount_positive CHECK (debit >= 0 AND credit >= 0),
    CONSTRAINT check_not_both_zero CHECK (debit > 0 OR credit > 0)
);

CREATE INDEX idx_je_transaction ON journal_entries (transaction_id);
CREATE INDEX idx_je_account ON journal_entries (account_id);
