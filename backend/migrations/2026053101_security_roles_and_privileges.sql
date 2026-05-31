/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

-- Up Migration

-- 1. Create System-Wide Global Privileges
CREATE TABLE privileges (
                            id VARCHAR(50) PRIMARY KEY, -- e.g., 'Use-Accounts', 'Manage-Partners'
                            description TEXT NOT NULL
);

-- Seed Initial Static Privileges Array
INSERT INTO privileges (id, description) VALUES
                                             ('Use-Accounts', 'Can view the Chart of Accounts, ledger summaries, and balances.'),
                                             ('Manage-Accounts', 'Can create, edit, restructure, or disable accounts.'),
                                             ('Use-Partners', 'Can view vendor/customer profiles and associated history ledger summaries.'),
                                             ('Manage-Partners', 'Can add, modify, or deactivate partners.'),
                                             ('Use-Transactions', 'Can input journal entries and draft transactions.'),
                                             ('Manage-Transactions', 'Can post, modify, or reverse entries (subject to Strict Audit Mode).'),
                                             ('Manage-Users', 'Can invite new teammates, alter roles, and deactivate users.'),
                                             ('Manage-Organization', 'Can change settings like Strict Audit Mode, open/close periods, and years.');

-- 2. Create Dynamic Tenant-Specific Roles Table
CREATE TABLE roles (
                       id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                       organization_id UUID NOT NULL, -- Isolated multi-tenant target
                       name VARCHAR(100) NOT NULL,    -- e.g., 'Senior Accountant'
                       created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    -- Prevent an organization from making duplicate role names
                       CONSTRAINT uk_org_role_name UNIQUE (organization_id, name)
);

-- 3. Create the Many-to-Many Privilege-to-Role Matrix Join Table
CREATE TABLE role_privileges (
                                 role_id UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
                                 privilege_id VARCHAR(50) NOT NULL REFERENCES privileges(id) ON DELETE RESTRICT,
                                 PRIMARY KEY (role_id, privilege_id)
);

-- 4. Safely Update Users table
ALTER TABLE users ADD COLUMN role_id UUID REFERENCES roles(id) ON DELETE SET NULL;