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
CREATE TYPE system_privilege AS ENUM (
    'org_admin'
    'use_accounts',
    'manage_accounts',
    'use_partners',
    'manage_partners',
    'use_transactions',
    'manage_transactions',
    'manage_users',
    'manage_organization'
    );

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

-- 4. Safely Update Users table
ALTER TABLE users ADD COLUMN role_id UUID REFERENCES roles(id) ON DELETE SET NULL;