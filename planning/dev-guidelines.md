# Developer Guidelines

This document summarises the current project layout, application structure, and styling conventions used in KelpieBooks. It is intended to help contributors make changes that fit naturally with the existing codebase.

## Project Overview

KelpieBooks is organised as a Rust workspace with separate backend, frontend, shared domain code, planning documents, and deployment/configuration assets.


## Top-Level Directories

### `backend/`

The backend contains the Rocket-based server application.

Key areas:

- `backend/src/main.rs`  
  Application entry point and server setup.

- `backend/src/db/`  
  Database access layer. Files are grouped by domain concept, such as accounts, users, organisations, transactions, journal entries, and security.

- `backend/src/routes/`  
  HTTP route handlers grouped by feature area.

- `backend/src/services/`  
  Business logic that sits between routes and database access.

- `backend/src/util/`  
  Shared backend utilities, such as logging and common types.

- `backend/migrations/`  
  SQL database migrations.

- `backend/static/`  
  Static files served by the backend if required.

### `frontend/`

The frontend is a Yew application compiled to WebAssembly and built with Trunk.

Key areas:

- `frontend/src/main.rs`  
  Frontend application entry point.

- `frontend/src/lib.rs`  
  Main frontend library module.

- `frontend/src/auth.rs`  
  Frontend authentication-related logic.

- `frontend/src/components/`  
  Reusable UI components such as layout, sidebar, header, account rows, journal entry rows, and modal dialogs.

- `frontend/src/pages/`  
  Route-level page components such as dashboard, login, register, profile, ledger, account ledger, and transaction entry pages.

- `frontend/assets/`  
  Static frontend assets, including images and CSS assets.

- `frontend/styles/kelpie.css`  
  Primary shared stylesheet for the frontend.

- `frontend/index.html`  
  HTML entry point used by Trunk.

- `frontend/Trunk.toml`  
  Trunk build configuration.

### `shared_core/`

Shared Rust code used across the backend and frontend.

Typical responsibilities include:

- Shared models
- DTOs
- Request types
- Utility functions
- Common business/domain types

Use this crate for types that must remain consistent between client and server.

### `planning/`

Project planning and design documentation.

Current documents include onboarding and ledger planning notes. New design, architecture, feature, and implementation planning documents should be added here.

### `templates/`

Configuration or deployment templates.

### `logs/`

Runtime logs. These should generally be treated as generated output, not source material.

## Architectural Conventions

### Keep Domain Types Shared

If a type is exchanged between frontend and backend, place it in `shared_core` rather than duplicating it.

Good candidates for `shared_core` include:

- Request payloads
- Response DTOs
- Domain models
- Enum values used by both frontend and backend
- Shared formatting or validation helpers

### Keep Backend Layers Separate

Backend changes should generally follow this flow:


Use each layer for its intended purpose:

- **Routes** should handle HTTP concerns:
    - Request parsing
    - Authentication/authorisation checks
    - Response construction
    - Status codes

- **Services** should handle business logic:
    - Validation
    - Multi-step operations
    - Coordination between database functions
    - Domain rules

- **DB modules** should handle database access:
    - SQL queries
    - Mapping database rows to Rust types
    - Insert/update/delete/select operations

Avoid placing complex business logic directly inside route handlers or database functions.

### Keep Frontend Components Focused

Frontend code is split into:

- **Pages** for route-level screens
- **Components** for reusable UI pieces
- **Auth logic** for authentication helpers
- **Shared styles** in `kelpie.css`

A page should compose components rather than becoming one large component itself.

Reusable UI patterns, such as modals, rows, layout wrappers, and navigation elements, should live in `frontend/src/components/`.

## Frontend Layout Guidelines

The application uses a dashboard-style layout with:

- A sidebar for primary navigation
- A header for top-level page/application controls
- A main content area for page content

The general structure is:


When adding authenticated/dashboard pages, they should fit inside the existing layout rather than creating independent page chrome.

Authentication pages may use centred form layouts instead of the dashboard layout where appropriate.

## Styling Guidelines

The frontend uses a central stylesheet:

Use these variables for consistent colour usage.

### Typography


Headings use the custom dark red brand colour and modest sizing.

General conventions:

- `h1` is used for primary page titles.
- `h2` is used for section headings and includes a subtle bottom border.
- `h3` is used for smaller section or panel headings.

### Layout Containers

Common layout classes include:

- `.page-container`  
  Used for full-height centred page layouts, especially auth-style screens.

- `.content`  
  Flexible vertical content container with scroll support.

- `.scrollable-table`  
  Scrollable area for table-heavy views.

- `.scrollable-list`  
  Fixed-height scrollable list region.

When building new screens, prefer these existing layout classes before introducing new layout rules.

### Buttons

Base `button` styles are defined globally.

Default buttons use:

- Light grey background
- Dark red text
- Rounded corners
- Subtle border
- Hover state
- Disabled state

Additional button patterns:

- `.button-secondary` for secondary actions
- `.button-group` for radio-style grouped choices
- `.icon-button` for compact icon actions
- `.key-button` for minimal key-style buttons
- `.button-row` for horizontally spaced button groups

Guidelines:

