# Onboarding Workflow Plan: KelpieBooks

This document outlines the initial user flow for setting up a new instance of the accounting system. The goal is to move from a fresh database to a functional Organization with a populated Chart of Accounts (COA).

## 1. Phase 1: Identity Creation (The "Owner" User)
Since the system is multi-user and multi-tenant, every action must be scoped to a User and an Organization.

**Steps:**
- **Prompt:** Email and Password.
- **Backend Action:** Hash the password (using `argon2` or `bcrypt`).
- **Data Model:** Create a record in the `users` table. 

## 2. Phase 2: Organization Setup
**Steps:**
- **Prompt:** Legal Organization Name (e.g., "Acme Services Ltd").
- **Backend Action:** 1. Create the `organizations` record.
    2. Update the User created in Phase 1 with the new `organization_id`.

## 3. Phase 3: Chart of Accounts (COA) Initialization
An accounting system is useless without a structure to post to.

**Steps:**
- **Prompt:** Select a template (e.g., "Service Business", "Retail", "Empty").
- **Backend Action:** 1. Load the corresponding template from the `shared_core` crate.
    2. Recursively insert the accounts into the `accounts` table using the new `organization_id`.
    3. Ensure "System Accounts" (Retained Earnings, etc.) are tagged correctly.

---

## Technical Implementation Checklist

### Backend (Rocket + SQLX)
- [ ] Create a `/api/onboard` POST endpoint.
- [ ] Implement a DTO (Data Transfer Object) struct: `OnboardingRequest`.
- [ ] Write a service function that uses a **SQL Transaction** (`pool.begin()`) to ensure atomicity.

### Frontend (Yew + WASM)
- [ ] **View 1:** User registration form.
- [ ] **View 2:** Organization details form.
- [ ] **View 3:** Template selection cards.

### Logic Rules
- Account codes must be unique within the organization.
- Ensure the user is linked to the organization before finishing.
