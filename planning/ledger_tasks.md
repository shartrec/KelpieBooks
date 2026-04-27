# General Ledger Implementation Plan

This document outlines the tasks required to build the core General Ledger functionality. The plan is broken down into phases, starting with read-only display and progressively adding more complex features.

## Phase 1: Display Chart of Accounts (Read-Only)

The goal of this phase is to display a hierarchical, read-only Chart of Accounts (CoA) with correctly calculated and rolled-up balances.

### Backend Tasks
1.  **Create `AccountWithBalance` DTO:** In `shared_core`, create a new DTO that includes all fields from the `Account` model plus a `balance: i64` field.
2.  **Database Balance Logic:**
    *   Create a DB function to calculate the balance for a *single* account by summing its journal entries.
    *   Create a service-level function that fetches all accounts and journal entries for an organization, then calculates the rolled-up balance for every account in the hierarchy. This is the most complex part and may require an iterative or recursive approach in Rust.
3.  **Create API Endpoint:**
    *   Create a new Rocket route: `GET /api/accounts`.
    *   This endpoint will use the service function from the previous step to get all accounts with their rolled-up balances.
    *   It should return the data as a `Json<Vec<AccountWithBalance>>`.

### Frontend Tasks
1.  **Create `LedgerPage`:** This will be the main page for all ledger-related activities, accessible from the sidebar.
2.  **Create `ChartOfAccountsTable` Component:**
    *   This component will fetch data from the `GET /api/accounts` endpoint when it mounts.
    *   It will be responsible for rendering the flat list of accounts as a hierarchical tree view (e.g., using indentation for child accounts).
    *   It will display columns for Account Code, Name, Category, and the calculated Balance.
    *   Implement loading and error states for the API call.

## Phase 2: Manage Chart of Accounts (CRUD)

This phase focuses on allowing users to add, edit, and delete accounts.

### Backend Tasks
1.  **Create `POST /api/accounts` Endpoint:** For adding a new account.
2.  **Create `PUT /api/accounts/{id}` Endpoint:** For editing an existing account's name, code, etc.
3.  **Create `DELETE /api/accounts/{id}` Endpoint:**
    *   **Crucial Validation:** This endpoint must check if an account has any associated journal entries. If it does, the deletion should be rejected with a clear error message (e.g., `409 Conflict`).

### Frontend Tasks
1.  **Create "Add Account" Form:**
    *   This could be a modal or a separate page.
    *   The form should include a dropdown to select the `parent_account`, populated from the list of existing group accounts.
2.  **Add Action Buttons:** Add "Edit" and "Delete" icon buttons to each row of the `ChartOfAccountsTable`.
3.  **Implement Confirmation Modal:** The "Delete" button should trigger a confirmation dialog to prevent accidental deletions.

## Phase 3: Account Ledger View (Drill-Down)

This phase allows users to see the transaction details that make up an account's balance.

### Backend Tasks
1.  **Create `GET /api/accounts/{id}/entries` Endpoint:**
    *   This endpoint will return a list of all journal entries for a specific account.
    *   It should support date range query parameters (e.g., `?start_date=...&end_date=...`).
    *   The response should include a "running balance" for each entry to make rendering the ledger easier for the frontend.

### Frontend Tasks
1.  **Make CoA Rows Clickable:** The account names in the `ChartOfAccountsTable` should be links.
2.  **Create `AccountLedgerPage`:**
    *   This page will take an account ID from the URL.
    *   It will fetch data from the `/api/accounts/{id}/entries` endpoint.
    *   It will display the entries in a classic ledger table: Date, Description, Debit, Credit, Running Balance.
    *   Add date filter controls (e.g., "This Month", "Last Quarter", custom range).

## Phase 4: General Journal Transaction Entry

This is the most complex UI/UX part, allowing users to create new, balanced journal transactions.

### Backend Tasks
1.  **Create `POST /api/transactions` Endpoint:**
    *   The endpoint will accept a JSON object containing a date, description, and a list of entry lines (account ID, debit amount, credit amount).
    *   **Crucial Validation:** This endpoint **must** use a database transaction. It must verify that the sum of all debits equals the sum of all credits before committing. If they don't balance, it should return a `400 Bad Request` error.
    *   It will first create the parent `Transaction` record, then loop through and create all the associated `JournalEntry` records.

### Frontend Tasks
1.  **Create `NewTransactionPage`:**
    *   This page will feature a form for the transaction date and description.
    *   It will have a dynamic, multi-line section for the journal entries.
    *   Each line will have a dropdown to select a postable account, a debit input, and a credit input.
    *   The UI should provide a running total of debits and credits and visually indicate whether the transaction is balanced.
    *   The "Save" button should be disabled until the transaction is balanced.

## Supplementary tasks
1. Look at refactoring AccountCategory and SystemTag Enums to work more seemlessly with psql.
2. 