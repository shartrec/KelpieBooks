# Contributing to KelpieBooks

Thank you for your interest in contributing to KelpieBooks! We are building a fresh, multi-user, open-source accounting engine using modern Rust and WebAssembly.

Because accounting engines demand absolute mathematical precision and immutable audit trailing, we maintain rigorous standards for our codebase. This guide outlines our architecture rules, development workflow, and PR standards.

---

## 1. Code of Architecture Invariants

Before writing any code, you must understand our core architectural invariants. Pull Requests that violate these rules will not be merged:

### Rule A: Zero Floating-Point Numbers for Currency
* Never use `f32` or `f64` for tracking financial values.
* All monetary values must be represented as `i64` whole cents (e.g., `$10.50` is stored and calculated as `1050`).
* All mathematical modifications must happen via safe integer calculations to completely eliminate rounding errors.
* Use the available format_currency() function to format monetary values for display.
* Use the CurrencyInput component in the front end for currency input.

### Rule B: WebAssembly Isolation (`shared_core`)
* The `shared_core` crate is a database-agnostic domain layer shared directly with the frontend Yew app compiled to WASM.
* **Do not** add `sqlx`, native networking, or file-system crate dependencies to `shared_core`.
* Keep database-specific attributes, macros, and connections confined strictly to the `backend` server crate.

### Rule C: Multi-Tenant Scoping
* Every entity, account, transaction, and sub-ledger document must be explicitly scoped to an `organization_id`.
* Ensure database queries filter against the validated organization ID provided by the active user session context.

---

## 2. Our Development Workflow

### Step 1: Find or Open an Issue
Before writing code, search our GitHub issues. If you are fixing a bug or adding a feature that doesn't have an issue yet, please create one describing the change. Look for issues labeled `good first issue` if you are looking for an easy place to start!

### Step 2: Clone & Local Setup
Follow the compilation guides inside our `README.md` to spin up your PostgreSQL instance, seed migrations via the SQLx CLI, and boot the Rocket and Yew watch-servers.

### Step 3: Branch Naming Convention
Create a local branch off `main` using a descriptive slug prefix:
* For bug fixes: `fix/issue-123-broken-filter`
* For features: `feat/accounts-payable-vendor-list`
* For docs: `docs/update-api-spec`

---

## 3. Pull Request Standards

When you submit a Pull Request, please ensure it meets the following checklists:

### License and Copyright Headers
Every new `.rs` and `.scss` file must begin with our standard GPL licensing header. Copy this header block into the top of your files:

```rust
/*
 * Copyright (c) 2026. Trevor Campbell and others.
 *
 * This file is part of KelpieBooks.
 *
 * KelpieBooks is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 */
```

## 4. Sub-Ledger Contribution Opportunities

We are currently building out Module 2: Accounts Payable. If you want to dive into core sub-ledger development, we are actively looking for help with:

* Building Frontend Yew tables to present Vendor Directories.
* Creating input forms for logging incoming Vendor Invoices with automated ledger routing previews.
* Designing out-of-the-box templates for the Chart of Accounts (COA) loader setup.

If you have any questions or want to review code before submitting an official PR, feel free to start a discussion or leave comments inside our active issue tracker!