- Use normal buttons for primary page actions.
- Use `.button-secondary` for cancel/back/less important actions.
- Use `.icon-button` for row-level table actions.
- Keep destructive actions behind confirmation modals where appropriate.

### Forms

Inputs, selects, and textareas share consistent styling:

- Rounded border
- Compact padding
- Focus ring
- Max width control
- Validation classes

Validation classes:

- `.input-success`
- `.input-error`
- `.error`
- `.success-message`

Form layout patterns:

- `.auth-form` uses a two-column grid with labels on the left and inputs on the right.
- `.modal-form` uses a similar two-column grid inside modals.
- `.transaction-form` is used for larger accounting/transaction entry screens.

Guidelines:

- Align labels consistently.
- Use existing error and success message styles.
- Avoid inline styles for validation states.
- Prefer semantic form elements.

### Tables

Tables use the shared `.table` class.

Common behaviour:

- Full-width layout
- Collapsed borders
- White background
- Subtle shadow
- Compact cell padding
- Header background
- Alternating row colour

Special table conventions:

- `.coa-table` is used for chart of accounts tables.
- `.actions-cell` is used for action columns.
- `.amount` is used for right-aligned numeric values.
- `.transaction-detail-row` and `.transaction-detail-content` are used for expandable transaction details.

Guidelines:

- Use `.table` for tabular data.
- Right-align monetary and amount fields.
- Use action columns for edit/delete/view controls.
- Keep row-level action buttons compact.

### Modals

Modal styling uses:

- `.modal-overlay`
- `.modal-content`
- `.modal-form`

Modal conventions:

- Overlay covers the full viewport.
- Content is centred.
- Forms use grid alignment.
- Actions are placed at the bottom right.

Use modals for focused interactions such as:

- Adding records
- Editing records
- Confirming destructive actions
- Confirming journal entry reversals

### Cards and Alerts

Use `.card` for boxed content panels.

Use `.alert` for prominent warning/error notices.

Use `.success-message` for successful operation feedback.

Use `.error` for form or request errors.

### Links

Links use the shared link colour and underline on hover.

Avoid styling links as buttons unless they perform navigation. Use buttons for actions.

## Accounting UI Conventions

KelpieBooks contains accounting-focused screens, so consistency is especially important for financial data.

### Amounts

Use `.amount` for numeric money values.

Amounts should be:

- Right-aligned
- Consistently formatted
- Clearly separated into debit/credit/balance columns where relevant

### Journal Entries

Journal entry forms use grid-based rows with consistent columns.

Existing classes include:

- `.journal-entry-header`
- `.journal-entry-row`
- `.journal-entry-line`
- `.totals`
- `.balanced`
- `.unbalanced`

Guidelines:

- Keep debit and credit columns aligned.
- Clearly indicate whether an entry is balanced.
- Use success/error colour states for balanced/unbalanced indicators.
- Avoid hiding imbalance states from the user.

### Chart of Accounts

Chart of accounts tables use `.coa-table` with specific column classes:

- `.code-col`
- `.name-col`
- `.category-col`
- `.balance-col`
- `.actions-col`

Parent accounts are visually emphasised with `.parent-account`.

Use `.collapse-toggle` for expanding/collapsing account groups.

## Naming Guidelines

### Rust Modules

Use `snake_case` for Rust files and modules.

Examples:

### Components

Frontend component files should be named after the component or UI element they contain.

Examples:


Prefer descriptive class names over abbreviated names unless the abbreviation is already established, such as `coa-table`.

## Adding New Features

When adding a new feature:

1. Define shared request/response/domain types in `shared_core` if needed.
2. Add database logic under `backend/src/db/`.
3. Add business rules under `backend/src/services/`.
4. Add HTTP routes under `backend/src/routes/`.
5. Add frontend API calls/auth handling where needed.
6. Add reusable frontend components under `frontend/src/components/`.
7. Add route-level screens under `frontend/src/pages/`.
8. Reuse styles from `frontend/styles/kelpie.css`.
9. Add or update planning documentation in `planning/` for significant features.

## Style Maintenance Guidelines

When modifying styles:

- Prefer CSS variables for shared colours.
- Reuse existing utility/component classes.
- Keep table and form spacing compact and consistent.
- Avoid introducing page-specific styling unless necessary.
- Group related styles together.
- Use comments for major style sections.
- Avoid inline styles in Rust/Yew components unless there is a strong reason.

## Consistency Checklist

Before submitting frontend changes, check:

- Does the page fit into the established layout?
- Are buttons using existing button styles?
- Are forms using existing form patterns?
- Are errors and success messages styled consistently?
- Are tables using `.table`?
- Are amounts right-aligned?
- Are modals using the standard modal classes?
- Are colours coming from CSS variables where appropriate?

Before submitting backend changes, check:

- Are route handlers thin?
- Is business logic in services?
- Is SQL/database logic isolated in `db` modules?
- Are shared types placed in `shared_core`?
- Are errors handled clearly?
- Are new routes grouped in the correct route module?

## General Development Principles

- Keep code organised by responsibility.
- Prefer small, focused components and functions.
- Avoid duplicating shared types between frontend and backend.
- Keep user-facing interactions predictable and consistent.
- Preserve the accounting-specific clarity of the interface.
- Update this document when layout or styling conventions change.
