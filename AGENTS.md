
## Top-Level Directories

### backend/

The backend contains the Rocket-based server application.
* `backend/src/util/`: Shared backend utilities, such as logging and common types.
* `backend/migrations/`: SQL database migrations.
* `backend/static/`: Static files served by the backend if required.
* `backend/src/`: Rust code.
* 
The Rust code is divided into key functional and utility areas

* `core`: Components common across the application, such as users and security
* `ledger`: General Ledger
* `partners`: Customers and vendor
* `payables`: Vendor invoice
* `sales`: Sales and Sales invoicing 
* `util` : Utility functions

Each functional area is divided into

* `db`/ : Database access layer. Files are grouped by domain concept, such as accounts, users, organisations,
  transactions, journal entries, and security.
* `routes/`: HTTP route handlers grouped by feature area.
* `services/`: Business logic that sits between routes and database access.
* `reports/`: Reports


### frontend/

The frontend is a Yew application compiled to WebAssembly and built with Trunk.

Key areas:

* `frontend/src/main.rs`: Frontend application entry point.
* `frontend/assets/`: Static frontend assets, including images and CSS assets.
* `frontend/index.html`: HTML entry point used by Trunk.
* `frontend/Trunk.toml`: Trunk build configuration.
* `frontend/src`: Rust code

The src is divided into functional area in the same manner as the backend

Each functional area is divided into 
* `pages`: UI pages
* `components`: UI Components

### shared_core/

Shared Rust code used across the backend and frontend.

Typical responsibilities include:

* Shared models
* DTOs
* Request types
* Utility functions
* Common business/domain types

It is organised like the other top level modules into functional areas

### .docs/

The home of all our documentation source

This is divided into

* `general`: Overall documentation about all aspects of the project
* `user`: The end user documentation a.k.a. the user manual

## Architectural Conventions

### Keep Domain Types Shared

If a type is exchanged between frontend and backend, place it in `shared_core` rather than duplicating it.

Good candidates for `shared_core` include:

* Request payloads
* Response DTOs
* Domain models
* Enum values used by both frontend and backend
* Shared formatting or validation helpers

### Handling Currency values

* All monetary values must be represented as rust_decimal::Decimal and stored in the database as NUMERIC(15, 4).
* __Never__ use `f32` or `f64` for tracking financial values.
* All mathematical modifications must happen via safe integer calculations to completely eliminate rounding errors.

## Security Model

Our security model is based on a combination of users, roles, and privileges.

*   **Users**: Individual accounts that can log in to the system.
*   **Roles**: A collection of privileges. Each user is assigned a role, which determines what they are allowed to do.
*   **Privileges**: Specific permissions that grant access to certain actions or data. These are defined as an enum in `shared_core/src/models/auth.rs`.

### Backend

In the backend, we use Rocket's request guards to enforce security. These guards are defined in `backend/src/security.rs`. Each route that requires authentication or specific privileges should use the appropriate guard.

For example, to protect a route that requires the `manage_accounts` privilege, you would use the `RequirePrivilege<ManageAccounts>` guard:

```rust
#[get("/api/accounts")]
async fn get_accounts(
    mut pool: Connection<DbKelpie>,
    guard: RequirePrivilege<ManageAccounts>,
) -> Result<Json<Vec<Account>>, ApiError> {
    // ...
}
```

### Frontend

In the frontend, the `UserContext` provides a `has_privilege` method to check if the current user has a specific privilege. This should be used to conditionally render UI elements that correspond to protected actions.

For example, to only show a "Delete" button to users with the `manage_accounts` privilege:

```rust
{ if user_ctx.has_privilege(&SystemPrivilege::manage_accounts) {
    html! {
        <button class="icon-button btn-action" onclick={on_delete_click}>
            <img src="/images/delete.svg" alt={i18n.t("common-delete")} />
        </button>
    }
} else {
    html! {}
}}
```

This ensures that the UI accurately reflects the user's permissions and prevents them from attempting actions they are not authorized to perform.

## i18n - Internationalization

The back end and front end use similar techniques for internationalization.

Translations and language files are all in the shared_core.

### Frontend

* Use the LocalContext in each component that renders content
* At the start of the component get the locale context

```aiexclude
let i18n = use_locale();
```

* In the html retrieve the text by key

```aiexclude
i18n.t("common-expand")
```

or if it has arguments

```aiexclude
i18n.t_args("vendor-invoice-drawer-inv-number", &fluent_args!["number" => props.invoice.invoice_number.clone()])
```

* Format dates and currencies

```aiexclude
i18n.format_date(primary_entry.date)
i18n.format_currency(total_amount)
```

### Backend

* Use the LocalContext where appropriate, e.g. pdf exports
* At the start of the route create a Context using the data from the AuthenticatedUser

```aiexclude
let i18n = LocaleContext::new(&user.locale);
```

* Use the same techniques as above for translation and formatting.

### Keep Backend Layers Separate

Backend changes should generally follow this flow:

```text
[HTTP Request] -> [Routes] -> [Services] -> [DB Modules] -> [Database]
```

Use each layer for its intended purpose:

* **Routes** should handle HTTP concerns:
    * Request parsing
    * Authentication/authorisation checks
    * Response construction
    * Status codes

* **Services** should handle business logic:
    - Validation
    - Multi-step operations
    - Coordination between database functions
    - Domain rules

* **DB modules** should handle database access:
    - SQL queries
    - Mapping database rows to Rust types
    - Insert/update/delete/select operations

Avoid placing complex business logic directly inside route handlers or database functions.

Use __transactions__ where appropriate. Any route that makes multiple transaction entries, that must remain in balance
in the accounts, __must__ be placed in a transaction.
Most of the database updates are write only so there is generally little need to consider deadlocks in this situation.

__Do not__ use sqlx macros suxh as ```sqlx::query_as!```. They break CI.

### Keep Frontend Components Focused

Frontend code is split into:

* **Pages** for route-level screens
* **Components** for reusable UI pieces
* **Auth logic** for authentication helpers
* **Shared styles** in `kelpie.css`

A page should compose components rather than becoming one large component itself.

Reusable UI patterns, such as modals, rows, layout wrappers, and navigation elements, should live in
`frontend/src/components/`.

## Frontend Layout Guidelines


