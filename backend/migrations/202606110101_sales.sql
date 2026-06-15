/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

ALTER TYPE system_privilege ADD VALUE 'use_sales' ;
ALTER TYPE system_privilege ADD VALUE 'manage_sales';

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

CREATE TABLE sales_invoices (
                                id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
                                organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
                                invoice_number VARCHAR(50) NOT NULL UNIQUE, -- User-facing sequence ID (e.g., "INV-2026-001")
                                customer_id UUID NOT NULL,                  -- Links to your Customers/Contacts table
                                status invoice_status NOT NULL DEFAULT 'open',

    -- Key Accounting Dates
                                issue_date DATE NOT NULL DEFAULT CURRENT_DATE,
                                due_date DATE NOT NULL,

    -- Financial Summary Fields (Denormalized slightly for fast reading)
                                subtotal NUMERIC(15,4) NOT NULL DEFAULT 0,
                                tax_total NUMERIC(15,4) NOT NULL DEFAULT 0,
                                total_amount NUMERIC(15,4) NOT NULL DEFAULT 0,
                                amount_due NUMERIC(15,4) NOT NULL DEFAULT 0, -- Track remaining A/R balance

                                created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Indexing for standard A/R Aging reports ("Show me who is past due")
CREATE INDEX idx_invoices_due_date_status ON sales_invoices(due_date, status);
CREATE INDEX idx_invoices_customer ON sales_invoices(customer_id);

CREATE TABLE sales_invoice_lines (
                                     id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
                                     organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
                                     invoice_id UUID NOT NULL REFERENCES sales_invoices(id) ON DELETE CASCADE,
                                     item_id UUID REFERENCES items(id) ON DELETE SET NULL, -- Nullable if allowing free-text custom lines

                                     description TEXT NOT NULL,         -- Copy from item, but editable by user per invoice
                                     quantity BIGINT NOT NULL DEFAULT 1.0000,
                                     unit_price NUMERIC(15,4) NOT NULL DEFAULT 0,
                                     tax_rate_id UUID REFERENCES tax_rates(id) ON DELETE SET NULL,
                                     tax_amount NUMERIC(15,4) NOT NULL DEFAULT 0,
    -- Subtotals calculated automatically per line
                                     line_total NUMERIC(15,4) NOT NULL DEFAULT 0,

    -- Track sorting sequence for rendering PDFs correctly
                                     sort_order INT NOT NULL DEFAULT 0
);

CREATE TYPE ar_transaction_type AS ENUM ('invoice_charge', 'payment_received', 'credit_memo');

CREATE TABLE accounts_receivable_ledger (
                                            id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
                                            organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
                                            customer_id UUID NOT NULL,
                                            invoice_id UUID NOT NULL REFERENCES sales_invoices(id) ON DELETE CASCADE,
                                            transaction_type ar_transaction_type NOT NULL,

    -- Accounts Receivable is an Asset, so:
    -- Invoices INCREASE the balance (Debit)
    -- Payments DECREASE the balance (Credit)
                                            amount NUMERIC(15,4) NOT NULL,

                                            entry_date DATE NOT NULL DEFAULT CURRENT_DATE,
                                            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_ar_ledger_customer ON accounts_receivable_ledger(customer_id);